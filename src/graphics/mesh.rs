mod builder;

pub use builder::*;
use glam::Vec2;

use std::rc::Rc;

use bytemuck::{Pod, Zeroable};
use glow::{HasContext, NativeBuffer, NativeVertexArray};

use crate::{
	context::Context,
	graphics::{draw_params::DrawParams, texture::Texture},
	math::Rect,
	IntoOffsetAndCount, OffsetAndCount,
};

use super::color::Rgba;

#[derive(Debug)]
pub struct Mesh {
	gl: Rc<glow::Context>,
	vertex_array: NativeVertexArray,
	vertex_buffer: NativeBuffer,
	index_buffer: NativeBuffer,
	num_indices: i32,
}

impl Mesh {
	pub fn new(ctx: &Context, vertices: &[Vertex], indices: &[u32]) -> Self {
		Self::new_from_gl(ctx.gl.clone(), vertices, indices)
	}

	pub(crate) fn new_from_gl(gl: Rc<glow::Context>, vertices: &[Vertex], indices: &[u32]) -> Self {
		let vertex_array = unsafe { gl.create_vertex_array().unwrap() };
		let vertex_buffer = unsafe { gl.create_buffer().unwrap() };
		let index_buffer = unsafe { gl.create_buffer().unwrap() };
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
			gl.vertex_attrib_pointer_f32(
				0,
				2,
				glow::FLOAT,
				false,
				std::mem::size_of::<Vertex>() as i32,
				0,
			);
			gl.enable_vertex_attrib_array(0);
			gl.vertex_attrib_pointer_f32(
				1,
				2,
				glow::FLOAT,
				false,
				std::mem::size_of::<Vertex>() as i32,
				2 * std::mem::size_of::<f32>() as i32,
			);
			gl.enable_vertex_attrib_array(1);
			gl.vertex_attrib_pointer_f32(
				2,
				4,
				glow::FLOAT,
				false,
				std::mem::size_of::<Vertex>() as i32,
				4 * std::mem::size_of::<f32>() as i32,
			);
			gl.enable_vertex_attrib_array(2);
		}
		Self {
			gl,
			vertex_array,
			vertex_buffer,
			index_buffer,
			num_indices: indices.len().try_into().expect("Mesh has too many indices"),
		}
	}

	pub fn rectangle(ctx: &Context, rect: Rect) -> Self {
		Self::rectangle_with_texture_region(ctx, rect, Rect::xywh(0.0, 0.0, 1.0, 1.0))
	}

	pub fn rectangle_with_texture_region(
		ctx: &Context,
		display_rect: Rect,
		texture_rect: Rect,
	) -> Self {
		let vertices = display_rect
			.corners()
			.iter()
			.copied()
			.zip(texture_rect.corners())
			.map(|(position, texture_coords)| Vertex {
				position,
				texture_coords,
				color: Rgba::WHITE,
			})
			.collect::<Vec<_>>();
		Self::new(ctx, &vertices, &[0, 1, 3, 1, 2, 3])
	}

	pub fn styled_rectangle(ctx: &Context, style: ShapeStyle, rect: Rect) -> Self {
		MeshBuilder::new().with_rectangle(style, rect).build(ctx)
	}

	pub fn circle(ctx: &Context, style: ShapeStyle, center: Vec2, radius: f32) -> Self {
		MeshBuilder::new()
			.with_circle(style, center, radius)
			.build(ctx)
	}

	pub fn ellipse(
		ctx: &Context,
		style: ShapeStyle,
		center: Vec2,
		radii: Vec2,
		rotation: f32,
	) -> Self {
		MeshBuilder::new()
			.with_ellipse(style, center, radii, rotation)
			.build(ctx)
	}

	pub fn polygon(ctx: &Context, style: ShapeStyle, points: &[Vec2]) -> Self {
		MeshBuilder::new().with_polygon(style, points).build(ctx)
	}

	pub fn polyline(ctx: &Context, line_width: f32, points: &[Vec2]) -> Self {
		MeshBuilder::new()
			.with_polyline(line_width, points)
			.build(ctx)
	}

	pub fn set_vertex(&self, index: usize, vertex: Vertex) {
		let gl = &self.gl;
		unsafe {
			gl.bind_vertex_array(Some(self.vertex_array));
			gl.bind_buffer(glow::ARRAY_BUFFER, Some(self.vertex_buffer));
			gl.buffer_sub_data_u8_slice(
				glow::ARRAY_BUFFER,
				(std::mem::size_of::<Vertex>() * index) as i32,
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
			&ctx.default_texture,
			range.into_offset_and_count(
				self.num_indices
					.try_into()
					.expect("cannot convert usize into i32"),
			),
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
			range.into_offset_and_count(
				self.num_indices
					.try_into()
					.expect("cannot convert usize into i32"),
			),
			params.into(),
		);
	}

	fn draw_inner(
		&self,
		ctx: &Context,
		texture: &Texture,
		range: OffsetAndCount,
		params: DrawParams,
	) {
		let gl = &ctx.gl;
		unsafe {
			let shader = params.shader.unwrap_or(&ctx.default_shader);
			shader
				.send_color("blendColor", params.color)
				.expect("Shader does not have a blendColor uniform");
			shader
				.send_mat4("globalTransform", ctx.global_transform())
				.expect("Shader does not have a globalTransform uniform");
			shader
				.send_mat4("localTransform", params.transform())
				.expect("Shader does not have a localTransform uniform");
			shader.bind_sent_textures();
			gl.use_program(Some(shader.program));
			gl.bind_texture(glow::TEXTURE_2D, Some(texture.inner.texture));
			gl.bind_vertex_array(Some(self.vertex_array));
			params.blend_mode.apply(gl);
			gl.draw_elements(
				glow::TRIANGLES,
				range
					.count
					.try_into()
					.expect("cannot convert usize into i32"),
				glow::UNSIGNED_INT,
				i32::try_from(range.offset).expect("cannot convert usize into i32") * 4,
			);
		}
	}
}

impl Drop for Mesh {
	fn drop(&mut self) {
		unsafe {
			self.gl.delete_vertex_array(self.vertex_array);
			self.gl.delete_buffer(self.vertex_buffer);
			self.gl.delete_buffer(self.index_buffer);
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Pod, Zeroable)]
#[repr(C)]
pub struct Vertex {
	pub position: Vec2,
	pub texture_coords: Vec2,
	pub color: Rgba,
}
