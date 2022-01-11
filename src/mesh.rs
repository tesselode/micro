use std::error::Error;

use glam::Vec2;
use glow::{HasContext, NativeBuffer, NativeVertexArray};

use crate::context::Context;

pub struct Mesh {
	vao: NativeVertexArray,
	vbo: NativeBuffer,
}

impl Mesh {
	pub fn new(ctx: &Context, vertices: &[Vec2]) -> Result<Self, Box<dyn Error>> {
		let mut raw_vertices = vec![];
		for vertex in vertices {
			raw_vertices.push(vertex.x);
			raw_vertices.push(vertex.y);
			raw_vertices.push(0.0);
		}
		let gl = &ctx.graphics().gl;
		let vao;
		let vbo;
		unsafe {
			vao = gl.create_vertex_array()?;
			vbo = gl.create_buffer()?;
			gl.bind_vertex_array(Some(vao));
			gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
			gl.buffer_data_u8_slice(
				glow::ARRAY_BUFFER,
				bytemuck::cast_slice(&raw_vertices),
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
		Ok(Self { vao, vbo })
	}

	pub fn draw(&self, ctx: &Context) {
		let gl = &ctx.graphics().gl;
		unsafe {
			gl.bind_vertex_array(Some(self.vao));
			gl.draw_arrays(glow::TRIANGLES, 0, 3);
		}
	}
}
