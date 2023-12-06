use glam::{Vec2, Vec3};
use lyon_tessellation::{FillVertex, FillVertexConstructor, StrokeVertex, StrokeVertexConstructor};
use palette::LinSrgba;

use super::super::Vertex;

pub(super) struct PointWithoutColorToVertex {
	pub(super) color: LinSrgba,
}

impl FillVertexConstructor<Vertex> for PointWithoutColorToVertex {
	fn new_vertex(&mut self, vertex: FillVertex) -> Vertex {
		Vertex {
			position: Vec3::new(vertex.position().x, vertex.position().y, 0.0),
			normal: Vec3::ZERO,
			texture_coords: Vec2::ZERO,
			color: self.color,
		}
	}
}

impl StrokeVertexConstructor<Vertex> for PointWithoutColorToVertex {
	fn new_vertex(&mut self, vertex: StrokeVertex) -> Vertex {
		Vertex {
			position: Vec3::new(vertex.position().x, vertex.position().y, 0.0),
			normal: Vec3::ZERO,
			texture_coords: Vec2::ZERO,
			color: self.color,
		}
	}
}

pub(super) struct PointWithColorToVertex;

impl FillVertexConstructor<Vertex> for PointWithColorToVertex {
	fn new_vertex(&mut self, mut vertex: FillVertex) -> Vertex {
		let position = Vec3::new(vertex.position().x, vertex.position().y, 0.0);
		let attributes = vertex.interpolated_attributes();
		Vertex {
			position,
			normal: Vec3::ZERO,
			texture_coords: Vec2::ZERO,
			color: LinSrgba::new(attributes[0], attributes[1], attributes[2], attributes[3]),
		}
	}
}

impl StrokeVertexConstructor<Vertex> for PointWithColorToVertex {
	fn new_vertex(&mut self, mut vertex: StrokeVertex) -> Vertex {
		let position = Vec3::new(vertex.position().x, vertex.position().y, 0.0);
		let attributes = vertex.interpolated_attributes();
		Vertex {
			position,
			normal: Vec3::ZERO,
			texture_coords: Vec2::ZERO,
			color: LinSrgba::new(attributes[0], attributes[1], attributes[2], attributes[3]),
		}
	}
}
