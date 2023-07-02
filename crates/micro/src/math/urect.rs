use glam::UVec2;

use super::{IRect, Rect};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct URect {
	pub top_left: UVec2,
	pub size: UVec2,
}

impl URect {
	pub fn new(top_left: UVec2, size: UVec2) -> Self {
		Self { top_left, size }
	}

	pub fn from_top_left_and_bottom_right(top_left: UVec2, bottom_right: UVec2) -> Self {
		Self::new(top_left, bottom_right - top_left)
	}

	pub fn from_xywh(x: u32, y: u32, width: u32, height: u32) -> Self {
		Self::new(UVec2::new(x, y), UVec2::new(width, height))
	}

	pub fn as_rect(self) -> Rect {
		Rect {
			top_left: self.top_left.as_vec2(),
			size: self.size.as_vec2(),
		}
	}

	pub fn as_irect(self) -> IRect {
		IRect {
			top_left: self.top_left.as_ivec2(),
			size: self.size.as_ivec2(),
		}
	}

	pub fn left(&self) -> u32 {
		self.top_left.x
	}

	pub fn right(&self) -> u32 {
		self.top_left.x + self.size.x
	}

	pub fn top(&self) -> u32 {
		self.top_left.y
	}

	pub fn bottom(&self) -> u32 {
		self.top_left.y + self.size.y
	}

	pub fn top_right(&self) -> UVec2 {
		UVec2::new(self.right(), self.top())
	}

	pub fn bottom_left(&self) -> UVec2 {
		UVec2::new(self.left(), self.bottom())
	}

	pub fn bottom_right(&self) -> UVec2 {
		UVec2::new(self.right(), self.bottom())
	}

	pub fn fractional_x(&self, fraction: u32) -> u32 {
		self.left() + (self.right() - self.left()) * fraction
	}

	pub fn fractional_y(&self, fraction: u32) -> u32 {
		self.top() + (self.bottom() - self.top()) * fraction
	}

	pub fn fractional_point(&self, fraction: UVec2) -> UVec2 {
		UVec2::new(self.fractional_x(fraction.x), self.fractional_y(fraction.y))
	}

	pub fn corners(&self) -> [UVec2; 4] {
		[
			self.bottom_right(),
			self.top_right(),
			self.top_left,
			self.bottom_left(),
		]
	}

	pub fn translated(&self, translation: UVec2) -> Self {
		Self {
			top_left: self.top_left + translation,
			size: self.size,
		}
	}

	pub fn padded(&self, padding: UVec2) -> Self {
		Self {
			top_left: self.top_left - padding,
			size: self.size + padding * 2,
		}
	}

	pub fn union(&self, other: Self) -> Self {
		let top_left = UVec2::new(
			self.top_left.x.min(other.top_left.x),
			self.top_left.y.min(other.top_left.y),
		);
		let bottom_right = UVec2::new(
			self.bottom_right().x.max(other.bottom_right().x),
			self.bottom_right().y.max(other.bottom_right().y),
		);
		Self::from_top_left_and_bottom_right(top_left, bottom_right)
	}

	pub fn contains_point(&self, point: UVec2) -> bool {
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
