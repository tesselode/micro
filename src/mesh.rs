use std::sync::Arc;

use bytemuck::{Pod, Zeroable};
use glam::Vec3;
use glow::{HasContext, NativeBuffer, NativeVertexArray};

use crate::context::Context;

#[derive(Debug, Clone, Copy, PartialEq, Pod, Zeroable)]
#[repr(C)]
pub struct Vertex {
	pub position: Vec3,
}

pub struct RawMesh {
	gl: Arc<glow::Context>,
	vertex_array: NativeVertexArray,
	vertex_buffer: NativeBuffer,
	num_vertices: i32,
}

impl RawMesh {
	pub fn new(gl: Arc<glow::Context>, vertices: &[Vertex]) -> Result<Self, String> {
		let vertex_array = unsafe { gl.create_vertex_array()? };
		let vertex_buffer = unsafe { gl.create_buffer()? };
		unsafe {
			gl.bind_vertex_array(Some(vertex_array));
			gl.bind_buffer(glow::ARRAY_BUFFER, Some(vertex_buffer));
			gl.buffer_data_u8_slice(
				glow::ARRAY_BUFFER,
				bytemuck::cast_slice(vertices),
				glow::STATIC_DRAW,
			);
			gl.vertex_attrib_pointer_f32(
				0,
				3,
				glow::FLOAT,
				false,
				(3 * std::mem::size_of::<f32>()) as i32,
				0,
			);
			gl.enable_vertex_attrib_array(0);
		}
		Ok(Self {
			gl,
			vertex_array,
			vertex_buffer,
			num_vertices: vertices
				.len()
				.try_into()
				.expect("Mesh has too many vertices"),
		})
	}
}

impl Drop for RawMesh {
	fn drop(&mut self) {
		unsafe {
			self.gl.delete_vertex_array(self.vertex_array);
			self.gl.delete_buffer(self.vertex_buffer);
		}
	}
}

pub struct Mesh {
	raw_mesh: Arc<RawMesh>,
}

impl Mesh {
	pub fn new(ctx: &Context, vertices: &[Vertex]) -> Result<Self, String> {
		Ok(Self {
			raw_mesh: Arc::new(RawMesh::new(ctx.gl.clone(), vertices)?),
		})
	}

	pub fn draw(&self, ctx: &Context) {
		let gl = &ctx.gl;
		unsafe {
			gl.bind_vertex_array(Some(self.raw_mesh.vertex_array));
			gl.draw_arrays(glow::TRIANGLES, 0, self.raw_mesh.num_vertices);
		}
	}
}
