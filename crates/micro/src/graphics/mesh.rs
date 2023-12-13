mod builder;

pub use builder::*;
use glam::Vec2;
use lyon_tessellation::TessellationError;
use palette::LinSrgba;

use std::{fmt::Debug, marker::PhantomData, rc::Rc};

use glow::{HasContext, NativeBuffer, NativeVertexArray};

use crate::{
	context::Context,
	graphics::{draw_params::DrawParams, texture::Texture},
	math::{Circle, Rect},
	IntoOffsetAndCount, OffsetAndCount,
};

use super::{
	color_constants::ColorConstants, configure_vertex_attributes_for_buffer, Vertex, Vertex2d,
	VertexAttributeBuffer, VertexAttributeDivisor,
};

#[derive(Debug)]
pub struct Mesh<V: Vertex = Vertex2d> {
	gl: Rc<glow::Context>,
	vertex_array: NativeVertexArray,
	vertex_buffer: NativeBuffer,
	index_buffer: NativeBuffer,
	num_indices: i32,
	_phantom_data: PhantomData<V>,
}

impl<V: Vertex> Mesh<V> {
	pub fn new(ctx: &Context, vertices: &[V], indices: &[u32]) -> Self {
		Self::new_from_gl(ctx.graphics.gl.clone(), vertices, indices)
	}

	pub(crate) fn new_from_gl(gl: Rc<glow::Context>, vertices: &[V], indices: &[u32]) -> Self {
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
				&gl,
				vertex_buffer,
				V::ATTRIBUTE_KINDS,
				VertexAttributeDivisor::PerVertex,
				0,
			);
		}
		Self {
			gl,
			vertex_array,
			vertex_buffer,
			index_buffer,
			num_indices: indices.len() as i32,
			_phantom_data: PhantomData,
		}
	}

	pub fn set_vertex(&self, index: usize, vertex: V) {
		let gl = &self.gl;
		unsafe {
			gl.bind_vertex_array(Some(self.vertex_array));
			gl.bind_buffer(glow::ARRAY_BUFFER, Some(self.vertex_buffer));
			gl.buffer_sub_data_u8_slice(
				glow::ARRAY_BUFFER,
				(std::mem::size_of::<V>() * index) as i32,
				bytemuck::cast_slice(&[vertex]),
			);
		}
	}

	pub fn draw<'a>(&self, ctx: &Context, params: impl Into<DrawParams<'a>>) {
		self.draw_range(ctx, .., params.into());
	}

	pub fn draw_range<'a>(
		&self,
		ctx: &Context,
		range: impl IntoOffsetAndCount,
		params: impl Into<DrawParams<'a>>,
	) {
		self.draw_inner(
			ctx,
			&ctx.graphics.default_texture,
			range.into_offset_and_count(self.num_indices as usize),
			params.into(),
		);
	}

	pub fn draw_textured<'a>(
		&self,
		ctx: &Context,
		texture: &Texture,
		params: impl Into<DrawParams<'a>>,
	) {
		self.draw_range_textured(ctx, texture, .., params.into());
	}

	pub fn draw_range_textured<'a>(
		&self,
		ctx: &Context,
		texture: &Texture,
		range: impl IntoOffsetAndCount,
		params: impl Into<DrawParams<'a>>,
	) {
		self.draw_inner(
			ctx,
			texture,
			range.into_offset_and_count(self.num_indices as usize),
			params.into(),
		);
	}

	pub fn draw_instanced<'a>(
		&self,
		ctx: &Context,
		num_instances: usize,
		vertex_attribute_buffers: &[&VertexAttributeBuffer],
		params: impl Into<DrawParams<'a>>,
	) {
		self.draw_instanced_inner(
			ctx,
			&ctx.graphics.default_texture,
			num_instances,
			vertex_attribute_buffers,
			params.into(),
		)
	}

	pub fn draw_instanced_textured<'a>(
		&self,
		ctx: &Context,
		texture: &Texture,
		num_instances: usize,
		vertex_attribute_buffers: &[&VertexAttributeBuffer],
		params: impl Into<DrawParams<'a>>,
	) {
		self.draw_instanced_inner(
			ctx,
			texture,
			num_instances,
			vertex_attribute_buffers,
			params.into(),
		)
	}

	fn draw_instanced_inner(
		&self,
		ctx: &Context,
		texture: &Texture,
		num_instances: usize,
		vertex_attribute_buffers: &[&VertexAttributeBuffer],
		params: DrawParams,
	) {
		let gl = &ctx.graphics.gl;
		unsafe {
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
			gl.bind_vertex_array(Some(self.vertex_array));
			params.blend_mode.apply(gl);
			gl.draw_elements_instanced(
				glow::TRIANGLES,
				self.num_indices,
				glow::UNSIGNED_INT,
				0,
				num_instances as i32,
			);
		}
	}

	fn draw_inner(
		&self,
		ctx: &Context,
		texture: &Texture,
		range: OffsetAndCount,
		params: DrawParams,
	) {
		let gl = &ctx.graphics.gl;
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
			gl.draw_elements(
				glow::TRIANGLES,
				range.count as i32,
				glow::UNSIGNED_INT,
				range.offset as i32 * 4,
			);
		}
	}
}

impl Mesh<Vertex2d> {
	pub fn rectangle(ctx: &Context, rect: Rect) -> Self {
		Self::rectangle_with_texture_region(ctx, rect, Rect::from_xywh(0.0, 0.0, 1.0, 1.0))
	}

	pub fn rectangle_with_texture_region(
		ctx: &Context,
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

	pub fn styled_rectangle(
		ctx: &Context,
		style: ShapeStyle,
		rect: Rect,
		color: LinSrgba,
	) -> Result<Self, TessellationError> {
		Ok(MeshBuilder::new()
			.with_rectangle(style, rect, color)?
			.build(ctx))
	}

	pub fn circle(
		ctx: &Context,
		style: ShapeStyle,
		circle: Circle,
		color: LinSrgba,
	) -> Result<Self, TessellationError> {
		Ok(MeshBuilder::new()
			.with_circle(style, circle, color)?
			.build(ctx))
	}

	pub fn ellipse(
		ctx: &Context,
		style: ShapeStyle,
		center: Vec2,
		radii: Vec2,
		rotation: f32,
		color: LinSrgba,
	) -> Result<Self, TessellationError> {
		Ok(MeshBuilder::new()
			.with_ellipse(style, center, radii, rotation, color)?
			.build(ctx))
	}

	pub fn filled_polygon(
		ctx: &Context,
		points: impl IntoIterator<Item = FilledPolygonPoint>,
	) -> Result<Self, TessellationError> {
		Ok(MeshBuilder::new().with_filled_polygon(points)?.build(ctx))
	}

	pub fn polyline(
		ctx: &Context,
		points: impl IntoIterator<Item = StrokePoint>,
		closed: bool,
	) -> Result<Self, TessellationError> {
		Ok(MeshBuilder::new().with_polyline(points, closed)?.build(ctx))
	}

	pub fn simple_polygon(
		ctx: &Context,
		style: ShapeStyle,
		points: impl IntoIterator<Item = Vec2>,
		color: LinSrgba,
	) -> Result<Self, TessellationError> {
		Ok(MeshBuilder::new()
			.with_simple_polygon(style, points, color)?
			.build(ctx))
	}

	pub fn simple_polyline(
		ctx: &Context,
		stroke_width: f32,
		points: impl IntoIterator<Item = Vec2>,
		color: LinSrgba,
	) -> Result<Self, TessellationError> {
		Ok(MeshBuilder::new()
			.with_simple_polyline(stroke_width, points, color)?
			.build(ctx))
	}
}

impl<V: Vertex> Drop for Mesh<V> {
	fn drop(&mut self) {
		unsafe {
			self.gl.delete_vertex_array(self.vertex_array);
			self.gl.delete_buffer(self.vertex_buffer);
			self.gl.delete_buffer(self.index_buffer);
		}
	}
}
