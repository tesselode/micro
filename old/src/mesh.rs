use std::rc::Rc;

use glam::Vec2;
use glow::{HasContext, NativeBuffer, NativeVertexArray};

use crate::{color::Rgba, context::Context, draw_params::DrawParams, texture::Texture};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vertex {
	pub position: Vec2,
	pub color: Rgba,
	pub texture_coords: Vec2,
}

pub struct Mesh {
	gl: Rc<glow::Context>,
	vertex_array: NativeVertexArray,
	vertex_buffer: NativeBuffer,
	element_buffer: NativeBuffer,
	num_indices: i32,
	texture: Texture,
}

impl Mesh {
	pub fn new(
		ctx: &Context,
		vertices: &[Vertex],
		indices: &[u32],
		texture: Option<&Texture>,
	) -> Result<Self, String> {
		let mut raw_vertex_data = vec![];
		for vertex in vertices {
			raw_vertex_data.push(vertex.position.x);
			raw_vertex_data.push(vertex.position.y);
			raw_vertex_data.push(0.0);
			raw_vertex_data.push(vertex.color.red);
			raw_vertex_data.push(vertex.color.green);
			raw_vertex_data.push(vertex.color.blue);
			raw_vertex_data.push(vertex.color.alpha);
			raw_vertex_data.push(vertex.texture_coords.x);
			raw_vertex_data.push(vertex.texture_coords.y);
		}
		let gl = ctx.graphics().gl();
		let vertex_array;
		let vertex_buffer;
		let element_buffer;
		unsafe {
			vertex_array = gl.create_vertex_array()?;
			vertex_buffer = gl.create_buffer()?;
			element_buffer = gl.create_buffer()?;
			gl.bind_vertex_array(Some(vertex_array));
			gl.bind_buffer(glow::ARRAY_BUFFER, Some(vertex_buffer));
			gl.buffer_data_u8_slice(
				glow::ARRAY_BUFFER,
				bytemuck::cast_slice(&raw_vertex_data),
				glow::STATIC_DRAW,
			);
			gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(element_buffer));
			gl.buffer_data_u8_slice(
				glow::ELEMENT_ARRAY_BUFFER,
				bytemuck::cast_slice(indices),
				glow::STATIC_DRAW,
			);
			gl.vertex_attrib_pointer_f32(
				0,
				3,
				glow::FLOAT,
				false,
				(9 * std::mem::size_of::<f32>()).try_into().unwrap(),
				0,
			);
			gl.enable_vertex_attrib_array(0);
			gl.vertex_attrib_pointer_f32(
				1,
				4,
				glow::FLOAT,
				false,
				(9 * std::mem::size_of::<f32>()).try_into().unwrap(),
				(3 * std::mem::size_of::<f32>()).try_into().unwrap(),
			);
			gl.enable_vertex_attrib_array(1);
			gl.vertex_attrib_pointer_f32(
				2,
				2,
				glow::FLOAT,
				false,
				(9 * std::mem::size_of::<f32>()).try_into().unwrap(),
				(7 * std::mem::size_of::<f32>()).try_into().unwrap(),
			);
			gl.enable_vertex_attrib_array(2);
		}
		Ok(Self {
			gl,
			vertex_array,
			vertex_buffer,
			element_buffer,
			num_indices: indices.len().try_into().unwrap(),
			texture: texture
				.cloned()
				.unwrap_or_else(|| ctx.graphics().default_texture()),
		})
	}

	pub fn rectangle(
		ctx: &Context,
		position: Vec2,
		size: Vec2,
		texture: Option<&Texture>,
	) -> Result<Self, String> {
		Self::new(
			ctx,
			&[
				Vertex {
					position: position + size,
					color: Rgba::WHITE,
					texture_coords: Vec2::new(1.0, 1.0),
				},
				Vertex {
					position: position + Vec2::new(size.x, 0.0),
					color: Rgba::WHITE,
					texture_coords: Vec2::new(1.0, 0.0),
				},
				Vertex {
					position,
					color: Rgba::WHITE,
					texture_coords: Vec2::new(0.0, 0.0),
				},
				Vertex {
					position: position + Vec2::new(0.0, size.y),
					color: Rgba::WHITE,
					texture_coords: Vec2::new(0.0, 1.0),
				},
			],
			&[0, 1, 3, 1, 2, 3],
			texture,
		)
	}

	pub fn draw(&self, ctx: &Context, params: DrawParams) {
		ctx.graphics()
			.shader
			.send_color("BlendColor", params.color)
			.unwrap();
		ctx.graphics()
			.shader
			.send_mat4("LocalTransform", params.transform)
			.unwrap();
		unsafe {
			self.gl
				.bind_texture(glow::TEXTURE_2D, Some(self.texture.native_texture()));
			self.gl.bind_vertex_array(Some(self.vertex_array));
			self.gl
				.draw_elements(glow::TRIANGLES, self.num_indices, glow::UNSIGNED_INT, 0);
			self.gl.bind_vertex_array(None);
		}
	}
}

impl Drop for Mesh {
	fn drop(&mut self) {
		unsafe {
			self.gl.delete_vertex_array(self.vertex_array);
			self.gl.delete_buffer(self.vertex_buffer);
			self.gl.delete_buffer(self.element_buffer);
		}
	}
}