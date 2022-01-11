use std::{error::Error, rc::Rc};

use glam::Vec2;
use glow::{HasContext, NativeBuffer, NativeVertexArray};

use crate::context::Context;

pub struct Mesh {
	gl: Rc<glow::Context>,
	vao: NativeVertexArray,
	vbo: NativeBuffer,
	ebo: NativeBuffer,
	num_indices: i32,
}

impl Mesh {
	pub fn new(ctx: &Context, vertices: &[Vec2], indices: &[u32]) -> Result<Self, Box<dyn Error>> {
		let mut raw_vertices = vec![];
		for vertex in vertices {
			raw_vertices.push(vertex.x);
			raw_vertices.push(vertex.y);
			raw_vertices.push(0.0);
		}
		let gl = ctx.graphics().gl();
		let vao;
		let vbo;
		let ebo;
		unsafe {
			vao = gl.create_vertex_array()?;
			vbo = gl.create_buffer()?;
			ebo = gl.create_buffer()?;
			gl.bind_vertex_array(Some(vao));
			gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
			gl.buffer_data_u8_slice(
				glow::ARRAY_BUFFER,
				bytemuck::cast_slice(&raw_vertices),
				glow::STATIC_DRAW,
			);
			gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(ebo));
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
				(3 * std::mem::size_of::<f32>()).try_into().unwrap(),
				0,
			);
			gl.enable_vertex_attrib_array(0);
		}
		Ok(Self {
			gl,
			vao,
			vbo,
			ebo,
			num_indices: indices.len().try_into().unwrap(),
		})
	}

	pub fn draw(&self) {
		unsafe {
			self.gl.bind_vertex_array(Some(self.vao));
			self.gl
				.draw_elements(glow::TRIANGLES, self.num_indices, glow::UNSIGNED_INT, 0);
			self.gl.bind_vertex_array(None);
		}
	}
}

impl Drop for Mesh {
	fn drop(&mut self) {
		unsafe {
			self.gl.delete_vertex_array(self.vao);
			self.gl.delete_buffer(self.vbo);
			self.gl.delete_buffer(self.ebo);
		}
	}
}
