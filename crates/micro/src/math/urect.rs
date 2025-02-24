use glam::{UVec2, uvec2};

use super::{IRect, Rect};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serializing", derive(serde::Serialize, serde::Deserialize))]
pub struct URect {
	pub top_left: UVec2,
	pub size: UVec2,
}

impl URect {
	pub fn new(top_left: impl Into<UVec2>, size: impl Into<UVec2>) -> Self {
		Self {
			top_left: top_left.into(),
			size: size.into(),
		}
	}

	pub fn from_corners(top_left: impl Into<UVec2>, bottom_right: impl Into<UVec2>) -> Self {
		let top_left = top_left.into();
		let bottom_right = bottom_right.into();
		Self::new(top_left, bottom_right - top_left)
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

	pub const fn left(self) -> u32 {
		self.top_left.x
	}

	pub const fn right(self) -> u32 {
		self.top_left.x + self.size.x
	}

	pub const fn top(self) -> u32 {
		self.top_left.y
	}

	pub const fn bottom(self) -> u32 {
		self.top_left.y + self.size.y
	}

	pub const fn top_right(self) -> UVec2 {
		UVec2::new(self.right(), self.top())
	}

	pub const fn bottom_left(self) -> UVec2 {
		UVec2::new(self.left(), self.bottom())
	}

	pub const fn bottom_right(self) -> UVec2 {
		UVec2::new(self.right(), self.bottom())
	}

	pub const fn corners(self) -> [UVec2; 4] {
		[
			self.bottom_right(),
			self.top_right(),
			self.top_left,
			self.bottom_left(),
		]
	}

	pub fn translated_x(self, translation: u32) -> Self {
		self.translated(uvec2(translation, 0))
	}

	pub fn translated_y(self, translation: u32) -> Self {
		self.translated(uvec2(0, translation))
	}

	pub fn translated(self, translation: impl Into<UVec2>) -> Self {
		let translation = translation.into();
		Self {
			top_left: self.top_left + translation,
			size: self.size,
		}
	}

	pub fn padded_x(self, padding: u32) -> Self {
		self.padded(uvec2(padding, 0))
	}

	pub fn padded_y(self, padding: u32) -> Self {
		self.padded(uvec2(0, padding))
	}

	pub fn padded(self, padding: impl Into<UVec2>) -> Self {
		let padding = padding.into();
		Self {
			top_left: self.top_left - padding,
			size: self.size + padding * 2,
		}
	}

	pub fn union(self, other: Self) -> Self {
		let top_left = UVec2::new(
			self.top_left.x.min(other.top_left.x),
			self.top_left.y.min(other.top_left.y),
		);
		let bottom_right = UVec2::new(
			self.bottom_right().x.max(other.bottom_right().x),
			self.bottom_right().y.max(other.bottom_right().y),
		);
		Self::from_corners(top_left, bottom_right)
	}

	pub fn contains_point(self, point: impl Into<UVec2>) -> bool {
		let point = point.into();
		point.x >= self.left()
			&& point.x <= self.right()
			&& point.y >= self.top()
			&& point.y <= self.bottom()
	}

	pub const fn overlaps(self, other: Self) -> bool {
		self.left() < other.right()
			&& other.left() < self.right()
			&& self.top() < other.bottom()
			&& other.top() < self.bottom()
	}
}
