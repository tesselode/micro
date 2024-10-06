use glam::{vec2, Mat4, Vec2};

use super::{Circle, Lerp};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LineSegment {
	pub start: Vec2,
	pub end: Vec2,
}

impl LineSegment {
	pub fn midpoint(self) -> Vec2 {
		(self.start + self.end) / 2.0
	}

	pub fn transformed(self, transform: Mat4) -> Self {
		Self {
			start: transform
				.transform_point3(self.start.extend(0.0))
				.truncate(),
			end: transform.transform_point3(self.end.extend(0.0)).truncate(),
		}
	}

	pub fn translated(self, translation: Vec2) -> Self {
		Self {
			start: self.start + translation,
			end: self.end + translation,
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
			start: self.start * scale,
			end: self.end * scale,
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

	pub fn length(self) -> f32 {
		(self.end - self.start).length()
	}

	pub fn length_squared(self) -> f32 {
		(self.end - self.start).length_squared()
	}

	pub fn contains_point(self, point: Vec2, tolerance: f32) -> bool {
		let distance_from_start = (point - self.start).length();
		let distance_from_end = (point - self.end).length();
		let distance_difference = ((distance_from_start + distance_from_end) - self.length()).abs();
		distance_difference <= tolerance
	}

	// https://www.jeffreythompson.org/collision-detection/line-circle.php
	pub fn intersects_circle(self, circle: Circle) -> bool {
		const LINE_ON_POINT_TOLERANCE: f32 = 0.01;
		if circle.contains_point(self.start) {
			return true;
		}
		if circle.contains_point(self.end) {
			return true;
		}
		let dot = (circle.center - self.start).dot(self.end - self.start) / self.length_squared();
		let closest = self.start + dot * (self.end - self.start);
		if !self.contains_point(closest, LINE_ON_POINT_TOLERANCE) {
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
