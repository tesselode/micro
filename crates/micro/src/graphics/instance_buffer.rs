use bytemuck::Pod;
use wgpu::{
	BufferUsages,
	util::{BufferInitDescriptor, DeviceExt},
};

use crate::Context;

use super::HasVertexAttributes;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct InstanceBuffer(pub(crate) wgpu::Buffer);

impl InstanceBuffer {
	pub fn new<T: HasVertexAttributes + Pod>(ctx: &mut Context, data: &[T]) -> Self {
		Self(
			ctx.graphics
				.device
				.create_buffer_init(&BufferInitDescriptor {
					label: None,
					contents: bytemuck::cast_slice(data),
					usage: BufferUsages::VERTEX,
				}),
		)
	}
}
