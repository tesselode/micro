use glam::Vec2;

use super::Rect;

/// Represents a circle.
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serializing", derive(serde::Serialize, serde::Deserialize))]
pub struct Circle {
	/// The coordinates of the center of the circle.
	pub center: Vec2,
	/// The distance from the center of the circle to the edge.
	pub radius: f32,
}

impl Circle {
	/// Creates a new circle.
	pub fn new(center: impl Into<Vec2>, radius: f32) -> Self {
		Self {
			center: center.into(),
			radius,
		}
	}

	/// Creates a new circle with the center at (0.0, 0.0).
	pub fn around_zero(radius: f32) -> Self {
		Self::new(Vec2::ZERO, radius)
	}

	/// Moves the center of the circle by the specified amount.
	pub fn translated(self, translation: impl Into<Vec2>) -> Self {
		Self {
			center: self.center + translation.into(),
			..self
		}
	}

	/// Multiplies the radius of the circle by the specified amount.
	pub fn scaled(self, scale: f32) -> Self {
		Self {
			radius: self.radius * scale,
			..self
		}
	}

	/// Returns a rectangle that tightly hugs the edges of the circle.
	pub fn bounding_rect(self) -> Rect {
		Rect::centered_around(self.center, Vec2::splat(self.radius * 2.0))
	}

	/// Returns `true` if the specified point lies within (or on the edge of)
	/// the circle.
	pub fn contains_point(self, point: impl Into<Vec2>) -> bool {
		(point.into() - self.center).length_squared() <= self.radius.powi(2)
	}

	/// Returns `true` if this circle touches the specified other circle.
	pub fn overlaps(self, other: Circle) -> bool {
		(other.center - self.center).length_squared() <= (self.radius + other.radius).powi(2)
	}

	/// Returns `true` if this circle touches the specified rectangle.
	pub fn overlaps_rect(self, rect: Rect) -> bool {
		rect.overlaps_circle(self)
	}
}
