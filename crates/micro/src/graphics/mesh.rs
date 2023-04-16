use std::rc::Rc;

use wgpu::{
	util::{BufferInitDescriptor, DeviceExt},
	Buffer, BufferUsages, Device,
};

use crate::Context;

use super::{texture::Texture, DrawParams, Vertex};

#[derive(Clone)]
pub struct Mesh(pub(crate) Rc<MeshInner>);

impl Mesh {
	pub fn new(ctx: &Context, vertices: &[Vertex], indices: &[u16]) -> Self {
		Self::new_internal(vertices, indices, &ctx.graphics_ctx.device)
	}

	pub fn draw(&self, ctx: &mut Context, params: impl Into<DrawParams>) {
		self.draw_textured(ctx, &ctx.graphics_ctx.default_texture.clone(), params);
	}

	pub fn draw_textured(
		&self,
		ctx: &mut Context,
		texture: &Texture,
		params: impl Into<DrawParams>,
	) {
		ctx.graphics_ctx
			.push_instruction(self.clone(), texture.clone(), params.into());
	}

	pub(crate) fn new_internal(vertices: &[Vertex], indices: &[u16], device: &Device) -> Self {
		let vertex_buffer = device.create_buffer_init(&BufferInitDescriptor {
			label: Some("Vertex Buffer"),
			contents: bytemuck::cast_slice(vertices),
			usage: BufferUsages::VERTEX,
		});
		let index_buffer = device.create_buffer_init(&BufferInitDescriptor {
			label: Some("Index Buffer"),
			contents: bytemuck::cast_slice(indices),
			usage: BufferUsages::INDEX,
		});
		Self(Rc::new(MeshInner {
			vertex_buffer,
			index_buffer,
			num_indices: indices.len() as u32,
		}))
	}
}

pub(crate) struct MeshInner {
	pub(crate) vertex_buffer: Buffer,
	pub(crate) index_buffer: Buffer,
	pub(crate) num_indices: u32,
}
