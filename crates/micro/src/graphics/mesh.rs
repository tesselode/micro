mod builder;

pub use builder::*;
use glam::{Mat4, Vec2, Vec3};
use lyon_tessellation::TessellationError;
use palette::LinSrgba;

use std::{fmt::Debug, marker::PhantomData, sync::mpsc::Sender};

use glow::{HasContext, NativeBuffer, NativeVertexArray};

use crate::{
	context::Context,
	graphics::{draw_params::DrawParams, texture::Texture},
	math::{Circle, Rect},
	IntoOffsetAndCount, OffsetAndCount,
};

use super::{
	color_constants::ColorConstants, configure_vertex_attributes_for_buffer, shader::Shader,
	unused_resource::UnusedGraphicsResource, BlendMode, Culling, Vertex, Vertex2d,
	VertexAttributeBuffer, VertexAttributeDivisor,
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

	pub fn draw(&self) -> MeshDrawCommand<V> {
		MeshDrawCommand {
			mesh: self,
			params: DrawParams::default(),
			texture: None,
			range: (..).into_offset_and_count(self.num_indices as usize),
		}
	}

	pub fn draw_instanced<'a>(
		&self,
		num_instances: usize,
		vertex_attribute_buffers: &[&VertexAttributeBuffer],
		params: impl Into<DrawParams<'a>>,
	) {
		let params = params.into();
		Context::with(|ctx| {
			self.draw_instanced_inner(
				&ctx.graphics.default_texture,
				num_instances,
				vertex_attribute_buffers,
				params,
			)
		});
	}

	pub fn draw_instanced_textured<'a>(
		&self,
		texture: &Texture,
		num_instances: usize,
		vertex_attribute_buffers: &[&VertexAttributeBuffer],
		params: impl Into<DrawParams<'a>>,
	) {
		self.draw_instanced_inner(
			texture,
			num_instances,
			vertex_attribute_buffers,
			params.into(),
		)
	}

	fn draw_instanced_inner(
		&self,
		texture: &Texture,
		num_instances: usize,
		vertex_attribute_buffers: &[&VertexAttributeBuffer],
		params: DrawParams,
	) {
		Context::with(|ctx| {
			let gl = &ctx.graphics.gl;
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

	fn draw_inner(&self, texture: Option<&Texture>, range: OffsetAndCount, params: DrawParams) {
		Context::with(|ctx| {
			let gl = &ctx.graphics.gl;
			let texture = texture.unwrap_or(&ctx.graphics.default_texture);
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
					range.count as i32,
					glow::UNSIGNED_INT,
					range.offset as i32 * 4,
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

	pub fn styled_rectangle(
		style: ShapeStyle,
		rect: Rect,
		color: LinSrgba,
	) -> Result<Self, TessellationError> {
		Ok(MeshBuilder::new()
			.with_rectangle(style, rect, color)?
			.build())
	}

	pub fn circle(
		style: ShapeStyle,
		circle: Circle,
		color: LinSrgba,
	) -> Result<Self, TessellationError> {
		Ok(MeshBuilder::new()
			.with_circle(style, circle, color)?
			.build())
	}

	pub fn ellipse(
		style: ShapeStyle,
		center: Vec2,
		radii: Vec2,
		rotation: f32,
		color: LinSrgba,
	) -> Result<Self, TessellationError> {
		Ok(MeshBuilder::new()
			.with_ellipse(style, center, radii, rotation, color)?
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
		color: LinSrgba,
	) -> Result<Self, TessellationError> {
		Ok(MeshBuilder::new()
			.with_simple_polygon(style, points, color)?
			.build())
	}

	pub fn simple_polyline(
		stroke_width: f32,
		points: impl IntoIterator<Item = Vec2>,
		color: LinSrgba,
	) -> Result<Self, TessellationError> {
		Ok(MeshBuilder::new()
			.with_simple_polyline(stroke_width, points, color)?
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

pub struct MeshDrawCommand<'a, V: Vertex> {
	mesh: &'a Mesh<V>,
	params: DrawParams<'a>,
	texture: Option<Texture>,
	range: OffsetAndCount,
}

impl<'a, V: Vertex> MeshDrawCommand<'a, V> {
	pub fn texture(self, texture: impl Into<Option<&'a Texture>>) -> Self {
		Self {
			texture: texture.into().cloned(),
			..self
		}
	}

	pub fn range(mut self, range: impl Into<OffsetAndCount>) -> Self {
		self.range = range.into();
		self
	}

	pub fn params(mut self, params: DrawParams<'a>) -> Self {
		self.params = params;
		self
	}

	pub fn shader(mut self, shader: &'a Shader) -> Self {
		self.params = self.params.shader(shader);
		self
	}

	pub fn transformed(mut self, transform: Mat4) -> Self {
		self.params = self.params.transformed(transform);
		self
	}

	pub fn translated_2d(mut self, translation: Vec2) -> Self {
		self.params = self.params.translated_2d(translation);
		self
	}

	pub fn translated_3d(mut self, translation: Vec3) -> Self {
		self.params = self.params.translated_3d(translation);
		self
	}

	pub fn scaled_2d(mut self, scale: Vec2) -> Self {
		self.params = self.params.scaled_2d(scale);
		self
	}

	pub fn scaled_3d(mut self, scale: Vec3) -> Self {
		self.params = self.params.scaled_3d(scale);
		self
	}

	pub fn rotated_x(mut self, rotation: f32) -> Self {
		self.params = self.params.rotated_x(rotation);
		self
	}

	pub fn rotated_y(mut self, rotation: f32) -> Self {
		self.params = self.params.rotated_y(rotation);
		self
	}

	pub fn rotated_z(mut self, rotation: f32) -> Self {
		self.params = self.params.rotated_z(rotation);
		self
	}

	pub fn color(mut self, color: impl Into<LinSrgba>) -> Self {
		self.params = self.params.color(color);
		self
	}

	pub fn blend_mode(mut self, blend_mode: BlendMode) -> Self {
		self.params = self.params.blend_mode(blend_mode);
		self
	}

	pub fn culling(mut self, culling: Culling) -> Self {
		self.params = self.params.culling(culling);
		self
	}
}

impl<'a, V: Vertex> Drop for MeshDrawCommand<'a, V> {
	fn drop(&mut self) {
		self.mesh
			.draw_inner(self.texture.as_ref(), self.range, self.params);
	}
}
