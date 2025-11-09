use glam::{Mat4, Vec2, vec2};

use super::{Circle, Lerp};

/// A 2D line segment with a start and end point.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LineSegment {
	/// The coordinates of the start of the line segment.
	pub start: Vec2,
	/// The coordinates of the end of the line segment.
	pub end: Vec2,
}

impl LineSegment {
	/// The point halfway from the start to the end point.
	pub fn midpoint(self) -> Vec2 {
		(self.start + self.end) / 2.0
	}

	/// Returns a new line segment with the transform applied to the
	/// start and end points.
	pub fn transformed(self, transform: Mat4) -> Self {
		Self {
			start: transform
				.transform_point3(self.start.extend(0.0))
				.truncate(),
			end: transform.transform_point3(self.end.extend(0.0)).truncate(),
		}
	}

	/// Returns a new line segment with the start and end points translated
	/// by the specified vector.
	pub fn translated(self, translation: impl Into<Vec2>) -> Self {
		let translation = translation.into();
		Self {
			start: self.start + translation,
			end: self.end + translation,
		}
	}

	/// Returns a new line segment with the start and end points translated
	/// along the X axis by the specified amount.
	pub fn translated_x(self, translation: f32) -> Self {
		self.translated(vec2(translation, 0.0))
	}

	/// Returns a new line segment with the start and end points translated
	/// along the Y axis by the specified amount.
	pub fn translated_y(self, translation: f32) -> Self {
		self.translated(vec2(0.0, translation))
	}

	/// Returns a new line segment with the start and end points scaled
	/// by the specified amounts along the X and Y axes.
	pub fn scaled(self, scale: impl Into<Vec2>) -> Self {
		let scale = scale.into();
		Self {
			start: self.start * scale,
			end: self.end * scale,
		}
	}

	/// Returns a new line segment with the start and end points scaled
	/// by the specified amount along the X axis.
	pub fn scaled_x(self, scale: f32) -> Self {
		self.scaled(vec2(scale, 0.0))
	}

	/// Returns a new line segment with the start and end points scaled
	/// by the specified amount along the Y axis.
	pub fn scaled_y(self, scale: f32) -> Self {
		self.scaled(vec2(0.0, scale))
	}

	/// Returns a new line segment with the start and end points rotated
	/// by the specified amount (in radians).
	pub fn rotated(self, rotation: f32) -> Self {
		self.transformed(Mat4::from_rotation_z(rotation))
	}

	/// Returns the distance between the start and end points.
	pub fn length(self) -> f32 {
		(self.end - self.start).length()
	}

	/// Returns the squared distance between the start and end points.
	pub fn length_squared(self) -> f32 {
		(self.end - self.start).length_squared()
	}

	/// Returns `true` if the specified point roughly lies on the line segment.
	/// The `tolerance` is the maximum distance the point can be away from
	/// the line segment and still be considered "on" the segment.
	pub fn contains_point(self, point: Vec2, tolerance: f32) -> bool {
		let distance_from_start = (point - self.start).length();
		let distance_from_end = (point - self.end).length();
		let distance_difference = ((distance_from_start + distance_from_end) - self.length()).abs();
		distance_difference <= tolerance
	}

	/// Returns `true` if the line segment touches the specified circle.
	/// The `tolerance` is the maximum distance the segment can be away from
	/// the circle and still be considered "on" the circle.
	// https://www.jeffreythompson.org/collision-detection/line-circle.php
	pub fn intersects_circle(self, circle: Circle, tolerance: f32) -> bool {
		if circle.contains_point(self.start) {
			return true;
		}
		if circle.contains_point(self.end) {
			return true;
		}
		let dot = (circle.center - self.start).dot(self.end - self.start) / self.length_squared();
		let closest = self.start + dot * (self.end - self.start);
		if !self.contains_point(closest, tolerance) {
			return false;
		}
		let distance_squared = (closest - circle.center).length_squared();
		distance_squared <= circle.radius.powi(2)
	}
}

impl From<LineSegment> for [Vec2; 2] {
	fn from(value: LineSegment) -> Self {
		[value.start, value.end]
	}
}

impl From<[Vec2; 2]> for LineSegment {
	fn from(value: [Vec2; 2]) -> Self {
		Self {
			start: value[0],
			end: value[1],
		}
	}
}

impl From<LineSegment> for (Vec2, Vec2) {
	fn from(value: LineSegment) -> Self {
		(value.start, value.end)
	}
}

impl From<(Vec2, Vec2)> for LineSegment {
	fn from(value: (Vec2, Vec2)) -> Self {
		Self {
			start: value.0,
			end: value.1,
		}
	}
}

impl Lerp for LineSegment {
	fn lerp(self, other: Self, f: f32) -> Self {
		Self {
			start: self.start.lerp(other.start, f),
			end: self.end.lerp(other.end, f),
		}
	}
}
