use std::rc::Rc;

use bytemuck::{Pod, Zeroable};
use glam::{Vec2, Vec3};
use glow::{HasContext, NativeBuffer, NativeVertexArray};

use crate::{context::Context, texture::Texture};

pub struct Mesh {
	raw_mesh: Rc<RawMesh>,
}

impl Mesh {
	pub fn new(ctx: &Context, vertices: &[Vertex], indices: &[u32]) -> Result<Self, String> {
		Ok(Self {
			raw_mesh: Rc::new(RawMesh::new(ctx.gl.clone(), vertices, indices)?),
		})
	}

	pub fn draw(&self, ctx: &Context, texture: &Texture) {
		let gl = &ctx.gl;
		unsafe {
			gl.bind_texture(glow::TEXTURE_2D, Some(texture.raw_texture.texture));
			gl.bind_vertex_array(Some(self.raw_mesh.vertex_array));
			gl.draw_elements(
				glow::TRIANGLES,
				self.raw_mesh.num_indices,
				glow::UNSIGNED_INT,
				0,
			);
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Pod, Zeroable)]
#[repr(C)]
pub struct Vertex {
	pub position: Vec3,
	pub texture_coords: Vec2,
}

pub struct RawMesh {
	gl: Rc<glow::Context>,
	vertex_array: NativeVertexArray,
	vertex_buffer: NativeBuffer,
	index_buffer: NativeBuffer,
	num_indices: i32,
}

impl RawMesh {
	pub fn new(
		gl: Rc<glow::Context>,
		vertices: &[Vertex],
		indices: &[u32],
	) -> Result<Self, String> {
		let vertex_array = unsafe { gl.create_vertex_array()? };
		let vertex_buffer = unsafe { gl.create_buffer()? };
		let index_buffer = unsafe { gl.create_buffer()? };
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
				3,
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
				3 * std::mem::size_of::<f32>() as i32,
			);
			gl.enable_vertex_attrib_array(1);
		}
		Ok(Self {
			gl,
			vertex_array,
			vertex_buffer,
			index_buffer,
			num_indices: indices.len().try_into().expect("Mesh has too many indices"),
		})
	}
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
