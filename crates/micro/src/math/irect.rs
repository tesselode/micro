use glam::{IVec2, ivec2};

use super::{Rect, URect};

/// A rectangle represented by `i32`s.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serializing", derive(serde::Serialize, serde::Deserialize))]
pub struct IRect {
	/// The coordinates of the top-left corner of the rectangle.
	pub top_left: IVec2,
	/// The size of the rectangle.
	pub size: IVec2,
}

impl IRect {
	/// Returns a new `IRect`.
	pub fn new(top_left: impl Into<IVec2>, size: impl Into<IVec2>) -> Self {
		let top_left = top_left.into();
		let size = size.into();
		Self { top_left, size }
	}

	/// Creates an `IRect` from the coordinates of the top-left and bottom-right corners.
	pub fn from_corners(top_left: impl Into<IVec2>, bottom_right: impl Into<IVec2>) -> Self {
		let top_left = top_left.into();
		let bottom_right = bottom_right.into();
		Self::new(top_left, bottom_right - top_left)
	}

	/// Casts the `IRect` into a `Rect` represented by `f32`s.
	pub fn as_rect(self) -> Rect {
		Rect {
			top_left: self.top_left.as_vec2(),
			size: self.size.as_vec2(),
		}
	}

	/// Casts the `IRect` into a `URect` represented by `u32`s.
	pub fn as_urect(self) -> URect {
		URect {
			top_left: self.top_left.as_uvec2(),
			size: self.size.as_uvec2(),
		}
	}

	/// Returns the x coordinate of the left edge of the rectangle.
	pub const fn left(self) -> i32 {
		self.top_left.x
	}

	/// Returns the x coordinate of the right edge of the rectangle.
	pub const fn right(self) -> i32 {
		self.top_left.x + self.size.x
	}

	/// Returns the y coordinate of the top edge of the rectangle.
	pub const fn top(self) -> i32 {
		self.top_left.y
	}

	/// Returns the y coordinate of the bottom edge of the rectangle.
	pub const fn bottom(self) -> i32 {
		self.top_left.y + self.size.y
	}

	/// Returns the coordinates of the top right corner of the rectangle.
	pub const fn top_right(self) -> IVec2 {
		IVec2::new(self.right(), self.top())
	}

	/// Returns the coordinates of the bottom left corner of the rectangle.
	pub const fn bottom_left(self) -> IVec2 {
		IVec2::new(self.left(), self.bottom())
	}

	/// Returns the coordinates of the bottom right corner of the rectangle.
	pub const fn bottom_right(self) -> IVec2 {
		IVec2::new(self.right(), self.bottom())
	}

	/// Returns all 4 corners of the rectangle (counter-clockwise, starting from the bottom right).
	pub const fn corners(self) -> [IVec2; 4] {
		[
			self.bottom_right(),
			self.top_right(),
			self.top_left,
			self.bottom_left(),
		]
	}

	/// Returns the rectangle shifted along the X axis by the specified amount.
	pub fn translated_x(self, translation: i32) -> Self {
		self.translated(ivec2(translation, 0))
	}

	/// Returns the rectangle shifted along the Y axis by the specified amount.
	pub fn translated_y(self, translation: i32) -> Self {
		self.translated(ivec2(0, translation))
	}

	/// Returns the rectangle shifted along the X and Y axes by the specified amount.
	pub fn translated(self, translation: impl Into<IVec2>) -> Self {
		let translation = translation.into();
		Self {
			top_left: self.top_left + translation,
			size: self.size,
		}
	}

	/// Returns the rectangle with the given padding added to both the left and right sides,
	/// increasing the size of the rectangle while maintaining the same center.
	pub fn padded_x(self, padding: i32) -> Self {
		self.padded(ivec2(padding, 0))
	}

	/// Returns the rectangle with the given padding added to both the top and bottom sides,
	/// increasing the size of the rectangle while maintaining the same center.
	pub fn padded_y(self, padding: i32) -> Self {
		self.padded(ivec2(0, padding))
	}

	/// Pads the rectangle both horizontally and vertically, increasing the size of the
	/// rectangle while maintaining the same center.
	pub fn padded(self, padding: impl Into<IVec2>) -> Self {
		let padding = padding.into();
		Self {
			top_left: self.top_left - padding,
			size: self.size + padding * 2,
		}
	}

	/// Returns a rectangle that tightly hugs both this rectangle and the specified
	/// other rectangle.
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

	/// Returns `true` if a point lies within (or on the edge of) this rectangle.
	pub fn contains_point(self, point: impl Into<IVec2>) -> bool {
		let point = point.into();
		point.x >= self.left()
			&& point.x <= self.right()
			&& point.y >= self.top()
			&& point.y <= self.bottom()
	}

	/// Returns `true` if this rectangle touches the specified other rectangle.
	pub const fn overlaps(self, other: Self) -> bool {
		self.left() < other.right()
			&& other.left() < self.right()
			&& self.top() < other.bottom()
			&& other.top() < self.bottom()
	}
}
