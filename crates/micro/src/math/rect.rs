#[cfg(test)]
mod test;

use glam::Vec2;

use super::{IRect, URect};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rect {
	pub top_left: Vec2,
	pub size: Vec2,
}

impl Rect {
	pub const fn new(top_left: Vec2, size: Vec2) -> Self {
		Self { top_left, size }
	}

	pub fn from_top_left_and_bottom_right(top_left: Vec2, bottom_right: Vec2) -> Self {
		Self::new(top_left, bottom_right - top_left)
	}

	pub const fn from_xywh(x: f32, y: f32, width: f32, height: f32) -> Self {
		Self::new(Vec2::new(x, y), Vec2::new(width, height))
	}

	pub fn centered_around(center: Vec2, size: Vec2) -> Self {
		Self::new(center - size / 2.0, size)
	}

	pub fn centered_around_zero(size: Vec2) -> Self {
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

	pub const fn left(&self) -> f32 {
		self.top_left.x
	}

	pub fn right(&self) -> f32 {
		self.top_left.x + self.size.x
	}

	pub const fn top(&self) -> f32 {
		self.top_left.y
	}

	pub fn bottom(&self) -> f32 {
		self.top_left.y + self.size.y
	}

	pub fn top_right(&self) -> Vec2 {
		Vec2::new(self.right(), self.top())
	}

	pub fn bottom_left(&self) -> Vec2 {
		Vec2::new(self.left(), self.bottom())
	}

	pub fn bottom_right(&self) -> Vec2 {
		Vec2::new(self.right(), self.bottom())
	}

	pub fn fractional_x(&self, fraction: f32) -> f32 {
		self.left() + (self.right() - self.left()) * fraction
	}

	pub fn fractional_y(&self, fraction: f32) -> f32 {
		self.top() + (self.bottom() - self.top()) * fraction
	}

	pub fn fractional_point(&self, fraction: Vec2) -> Vec2 {
		Vec2::new(self.fractional_x(fraction.x), self.fractional_y(fraction.y))
	}

	pub fn center_x(&self) -> f32 {
		self.fractional_x(0.5)
	}

	pub fn center_y(&self) -> f32 {
		self.fractional_y(0.5)
	}

	pub fn center(&self) -> Vec2 {
		self.fractional_point(Vec2::splat(0.5))
	}

	pub fn corners(&self) -> [Vec2; 4] {
		[
			self.bottom_right(),
			self.top_right(),
			self.top_left,
			self.bottom_left(),
		]
	}

	pub fn translated(&self, translation: Vec2) -> Self {
		Self {
			top_left: self.top_left + translation,
			size: self.size,
		}
	}

	pub fn positioned_x(&self, x: f32, anchor: f32) -> Self {
		let left = x - self.size.x * anchor;
		Self {
			top_left: Vec2::new(left, self.top()),
			size: self.size,
		}
	}

	pub fn positioned_y(&self, y: f32, anchor: f32) -> Self {
		let top = y - self.size.y * anchor;
		Self {
			top_left: Vec2::new(self.left(), top),
			size: self.size,
		}
	}

	pub fn positioned(&self, position: Vec2, anchor: Vec2) -> Self {
		Self {
			top_left: position - self.size * anchor,
			size: self.size,
		}
	}

	pub fn resized_x(&self, width: f32, anchor: f32) -> Self {
		let left = self.left() - (width - self.size.x) * anchor;
		Self {
			top_left: Vec2::new(left, self.top()),
			size: Vec2::new(width, self.size.y),
		}
	}

	pub fn resized_y(&self, height: f32, anchor: f32) -> Self {
		let top = self.top() - (height - self.size.y) * anchor;
		Self {
			top_left: Vec2::new(self.left(), top),
			size: Vec2::new(self.size.x, height),
		}
	}

	pub fn resized(&self, size: Vec2, anchor: Vec2) -> Self {
		Self {
			top_left: self.top_left - (size - self.size) * anchor,
			size,
		}
	}

	pub fn expanded_x(&self, amount: f32, anchor: f32) -> Self {
		self.resized_x(self.size.x + amount, anchor)
	}

	pub fn expanded_y(&self, amount: f32, anchor: f32) -> Self {
		self.resized_y(self.size.y + amount, anchor)
	}

	pub fn expanded(&self, amount: Vec2, anchor: Vec2) -> Self {
		self.resized(self.size + amount, anchor)
	}

	pub fn scaled_x(&self, scale: f32, anchor: f32) -> Self {
		self.resized_x(self.size.x * scale, anchor)
	}

	pub fn scaled_y(&self, scale: f32, anchor: f32) -> Self {
		self.resized_y(self.size.y * scale, anchor)
	}

	pub fn scaled(&self, scale: Vec2, anchor: Vec2) -> Self {
		self.resized(self.size * scale, anchor)
	}

	pub fn padded(&self, padding: Vec2) -> Self {
		Self {
			top_left: self.top_left - padding,
			size: self.size + padding * 2.0,
		}
	}

	pub fn union(&self, other: Self) -> Self {
		let top_left = Vec2::new(
			self.top_left.x.min(other.top_left.x),
			self.top_left.y.min(other.top_left.y),
		);
		let bottom_right = Vec2::new(
			self.bottom_right().x.max(other.bottom_right().x),
			self.bottom_right().y.max(other.bottom_right().y),
		);
		Self::from_top_left_and_bottom_right(top_left, bottom_right)
	}

	pub fn contains_point(&self, point: Vec2) -> bool {
		point.x >= self.left()
			&& point.x <= self.right()
			&& point.y >= self.top()
			&& point.y <= self.bottom()
	}

	pub fn overlaps(&self, other: Self) -> bool {
		self.left() < other.right()
			&& other.left() < self.right()
			&& self.top() < other.bottom()
			&& other.top() < self.bottom()
	}
}
