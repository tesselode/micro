use glam::Vec2;

use super::Rect;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Circle {
	pub center: Vec2,
	pub radius: f32,
}

impl Circle {
	pub fn new(center: Vec2, radius: f32) -> Self {
		Self { center, radius }
	}

	pub fn translated(self, translation: Vec2) -> Self {
		Self {
			center: self.center + translation,
			..self
		}
	}

	pub fn bounding_rect(&self) -> Rect {
		Rect::centered_around(self.center, Vec2::splat(self.radius * 2.0))
	}

	pub fn contains_point(&self, point: Vec2) -> bool {
		(point - self.center).length_squared() <= self.radius.powi(2)
	}

	pub fn overlaps(&self, other: Circle) -> bool {
		(other.center - self.center).length_squared() <= (self.radius + other.radius).powi(2)
	}
}
