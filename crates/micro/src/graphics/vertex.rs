use bytemuck::{Pod, Zeroable};
use glam::Vec2;
use wgpu::{BufferAddress, VertexAttribute, VertexBufferLayout, VertexStepMode};

use super::color::Rgba;

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Vertex {
	pub position: Vec2,
	pub texture_coords: Vec2,
	pub color: Rgba,
}

impl Vertex {
	const ATTRIBUTES: [VertexAttribute; 3] =
		wgpu::vertex_attr_array![0 => Float32x2, 1 => Float32x2, 2 => Float32x4];

	pub(crate) fn buffer_layout<'a>() -> VertexBufferLayout<'a> {
		use std::mem;

		VertexBufferLayout {
			array_stride: mem::size_of::<Self>() as BufferAddress,
			step_mode: VertexStepMode::Vertex,
			attributes: &Self::ATTRIBUTES,
		}
	}
}
