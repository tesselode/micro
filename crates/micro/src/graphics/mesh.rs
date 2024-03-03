mod builder;

pub use builder::*;
use glam::{Mat4, Vec2};
use lyon_tessellation::TessellationError;
use palette::LinSrgba;

use std::{fmt::Debug, marker::PhantomData, sync::mpsc::Sender};

use glow::{HasContext, NativeBuffer, NativeVertexArray};

use crate::{
	context::Context,
	graphics::texture::Texture,
	math::{Circle, Rect},
	IntoOffsetAndCount, OffsetAndCount,
};

use super::{
	color_constants::ColorConstants, configure_vertex_attributes_for_buffer, shader::Shader,
	standard_draw_command_methods, unused_resource::UnusedGraphicsResource, BlendMode, Culling,
	Vertex, Vertex2d, VertexAttributeBuffer, VertexAttributeDivisor,
};

#[derive(Debug)]
pub struct Mesh<V: Vertex = Vertex2d> {
	vertex_array: NativeVertexArray,
	vertex_buffer: NativeBuffer,
	index_buffer: NativeBuffer,
	num_indices: i32,
	unused_resource_sender: Sender<UnusedGraphicsResource>,
	_phantom_data: PhantomData<V>,
}

impl<V: Vertex> Mesh<V> {
	pub fn new(vertices: &[V], indices: &[u32]) -> Self {
		Context::with(|ctx| {
			Self::new_from_gl(
				&ctx.graphics.gl,
				Context::with(|ctx| ctx.graphics.unused_resource_sender.clone()),
				vertices,
				indices,
			)
		})
	}

	pub(crate) fn new_from_gl(
		gl: &glow::Context,
		unused_resource_sender: Sender<UnusedGraphicsResource>,
		vertices: &[V],
		indices: &[u32],
	) -> Self {
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
		Self {
			vertex_array,
			vertex_buffer,
			index_buffer,
			num_indices: indices.len() as i32,
			unused_resource_sender,
			_phantom_data: PhantomData,
		}
	}

	pub fn set_vertex(&self, index: usize, vertex: V) {
		Context::with(|ctx| {
			let gl = &ctx.graphics.gl;
			unsafe {
				gl.bind_vertex_array(Some(self.vertex_array));
				gl.bind_buffer(glow::ARRAY_BUFFER, Some(self.vertex_buffer));
				gl.buffer_sub_data_u8_slice(
					glow::ARRAY_BUFFER,
					(std::mem::size_of::<V>() * index) as i32,
					bytemuck::cast_slice(&[vertex]),
				);
			}
		});
	}

	pub fn draw(&self) -> DrawMeshCommand<V> {
		DrawMeshCommand {
			mesh: self,
			params: DrawMeshParams {
				texture: None,
				range: (..).into_offset_and_count(self.num_indices as usize),
				shader: None,
				transform: Mat4::IDENTITY,
				color: LinSrgba::WHITE,
				blend_mode: BlendMode::default(),
				culling: Culling::default(),
			},
		}
	}

	fn draw_inner(&self, params: &DrawMeshParams) {
		Context::with(|ctx| {
			let gl = &ctx.graphics.gl;
			let texture = params
				.texture
				.as_ref()
				.unwrap_or(&ctx.graphics.default_texture);
			unsafe {
				let shader = params.shader.unwrap_or(&ctx.graphics.default_shader);
				shader.send_color("blendColor", params.color).ok();
				shader
					.send_mat4("globalTransform", ctx.graphics.global_transform())
					.ok();
				shader.send_mat4("localTransform", params.transform).ok();
				shader
					.send_mat4("normalTransform", params.transform.inverse().transpose())
					.ok();
				shader.bind_sent_textures();
				gl.use_program(Some(shader.program));
				gl.bind_texture(glow::TEXTURE_2D, Some(texture.inner.texture));
				gl.bind_vertex_array(Some(self.vertex_array));
				params.blend_mode.apply(gl);
				params.culling.apply(gl);
				gl.draw_elements(
					glow::TRIANGLES,
					params.range.count as i32,
					glow::UNSIGNED_INT,
					params.range.offset as i32 * 4,
				);
			}
		});
	}

	fn draw_instanced_inner(
		&self,
		num_instances: usize,
		vertex_attribute_buffers: &[&VertexAttributeBuffer],
		params: &DrawMeshInstancedParams,
	) {
		Context::with(|ctx| {
			let gl = &ctx.graphics.gl;
			let texture = params
				.texture
				.as_ref()
				.unwrap_or(&ctx.graphics.default_texture);
			unsafe {
				gl.bind_vertex_array(Some(self.vertex_array));
				let mut next_attribute_index = configure_vertex_attributes_for_buffer(
					gl,
					self.vertex_buffer,
					V::ATTRIBUTE_KINDS,
					VertexAttributeDivisor::PerVertex,
					0,
				);
				for buffer in vertex_attribute_buffers {
					next_attribute_index = configure_vertex_attributes_for_buffer(
						gl,
						buffer.buffer,
						&buffer.attribute_kinds,
						buffer.divisor,
						next_attribute_index,
					);
				}
				let shader = params.shader.unwrap_or(&ctx.graphics.default_shader);
				shader.send_color("blendColor", params.color).ok();
				shader
					.send_mat4("globalTransform", ctx.graphics.global_transform())
					.ok();
				shader.send_mat4("localTransform", params.transform).ok();
				shader
					.send_mat4("normalTransform", params.transform.inverse().transpose())
					.ok();
				shader.bind_sent_textures();
				gl.use_program(Some(shader.program));
				gl.bind_texture(glow::TEXTURE_2D, Some(texture.inner.texture));
				params.blend_mode.apply(gl);
				gl.draw_elements_instanced(
					glow::TRIANGLES,
					self.num_indices,
					glow::UNSIGNED_INT,
					0,
					num_instances as i32,
				);
			}
		});
	}
}

impl Mesh<Vertex2d> {
	pub fn rectangle(rect: Rect) -> Self {
		Self::rectangle_with_texture_region(rect, Rect::from_xywh(0.0, 0.0, 1.0, 1.0))
	}

	pub fn rectangle_with_texture_region(display_rect: Rect, texture_region: Rect) -> Self {
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
		Self::new(&vertices, &[0, 1, 3, 1, 2, 3])
	}

	pub fn outlined_rectangle(stroke_width: f32, rect: Rect) -> Result<Self, TessellationError> {
		Ok(MeshBuilder::new()
			.with_rectangle(ShapeStyle::Stroke(stroke_width), rect, LinSrgba::WHITE)?
			.build())
	}

	pub fn circle(style: ShapeStyle, circle: Circle) -> Result<Self, TessellationError> {
		Ok(MeshBuilder::new()
			.with_circle(style, circle, LinSrgba::WHITE)?
			.build())
	}

	pub fn ellipse(
		style: ShapeStyle,
		center: Vec2,
		radii: Vec2,
		rotation: f32,
	) -> Result<Self, TessellationError> {
		Ok(MeshBuilder::new()
			.with_ellipse(style, center, radii, rotation, LinSrgba::WHITE)?
			.build())
	}

	pub fn filled_polygon(
		points: impl IntoIterator<Item = FilledPolygonPoint>,
	) -> Result<Self, TessellationError> {
		Ok(MeshBuilder::new().with_filled_polygon(points)?.build())
	}

	pub fn polyline(
		points: impl IntoIterator<Item = StrokePoint>,
		closed: bool,
	) -> Result<Self, TessellationError> {
		Ok(MeshBuilder::new().with_polyline(points, closed)?.build())
	}

	pub fn simple_polygon(
		style: ShapeStyle,
		points: impl IntoIterator<Item = Vec2>,
	) -> Result<Self, TessellationError> {
		Ok(MeshBuilder::new()
			.with_simple_polygon(style, points, LinSrgba::WHITE)?
			.build())
	}

	pub fn simple_polyline(
		stroke_width: f32,
		points: impl IntoIterator<Item = Vec2>,
	) -> Result<Self, TessellationError> {
		Ok(MeshBuilder::new()
			.with_simple_polyline(stroke_width, points, LinSrgba::WHITE)?
			.build())
	}
}

impl<V: Vertex> Drop for Mesh<V> {
	fn drop(&mut self) {
		self.unused_resource_sender
			.send(UnusedGraphicsResource::VertexArray(self.vertex_array))
			.ok();
		self.unused_resource_sender
			.send(UnusedGraphicsResource::Buffer(self.vertex_buffer))
			.ok();
		self.unused_resource_sender
			.send(UnusedGraphicsResource::Buffer(self.index_buffer))
			.ok();
	}
}

pub struct DrawMeshParams<'a> {
	pub texture: Option<Texture>,
	pub range: OffsetAndCount,
	pub shader: Option<&'a Shader>,
	pub transform: Mat4,
	pub color: LinSrgba,
	pub blend_mode: BlendMode,
	pub culling: Culling,
}

pub struct DrawMeshCommand<'a, V: Vertex> {
	mesh: &'a Mesh<V>,
	params: DrawMeshParams<'a>,
}

impl<'a, V: Vertex> DrawMeshCommand<'a, V> {
	pub fn texture(mut self, texture: impl Into<Option<&'a Texture>>) -> Self {
		self.params.texture = texture.into().cloned();
		self
	}

	pub fn range(mut self, range: impl IntoOffsetAndCount) -> Self {
		self.params.range = range.into_offset_and_count(self.mesh.num_indices as usize);
		self
	}

	pub fn culling(mut self, culling: Culling) -> Self {
		self.params.culling = culling;
		self
	}

	standard_draw_command_methods!();
}

impl<'a, V: Vertex> Drop for DrawMeshCommand<'a, V> {
	fn drop(&mut self) {
		self.mesh.draw_inner(&self.params)
	}
}

pub struct DrawMeshInstancedParams<'a> {
	pub texture: Option<Texture>,
	pub shader: Option<&'a Shader>,
	pub transform: Mat4,
	pub color: LinSrgba,
	pub blend_mode: BlendMode,
	pub culling: Culling,
}

pub struct DrawMeshInstancedCommand<'a, V: Vertex> {
	mesh: &'a Mesh<V>,
	params: DrawMeshInstancedParams<'a>,
	num_instances: usize,
	vertex_attribute_buffers: &'a [&'a VertexAttributeBuffer],
}

impl<'a, V: Vertex> DrawMeshInstancedCommand<'a, V> {
	pub fn texture(mut self, texture: impl Into<Option<&'a Texture>>) -> Self {
		self.params.texture = texture.into().cloned();
		self
	}

	pub fn culling(mut self, culling: Culling) -> Self {
		self.params.culling = culling;
		self
	}

	standard_draw_command_methods!();
}

impl<'a, V: Vertex> Drop for DrawMeshInstancedCommand<'a, V> {
	fn drop(&mut self) {
		self.mesh.draw_instanced_inner(
			self.num_instances,
			self.vertex_attribute_buffers,
			&self.params,
		)
	}
}
