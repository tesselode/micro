use glam::Vec2;

use super::Rect;

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serializing", derive(serde::Serialize, serde::Deserialize))]
pub struct Circle {
	pub center: Vec2,
	pub radius: f32,
}

impl Circle {
	pub fn new(center: Vec2, radius: f32) -> Self {
		Self { center, radius }
	}

	pub fn around_zero(radius: f32) -> Self {
		Self::new(Vec2::ZERO, radius)
	}

	pub fn translated(self, translation: Vec2) -> Self {
		Self {
			center: self.center + translation,
			..self
		}
	}

	pub fn scaled(self, scale: f32) -> Self {
		Self {
			radius: self.radius * scale,
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
