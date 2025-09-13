use bytemuck::NoUninit;
use wgpu::{
	BufferUsages,
	util::{BufferInitDescriptor, DeviceExt},
};

use crate::Context;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StorageBuffer(pub(crate) wgpu::Buffer);

impl StorageBuffer {
	pub fn new<T: NoUninit>(ctx: &Context, label: &str, data: &[T]) -> Self {
		Self(
			ctx.graphics
				.device
				.create_buffer_init(&BufferInitDescriptor {
					label: Some(label),
					contents: bytemuck::cast_slice(data),
					usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
				}),
		)
	}
}
