mod circle;
mod irect;
mod rect;
mod urect;
mod vec_constants;

pub use circle::*;
pub use irect::*;
pub use rect::*;
pub use urect::*;
pub use vec_constants::*;

use glam::Vec2;
use lyon_tessellation::{TessellationError, VertexBuffers};
use palette::LinSrgba;

use crate::graphics::{
	mesh::{MeshBuilder, ShapeStyle, Vertex},
	ColorConstants,
};

pub fn triangulate_polygon(points: &[Vec2]) -> Result<Vec<Vec<Vec2>>, TessellationError> {
	let mesh_builder = MeshBuilder::new().with_simple_polygon(
		ShapeStyle::Fill,
		points.iter().copied(),
		LinSrgba::WHITE,
	)?;
	let buffers = &mesh_builder.buffers;
	Ok(buffers
		.indices
		.chunks_exact(3)
		.map(|indices| triangle_points(indices, buffers))
		.collect())
}

fn triangle_points(indices: &[u32], buffers: &VertexBuffers<Vertex, u32>) -> Vec<Vec2> {
	indices
		.iter()
		.map(|index| buffers.vertices[*index as usize].position)
		.collect()
}
