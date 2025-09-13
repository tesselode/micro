#[cfg(test)]
mod test;

use glam::{Vec2, vec2};

use super::{Circle, IRect, URect};

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serializing", derive(serde::Serialize, serde::Deserialize))]
pub struct Rect {
	pub top_left: Vec2,
	pub size: Vec2,
}

impl Rect {
	pub fn new(top_left: impl Into<Vec2>, size: impl Into<Vec2>) -> Self {
		Self {
			top_left: top_left.into(),
			size: size.into(),
		}
	}

	pub fn from_corners(top_left: impl Into<Vec2>, bottom_right: impl Into<Vec2>) -> Self {
		let top_left = top_left.into();
		let bottom_right = bottom_right.into();
		Self::new(top_left, bottom_right - top_left)
	}

	pub fn centered_around(center: impl Into<Vec2>, size: impl Into<Vec2>) -> Self {
		let center = center.into();
		let size = size.into();
		Self::new(center - size / 2.0, size)
	}

	pub fn centered_around_zero(size: impl Into<Vec2>) -> Self {
		Self::centered_around(Vec2::ZERO, size)
	}

	pub fn as_urect(self) -> URect {
		URect {
			top_left: self.top_left.as_uvec2(),
			size: self.size.as_uvec2(),
		}
	}

	pub fn as_irect(self) -> IRect {
		IRect {
			top_left: self.top_left.as_ivec2(),
			size: self.size.as_ivec2(),
		}
	}

	pub const fn left(self) -> f32 {
		self.top_left.x
	}

	pub fn right(self) -> f32 {
		self.top_left.x + self.size.x
	}

	pub const fn top(self) -> f32 {
		self.top_left.y
	}

	pub fn bottom(self) -> f32 {
		self.top_left.y + self.size.y
	}

	pub fn top_right(self) -> Vec2 {
		Vec2::new(self.right(), self.top())
	}

	pub fn bottom_left(self) -> Vec2 {
		Vec2::new(self.left(), self.bottom())
	}

	pub fn bottom_right(self) -> Vec2 {
		Vec2::new(self.right(), self.bottom())
	}

	pub fn fractional_x(self, fraction: f32) -> f32 {
		self.left() + (self.right() - self.left()) * fraction
	}

	pub fn fractional_y(self, fraction: f32) -> f32 {
		self.top() + (self.bottom() - self.top()) * fraction
	}

	pub fn fractional_point(self, fraction: impl Into<Vec2>) -> Vec2 {
		let fraction = fraction.into();
		Vec2::new(self.fractional_x(fraction.x), self.fractional_y(fraction.y))
	}

	pub fn center_x(self) -> f32 {
		self.fractional_x(0.5)
	}

	pub fn center_y(self) -> f32 {
		self.fractional_y(0.5)
	}

	pub fn center(self) -> Vec2 {
		self.fractional_point(Vec2::splat(0.5))
	}

	pub fn corners(self) -> [Vec2; 4] {
		[
			self.bottom_right(),
			self.top_right(),
			self.top_left,
			self.bottom_left(),
		]
	}

	pub fn translated_x(self, translation: f32) -> Self {
		self.translated(vec2(translation, 0.0))
	}

	pub fn translated_y(self, translation: f32) -> Self {
		self.translated(vec2(0.0, translation))
	}

	pub fn translated(self, translation: impl Into<Vec2>) -> Self {
		Self {
			top_left: self.top_left + translation.into(),
			size: self.size,
		}
	}

	pub fn positioned_x(self, x: f32, anchor: f32) -> Self {
		let left = x - self.size.x * anchor;
		Self {
			top_left: Vec2::new(left, self.top()),
			size: self.size,
		}
	}

	pub fn positioned_y(self, y: f32, anchor: f32) -> Self {
		let top = y - self.size.y * anchor;
		Self {
			top_left: Vec2::new(self.left(), top),
			size: self.size,
		}
	}

	pub fn positioned(self, position: impl Into<Vec2>, anchor: impl Into<Vec2>) -> Self {
		Self {
			top_left: position.into() - self.size * anchor.into(),
			size: self.size,
		}
	}

	pub fn resized_x(self, width: f32, anchor: f32) -> Self {
		let left = self.left() - (width - self.size.x) * anchor;
		Self {
			top_left: Vec2::new(left, self.top()),
			size: Vec2::new(width, self.size.y),
		}
	}

	pub fn resized_y(self, height: f32, anchor: f32) -> Self {
		let top = self.top() - (height - self.size.y) * anchor;
		Self {
			top_left: Vec2::new(self.left(), top),
			size: Vec2::new(self.size.x, height),
		}
	}

	pub fn resized(self, size: impl Into<Vec2>, anchor: impl Into<Vec2>) -> Self {
		let size = size.into();
		let anchor = anchor.into();
		Self {
			top_left: self.top_left - (size - self.size) * anchor,
			size,
		}
	}

	pub fn expanded_x(self, amount: f32, anchor: f32) -> Self {
		self.resized_x(self.size.x + amount, anchor)
	}

	pub fn expanded_y(self, amount: f32, anchor: f32) -> Self {
		self.resized_y(self.size.y + amount, anchor)
	}

	pub fn expanded(self, amount: impl Into<Vec2>, anchor: impl Into<Vec2>) -> Self {
		self.resized(self.size + amount.into(), anchor.into())
	}

	pub fn scaled_x(self, scale: f32, anchor: f32) -> Self {
		self.resized_x(self.size.x * scale, anchor)
	}

	pub fn scaled_y(self, scale: f32, anchor: f32) -> Self {
		self.resized_y(self.size.y * scale, anchor)
	}

	pub fn scaled(self, scale: impl Into<Vec2>, anchor: impl Into<Vec2>) -> Self {
		self.resized(self.size * scale.into(), anchor.into())
	}

	pub fn padded_x(self, padding: f32) -> Self {
		self.padded(vec2(padding, 0.0))
	}

	pub fn padded_y(self, padding: f32) -> Self {
		self.padded(vec2(0.0, padding))
	}

	pub fn padded(self, padding: impl Into<Vec2>) -> Self {
		let padding = padding.into();
		Self {
			top_left: self.top_left - padding,
			size: self.size + padding * 2.0,
		}
	}

	pub fn union(self, other: Self) -> Self {
		let top_left = Vec2::new(
			self.top_left.x.min(other.top_left.x),
			self.top_left.y.min(other.top_left.y),
		);
		let bottom_right = Vec2::new(
			self.bottom_right().x.max(other.bottom_right().x),
			self.bottom_right().y.max(other.bottom_right().y),
		);
		Self::from_corners(top_left, bottom_right)
	}

	pub fn contains_point(self, point: impl Into<Vec2>) -> bool {
		let point = point.into();
		point.x >= self.left()
			&& point.x <= self.right()
			&& point.y >= self.top()
			&& point.y <= self.bottom()
	}

	pub fn overlaps(self, other: Self) -> bool {
		self.left() < other.right()
			&& other.left() < self.right()
			&& self.top() < other.bottom()
			&& other.top() < self.bottom()
	}

	// https://www.jeffreythompson.org/collision-detection/circle-rect.php
	pub fn overlaps_circle(self, circle: Circle) -> bool {
		let test = vec2(
			if circle.center.x < self.left() {
				self.left()
			} else if circle.center.x > self.right() {
				self.right()
			} else {
				circle.center.x
			},
			if circle.center.y < self.top() {
				self.top()
			} else if circle.center.y > self.bottom() {
				self.bottom()
			} else {
				circle.center.y
			},
		);
		(circle.center - test).length_squared() <= circle.radius.powi(2)
	}
}
