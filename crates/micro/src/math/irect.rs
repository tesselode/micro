use glam::{ivec2, IVec2};

use super::{Rect, URect};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serializing", derive(serde::Serialize, serde::Deserialize))]
pub struct IRect {
	pub top_left: IVec2,
	pub size: IVec2,
}

impl IRect {
	pub fn new(top_left: impl Into<IVec2>, size: impl Into<IVec2>) -> Self {
		let top_left = top_left.into();
		let size = size.into();
		Self { top_left, size }
	}

	pub fn from_corners(top_left: impl Into<IVec2>, bottom_right: impl Into<IVec2>) -> Self {
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

	pub fn as_urect(self) -> URect {
		URect {
			top_left: self.top_left.as_uvec2(),
			size: self.size.as_uvec2(),
		}
	}

	pub const fn left(self) -> i32 {
		self.top_left.x
	}

	pub const fn right(self) -> i32 {
		self.top_left.x + self.size.x
	}

	pub const fn top(self) -> i32 {
		self.top_left.y
	}

	pub const fn bottom(self) -> i32 {
		self.top_left.y + self.size.y
	}

	pub const fn top_right(self) -> IVec2 {
		IVec2::new(self.right(), self.top())
	}

	pub const fn bottom_left(self) -> IVec2 {
		IVec2::new(self.left(), self.bottom())
	}

	pub const fn bottom_right(self) -> IVec2 {
		IVec2::new(self.right(), self.bottom())
	}

	pub const fn corners(self) -> [IVec2; 4] {
		[
			self.bottom_right(),
			self.top_right(),
			self.top_left,
			self.bottom_left(),
		]
	}

	pub fn translated_x(self, translation: i32) -> Self {
		self.translated(ivec2(translation, 0))
	}

	pub fn translated_y(self, translation: i32) -> Self {
		self.translated(ivec2(0, translation))
	}

	pub fn translated(self, translation: impl Into<IVec2>) -> Self {
		let translation = translation.into();
		Self {
			top_left: self.top_left + translation,
			size: self.size,
		}
	}

	pub fn padded_x(self, padding: i32) -> Self {
		self.padded(ivec2(padding, 0))
	}

	pub fn padded_y(self, padding: i32) -> Self {
		self.padded(ivec2(0, padding))
	}

	pub fn padded(self, padding: impl Into<IVec2>) -> Self {
		let padding = padding.into();
		Self {
			top_left: self.top_left - padding,
			size: self.size + padding * 2,
		}
	}

	pub fn union(self, other: Self) -> Self {
		let top_left = IVec2::new(
			self.top_left.x.min(other.top_left.x),
			self.top_left.y.min(other.top_left.y),
		);
		let bottom_right = IVec2::new(
			self.bottom_right().x.max(other.bottom_right().x),
			self.bottom_right().y.max(other.bottom_right().y),
		);
		Self::from_corners(top_left, bottom_right)
	}

	pub fn contains_point(self, point: impl Into<IVec2>) -> bool {
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
