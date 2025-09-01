pub use wgpu::{VertexAttribute, vertex_attr_array};

use bytemuck::{Pod, Zeroable};
use glam::Vec2;
use palette::LinSrgba;

pub trait Vertex: Copy + Pod + HasVertexAttributes {}

pub trait HasVertexAttributes {
	fn attributes() -> Vec<VertexAttribute>;
}

#[derive(Debug, Clone, Copy, PartialEq, Pod, Zeroable)]
#[repr(C)]
pub struct Vertex2d {
	pub position: Vec2,
	pub texture_coords: Vec2,
	pub color: LinSrgba,
}

impl Vertex for Vertex2d {}

impl HasVertexAttributes for Vertex2d {
	fn attributes() -> Vec<VertexAttribute> {
		vertex_attr_array![0 => Float32x2, 1 => Float32x2, 2 => Float32x4].into()
	}
}
