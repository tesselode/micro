mod cardinal_direction;
mod circle;
mod clock_direction;
mod irect;
mod lerp;
mod line_segment;
mod polygon;
mod ray;
mod rect;
mod to_index;
mod urect;
mod vec_constants;

pub use cardinal_direction::*;
pub use circle::*;
pub use clock_direction::*;
pub use irect::*;
pub use lerp::*;
pub use line_segment::*;
pub use polygon::*;
pub use ray::*;
pub use rect::*;
pub use to_index::*;
pub use urect::*;
pub use vec_constants::*;

pub use glam::*;

/* use lyon_tessellation::{TessellationError, VertexBuffers};
use palette::LinSrgba;

use crate::{
	color::ColorConstants,
	graphics::{
		Vertex2d,
		mesh::{MeshBuilder, ShapeStyle},
	},
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

fn triangle_points(indices: &[u32], buffers: &VertexBuffers<Vertex2d, u32>) -> Vec<Vec2> {
	indices
		.iter()
		.map(|index| {
			let position = buffers.vertices[*index as usize].position;
			Vec2::new(position.x, position.y)
		})
		.collect()
} */
