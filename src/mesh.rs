use std::rc::Rc;

use bytemuck::{Pod, Zeroable};
use glam::{Vec2, Vec3};
use glow::{HasContext, NativeBuffer, NativeVertexArray};

use crate::{context::Context, draw_params::DrawParams, texture::Texture};

pub struct Mesh {
	raw_mesh: Rc<RawMesh>,
	texture: Option<Texture>,
}

impl Mesh {
	pub fn new(ctx: &Context, vertices: &[Vertex], indices: &[u32]) -> Result<Self, String> {
		Ok(Self {
			raw_mesh: Rc::new(RawMesh::new(ctx.gl.clone(), vertices, indices)?),
			texture: None,
		})
	}

	pub fn textured(
		ctx: &Context,
		vertices: &[Vertex],
		indices: &[u32],
		texture: &Texture,
	) -> Result<Self, String> {
		Ok(Self {
			raw_mesh: Rc::new(RawMesh::new(ctx.gl.clone(), vertices, indices)?),
			texture: Some(texture.clone()),
		})
	}

	pub fn set_texture(&mut self, texture: Option<&Texture>) {
		self.texture = texture.cloned();
	}

	pub fn draw(&self, ctx: &Context, params: impl Into<DrawParams>) {
		fn inner(mesh: &Mesh, ctx: &Context, params: DrawParams) {
			let texture = mesh.texture.as_ref().unwrap_or(&ctx.default_texture);
			let gl = &ctx.gl;
			unsafe {
				let shader = params.shader.as_ref().unwrap_or(&ctx.default_shader);
				shader
					.send_color(ctx, "blendColor", params.color)
					.expect("Shader does not have a blendColor uniform");
				gl.use_program(Some(shader.raw_shader.program));
				gl.bind_texture(glow::TEXTURE_2D, Some(texture.raw_texture.texture));
				gl.bind_vertex_array(Some(mesh.raw_mesh.vertex_array));
				gl.draw_elements(
					glow::TRIANGLES,
					mesh.raw_mesh.num_indices,
					glow::UNSIGNED_INT,
					0,
				);
			}
		}

		inner(self, ctx, params.into())
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
