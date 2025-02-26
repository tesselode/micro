use glam::Vec2;
use lyon_tessellation::{FillVertex, FillVertexConstructor, StrokeVertex, StrokeVertexConstructor};
use palette::LinSrgba;

use crate::graphics::Vertex2d;

pub(super) struct PointWithoutColorToVertex {
	pub(super) color: LinSrgba,
}

impl FillVertexConstructor<Vertex2d> for PointWithoutColorToVertex {
	fn new_vertex(&mut self, vertex: FillVertex) -> Vertex2d {
		Vertex2d {
			position: Vec2::new(vertex.position().x, vertex.position().y),
			texture_coords: Vec2::ZERO,
			color: self.color,
		}
	}
}

impl StrokeVertexConstructor<Vertex2d> for PointWithoutColorToVertex {
	fn new_vertex(&mut self, vertex: StrokeVertex) -> Vertex2d {
		Vertex2d {
			position: Vec2::new(vertex.position().x, vertex.position().y),
			texture_coords: Vec2::ZERO,
			color: self.color,
		}
	}
}

pub(super) struct PointWithColorToVertex;

impl FillVertexConstructor<Vertex2d> for PointWithColorToVertex {
	fn new_vertex(&mut self, mut vertex: FillVertex) -> Vertex2d {
		let position = Vec2::new(vertex.position().x, vertex.position().y);
		let attributes = vertex.interpolated_attributes();
		Vertex2d {
			position,
			texture_coords: Vec2::ZERO,
			color: LinSrgba::new(attributes[0], attributes[1], attributes[2], attributes[3]),
		}
	}
}

impl StrokeVertexConstructor<Vertex2d> for PointWithColorToVertex {
	fn new_vertex(&mut self, mut vertex: StrokeVertex) -> Vertex2d {
		let position = Vec2::new(vertex.position().x, vertex.position().y);
		let attributes = vertex.interpolated_attributes();
		Vertex2d {
			position,
			texture_coords: Vec2::ZERO,
			color: LinSrgba::new(attributes[0], attributes[1], attributes[2], attributes[3]),
		}
	}
}
