use glam::{vec2, Mat4, Vec2};

use super::{Circle, LineSegment};

#[derive(Debug, Clone, PartialEq)]
pub struct Polygon {
	pub points: Vec<Vec2>,
}

impl Polygon {
	pub fn new(points: impl Into<Vec<Vec2>>) -> Self {
		Self {
			points: points.into(),
		}
	}

	pub fn transformed(self, transform: Mat4) -> Self {
		Self {
			points: self
				.points
				.iter()
				.copied()
				.map(|point| transform.transform_point3(point.extend(0.0)).truncate())
				.collect(),
		}
	}

	pub fn translated(self, translation: Vec2) -> Self {
		Self {
			points: self
				.points
				.iter()
				.copied()
				.map(|point| point + translation)
				.collect(),
		}
	}

	pub fn translated_x(self, translation: f32) -> Self {
		self.translated(vec2(translation, 0.0))
	}

	pub fn translated_y(self, translation: f32) -> Self {
		self.translated(vec2(0.0, translation))
	}

	pub fn scaled(self, scale: Vec2) -> Self {
		Self {
			points: self
				.points
				.iter()
				.copied()
				.map(|point| point * scale)
				.collect(),
		}
	}

	pub fn scaled_x(self, scale: f32) -> Self {
		self.scaled(vec2(scale, 0.0))
	}

	pub fn scaled_y(self, scale: f32) -> Self {
		self.scaled(vec2(0.0, scale))
	}

	pub fn rotated(self, rotation: f32) -> Self {
		self.transformed(Mat4::from_rotation_z(rotation))
	}

	pub fn line_segments(&self) -> impl Iterator<Item = LineSegment> + '_ {
		(0..self.points.len()).map(|i| {
			let start = self.points[i];
			let end = self.points[(i + 1) % self.points.len()];
			LineSegment { start, end }
		})
	}

	// https://www.jeffreythompson.org/collision-detection/poly-point.php
	pub fn contains_point(&self, point: Vec2) -> bool {
		self.line_segments()
			.filter(|LineSegment { start: s, end: e }| {
				((s.y > point.y) != (e.y > point.y))
					&& (point.x < (e.x - s.x) * (point.y - s.y) / (e.y - s.y) + s.x)
			})
			.count() % 2 == 1
	}

	pub fn overlaps_circle(&self, circle: Circle) -> bool {
		let edge_intersects_circle = self
			.line_segments()
			.any(|line_segment| line_segment.intersects_circle(circle));
		edge_intersects_circle || self.contains_point(circle.center)
	}
}
