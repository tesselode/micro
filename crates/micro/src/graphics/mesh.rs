mod builder;

pub use builder::*;
use glam::{Mat4, Vec2};
use lyon_tessellation::TessellationError;
use palette::LinSrgba;

use std::{
	marker::PhantomData,
	rc::Rc,
	sync::{
		atomic::{AtomicU64, Ordering},
		Weak,
	},
};

use glow::{HasContext, NativeBuffer, NativeVertexArray};

use crate::{
	color::ColorConstants,
	context::Context,
	graphics::{shader::Shader, texture::Texture, BlendMode},
	math::{Circle, Rect},
	IntoOffsetAndCount, OffsetAndCount,
};

use super::{
	configure_vertex_attributes_for_buffer,
	resource::{GraphicsResource, GraphicsResourceId},
	standard_draw_param_methods, Culling, Vertex, Vertex2d, VertexAttributeBuffer,
	VertexAttributeDivisor,
};

#[derive(Debug, Clone)]
pub struct Mesh<V: Vertex = Vertex2d> {
	id: MeshId,
	_weak: Weak<()>,
	num_indices: i32,
	_phantom_data: PhantomData<V>,

	// draw params
	pub texture: Option<Texture>,
	pub range: Option<OffsetAndCount>,
	pub shader: Option<Shader>,
	pub transform: Mat4,
	pub color: LinSrgba,
	pub blend_mode: BlendMode,
	pub culling: Culling,
}

impl<V: Vertex> Mesh<V> {
	pub fn new(ctx: &mut Context, vertices: &[V], indices: &[u32]) -> Self {
		let gl = &ctx.graphics.gl;
		let vertex_array = unsafe {
			gl.create_vertex_array()
				.expect("error creating vertex array")
		};
		let vertex_buffer = unsafe { gl.create_buffer().expect("error creating vertex buffer") };
		let index_buffer = unsafe { gl.create_buffer().expect("error creating index buffer") };
		unsafe {
			gl.bind_vertex_array(Some(vertex_array));
			gl.bind_buffer(glow::ARRAY_BUFFER, Some(vertex_buffer));
			gl.buffer_data_u8_slice(
				glow::ARRAY_BUFFER,
				bytemuck::cast_slice(vertices),
				glow::STATIC_DRAW,
			);
			gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(index_buffer));
			gl.buffer_data_u8_slice(
				glow::ELEMENT_ARRAY_BUFFER,
				bytemuck::cast_slice(indices),
				glow::STATIC_DRAW,
			);
			configure_vertex_attributes_for_buffer(
				gl,
				vertex_buffer,
				V::ATTRIBUTE_KINDS,
				VertexAttributeDivisor::PerVertex,
				0,
			);
		}
		let num_indices = indices.len() as i32;
		let (id, weak) = ctx.graphics.meshes.insert(RawMesh {
			gl: gl.clone(),
			vertex_array,
			vertex_buffer,
			index_buffer,
		});
		Self {
			id,
			_weak: weak,
			num_indices,
			_phantom_data: PhantomData,
			texture: None,
			range: None,
			shader: None,
			transform: Mat4::IDENTITY,
			color: LinSrgba::WHITE,
			blend_mode: BlendMode::default(),
			culling: Culling::default(),
		}
	}

	pub fn texture<'a>(&self, texture: impl Into<Option<&'a Texture>>) -> Self {
		let mut new = self.clone();
		new.texture = texture.into().cloned();
		new
	}

	standard_draw_param_methods!();

	pub fn culling(&self, culling: Culling) -> Self {
		let mut new = self.clone();
		new.culling = culling;
		new
	}

	pub fn range(&self, range: impl IntoOffsetAndCount) -> Self {
		let mut new = self.clone();
		new.range = range.into_offset_and_count(self.num_indices as usize);
		new
	}

	pub fn set_vertex(&self, ctx: &Context, index: usize, vertex: V) {
		let gl = &ctx.graphics.gl;
		let mesh = ctx.graphics.meshes.get(self.id);
		unsafe {
			gl.bind_vertex_array(Some(mesh.vertex_array));
			gl.bind_buffer(glow::ARRAY_BUFFER, Some(mesh.vertex_buffer));
			gl.buffer_sub_data_u8_slice(
				glow::ARRAY_BUFFER,
				(std::mem::size_of::<V>() * index) as i32,
				bytemuck::cast_slice(&[vertex]),
			);
		}
	}

	pub fn draw(&self, ctx: &mut Context) {
		let global_transform = ctx.graphics.global_transform();
		let texture = self
			.texture
			.clone()
			.unwrap_or(ctx.graphics.default_texture.clone());
		let shader = self
			.shader
			.clone()
			.unwrap_or(ctx.graphics.default_shader.clone());
		shader.send_color(ctx, "blendColor", self.color).ok();
		shader
			.send_mat4(ctx, "globalTransform", global_transform)
			.ok();
		shader.send_mat4(ctx, "localTransform", self.transform).ok();
		shader
			.send_mat4(ctx, "normalTransform", self.transform.inverse().transpose())
			.ok();
		shader.bind_sent_textures(ctx);
		let gl = &ctx.graphics.gl;
		let mesh = ctx.graphics.meshes.get(self.id);
		let raw_texture = ctx.graphics.textures.get(texture.id);
		let raw_shader = ctx.graphics.shaders.get(shader.id);
		let range = self.range.unwrap_or(OffsetAndCount {
			offset: 0,
			count: self.num_indices as usize,
		});
		unsafe {
			gl.use_program(Some(raw_shader.program));
			gl.bind_texture(glow::TEXTURE_2D, Some(raw_texture.texture));
			gl.bind_vertex_array(Some(mesh.vertex_array));
			self.blend_mode.apply(gl);
			self.culling.apply(gl);
			gl.draw_elements(
				glow::TRIANGLES,
				range.count as i32,
				glow::UNSIGNED_INT,
				range.offset as i32 * 4,
			);
		}
	}

	pub fn draw_instanced(
		&self,
		ctx: &mut Context,
		num_instances: usize,
		vertex_attribute_buffers: &[&VertexAttributeBuffer],
	) {
		let global_transform = ctx.graphics.global_transform();
		let texture = self
			.texture
			.clone()
			.unwrap_or(ctx.graphics.default_texture.clone());
		let shader = self
			.shader
			.clone()
			.unwrap_or(ctx.graphics.default_shader.clone());
		shader.send_color(ctx, "blendColor", self.color).ok();
		shader
			.send_mat4(ctx, "globalTransform", global_transform)
			.ok();
		shader.send_mat4(ctx, "localTransform", self.transform).ok();
		shader
			.send_mat4(ctx, "normalTransform", self.transform.inverse().transpose())
			.ok();
		shader.bind_sent_textures(ctx);
		let gl = &ctx.graphics.gl;
		let mesh = ctx.graphics.meshes.get(self.id);
		let raw_texture = ctx.graphics.textures.get(texture.id);
		let raw_shader = ctx.graphics.shaders.get(shader.id);
		unsafe {
			gl.bind_vertex_array(Some(mesh.vertex_array));
			let mut next_attribute_index = configure_vertex_attributes_for_buffer(
				gl,
				mesh.vertex_buffer,
				V::ATTRIBUTE_KINDS,
				VertexAttributeDivisor::PerVertex,
				0,
			);
			for buffer in vertex_attribute_buffers {
				let raw_buffer = ctx.graphics.vertex_attribute_buffers.get(buffer.id);
				next_attribute_index = configure_vertex_attributes_for_buffer(
					gl,
					raw_buffer.buffer,
					&raw_buffer.attribute_kinds,
					raw_buffer.divisor,
					next_attribute_index,
				);
			}
			gl.use_program(Some(raw_shader.program));
			gl.bind_texture(glow::TEXTURE_2D, Some(raw_texture.texture));
			self.blend_mode.apply(gl);
			gl.draw_elements_instanced(
				glow::TRIANGLES,
				self.num_indices,
				glow::UNSIGNED_INT,
				0,
				num_instances as i32,
			);
		}
	}
}

impl Mesh<Vertex2d> {
	pub fn rectangle(ctx: &mut Context, rect: Rect) -> Self {
		Self::rectangle_with_texture_region(ctx, rect, Rect::new((0.0, 0.0), (1.0, 1.0)))
	}

	pub fn rectangle_with_texture_region(
		ctx: &mut Context,
		display_rect: Rect,
		texture_region: Rect,
	) -> Self {
		let vertices = display_rect
			.corners()
			.iter()
			.copied()
			.zip(texture_region.corners())
			.map(|(position, texture_coords)| Vertex2d {
				position,
				texture_coords,
				color: LinSrgba::WHITE,
			})
			.collect::<Vec<_>>();
		Self::new(ctx, &vertices, &[0, 1, 3, 1, 2, 3])
	}

	pub fn outlined_rectangle(
		ctx: &mut Context,
		stroke_width: f32,
		rect: Rect,
	) -> Result<Self, TessellationError> {
		Ok(MeshBuilder::new()
			.with_rectangle(ShapeStyle::Stroke(stroke_width), rect, LinSrgba::WHITE)?
			.build(ctx))
	}

	pub fn circle(
		ctx: &mut Context,
		style: ShapeStyle,
		circle: Circle,
	) -> Result<Self, TessellationError> {
		Ok(MeshBuilder::new()
			.with_circle(style, circle, LinSrgba::WHITE)?
			.build(ctx))
	}

	pub fn ellipse(
		ctx: &mut Context,
		style: ShapeStyle,
		center: impl Into<Vec2>,
		radii: impl Into<Vec2>,
		rotation: f32,
	) -> Result<Self, TessellationError> {
		Ok(MeshBuilder::new()
			.with_ellipse(style, center, radii, rotation, LinSrgba::WHITE)?
			.build(ctx))
	}

	pub fn filled_polygon(
		ctx: &mut Context,
		points: impl IntoIterator<Item = FilledPolygonPoint>,
	) -> Result<Self, TessellationError> {
		Ok(MeshBuilder::new().with_filled_polygon(points)?.build(ctx))
	}

	pub fn polyline(
		ctx: &mut Context,
		points: impl IntoIterator<Item = StrokePoint>,
		closed: bool,
	) -> Result<Self, TessellationError> {
		Ok(MeshBuilder::new().with_polyline(points, closed)?.build(ctx))
	}

	pub fn simple_polygon(
		ctx: &mut Context,
		style: ShapeStyle,
		points: impl IntoIterator<Item = Vec2>,
	) -> Result<Self, TessellationError> {
		Ok(MeshBuilder::new()
			.with_simple_polygon(style, points, LinSrgba::WHITE)?
			.build(ctx))
	}

	pub fn simple_polyline(
		ctx: &mut Context,
		stroke_width: f32,
		points: impl IntoIterator<Item = Vec2>,
	) -> Result<Self, TessellationError> {
		Ok(MeshBuilder::new()
			.with_simple_polyline(stroke_width, points, LinSrgba::WHITE)?
			.build(ctx))
	}
}

#[derive(Debug)]
pub(crate) struct RawMesh {
	gl: Rc<glow::Context>,
	vertex_array: NativeVertexArray,
	vertex_buffer: NativeBuffer,
	index_buffer: NativeBuffer,
}

impl GraphicsResource for RawMesh {
	type Id = MeshId;
}

impl Drop for RawMesh {
	fn drop(&mut self) {
		unsafe {
			self.gl.delete_vertex_array(self.vertex_array);
			self.gl.delete_buffer(self.vertex_buffer);
			self.gl.delete_buffer(self.index_buffer);
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct MeshId(pub u64);

static NEXT_MESH_ID: AtomicU64 = AtomicU64::new(0);

impl GraphicsResourceId for MeshId {
	fn next() -> Self {
		MeshId(NEXT_MESH_ID.fetch_add(1, Ordering::SeqCst))
	}
}
