use bytemuck::NoUninit;
use wgpu::{
	BufferUsages,
	util::{BufferInitDescriptor, DeviceExt},
};

use crate::Micro;

/// A buffer of arbitrary data that can be used in a
/// [`Shader`](crate::graphics::Shader).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StorageBuffer(pub(crate) wgpu::Buffer);

impl StorageBuffer {
	/// Creates a new [`StorageBuffer`].
	///
	/// The label is visible in graphics debugging programs, like RenderDoc.
	pub fn new<T: NoUninit>(micro: &Micro, label: &str, data: &[T]) -> Self {
		Self(
			micro
				.graphics
				.device
				.create_buffer_init(&BufferInitDescriptor {
					label: Some(label),
					contents: bytemuck::cast_slice(data),
					usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
				}),
		)
	}
}
