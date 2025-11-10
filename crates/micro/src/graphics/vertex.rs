pub use wgpu::{VertexAttribute, vertex_attr_array};

use bytemuck::{Pod, Zeroable};
use glam::Vec2;
use palette::LinSrgba;

/// A trait for types that can be used as [`Mesh`](crate::graphics::mesh::Mesh)
/// vertices.
pub trait Vertex: Copy + Pod + HasVertexAttributes {}

/// A trait for anything that has vertex attributes.
pub trait HasVertexAttributes {
	/// Returns the vertex attributes.
	fn attributes() -> Vec<VertexAttribute>;
}

/// The default vertex type used for 2D meshes.
#[derive(Debug, Clone, Copy, PartialEq, Pod, Zeroable)]
#[repr(C)]
pub struct Vertex2d {
	/// The coordinates of the vertex.
	pub position: Vec2,
	/// The texture coordinates at this vertex.
	pub texture_coords: Vec2,
	/// The blend color of the vertex.
	pub color: LinSrgba,
}

impl Vertex for Vertex2d {}

impl HasVertexAttributes for Vertex2d {
	fn attributes() -> Vec<VertexAttribute> {
		vertex_attr_array![0 => Float32x2, 1 => Float32x2, 2 => Float32x4].into()
	}
}
