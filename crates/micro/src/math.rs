mod irect;
mod rect;

use glam::Vec2;
pub use irect::*;
use lyon_tessellation::VertexBuffers;
pub use rect::*;

use crate::graphics::mesh::{MeshBuilder, ShapeStyle, Vertex};

pub fn triangulate_polygon(points: &[Vec2]) -> Vec<Vec<Vec2>> {
	let mesh_builder = MeshBuilder::new().with_polygon(ShapeStyle::Fill, points);
	let buffers = mesh_builder.buffers();
	buffers
		.indices
		.chunks_exact(3)
		.map(|indices| triangle_points(indices, buffers))
		.collect()
}

fn triangle_points(indices: &[u32], buffers: &VertexBuffers<Vertex, u32>) -> Vec<Vec2> {
	indices
		.iter()
		.map(|index| buffers.vertices[*index as usize].position)
		.collect()
}