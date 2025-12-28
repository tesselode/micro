#[cfg(test)]
mod test;

use glam::{Vec2, vec2};

use super::{Circle, IRect, URect};

/// A rectangle represented by `f32`s.
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serializing", derive(serde::Serialize, serde::Deserialize))]
pub struct Rect {
	/// The coordinates of the top-left corner of the rectangle.
	pub top_left: Vec2,
	/// The size of the rectangle.
	pub size: Vec2,
}

impl Rect {
	/// Returns a new `Rect`.
	pub fn new(top_left: impl Into<Vec2>, size: impl Into<Vec2>) -> Self {
		Self {
			top_left: top_left.into(),
			size: size.into(),
		}
	}

	/// Creates an `Rect` from the coordinates of the top-left and bottom-right corners.
	pub fn from_corners(top_left: impl Into<Vec2>, bottom_right: impl Into<Vec2>) -> Self {
		let top_left = top_left.into();
		let bottom_right = bottom_right.into();
		Self::new(top_left, bottom_right - top_left)
	}

	/// Creates a `Rect` with the given center coordinates and size.
	pub fn centered_around(center: impl Into<Vec2>, size: impl Into<Vec2>) -> Self {
		let center = center.into();
		let size = size.into();
		Self::new(center - size / 2.0, size)
	}

	/// Creates a `Rect` with the given size and a center point of (0.0, 0.0).
	pub fn centered_around_zero(size: impl Into<Vec2>) -> Self {
		Self::centered_around(Vec2::ZERO, size)
	}

	/// Casts the `Rect` into a `URect` represented by `u32`s.
	pub fn as_urect(self) -> URect {
		URect {
			top_left: self.top_left.as_uvec2(),
			size: self.size.as_uvec2(),
		}
	}

	/// Casts the `Rect` into a `IRect` represented by `i32`s.
	pub fn as_irect(self) -> IRect {
		IRect {
			top_left: self.top_left.as_ivec2(),
			size: self.size.as_ivec2(),
		}
	}

	/// Returns the x coordinate of the left edge of the rectangle.
	pub const fn left(self) -> f32 {
		self.top_left.x
	}

	/// Returns the x coordinate of the right edge of the rectangle.
	pub fn right(self) -> f32 {
		self.top_left.x + self.size.x
	}

	/// Returns the y coordinate of the top edge of the rectangle.
	pub const fn top(self) -> f32 {
		self.top_left.y
	}

	/// Returns the y coordinate of the bottom edge of the rectangle.
	pub fn bottom(self) -> f32 {
		self.top_left.y + self.size.y
	}

	/// Returns the coordinates of the top right corner of the rectangle.
	pub fn top_right(self) -> Vec2 {
		Vec2::new(self.right(), self.top())
	}

	/// Returns the coordinates of the bottom left corner of the rectangle.
	pub fn bottom_left(self) -> Vec2 {
		Vec2::new(self.left(), self.bottom())
	}

	/// Returns the coordinates of the bottom right corner of the rectangle.
	pub fn bottom_right(self) -> Vec2 {
		Vec2::new(self.right(), self.bottom())
	}

	/// Returns a point along the X axis the specified amount of the way from
	/// the left edge to the right edge. A `fraction` of `0.0` would return
	/// the left edge, `0.5` would return the horizontal center, and `1.0`
	/// would return the right edge.
	pub fn fractional_x(self, fraction: f32) -> f32 {
		self.left() + (self.right() - self.left()) * fraction
	}

	/// Returns a point along the Y axis the specified amount of the way from
	/// the top edge to the bottom edge. A `fraction` of `0.0` would return
	/// the top edge, `0.5` would return the vertical center, and `1.0`
	/// would return the bottom edge.
	pub fn fractional_y(self, fraction: f32) -> f32 {
		self.top() + (self.bottom() - self.top()) * fraction
	}

	/// Returns a point along both axes the specified amount of the way from
	/// the top left corner to the bottom right corner.
	pub fn fractional_point(self, fraction: impl Into<Vec2>) -> Vec2 {
		let fraction = fraction.into();
		Vec2::new(self.fractional_x(fraction.x), self.fractional_y(fraction.y))
	}

	/// Returns the horizontal center of the rectangle.
	pub fn center_x(self) -> f32 {
		self.fractional_x(0.5)
	}

	/// Returns the vertical center of the rectangle.
	pub fn center_y(self) -> f32 {
		self.fractional_y(0.5)
	}

	/// Returns the center of the rectangle.
	pub fn center(self) -> Vec2 {
		self.fractional_point(Vec2::splat(0.5))
	}

	/// Returns all 4 corners of the rectangle (counter-clockwise, starting from the bottom right).
	pub fn corners(self) -> [Vec2; 4] {
		[
			self.bottom_right(),
			self.top_right(),
			self.top_left,
			self.bottom_left(),
		]
	}

	/// Returns the rectangle shifted along the X axis by the specified amount.
	pub fn translated_x(self, translation: f32) -> Self {
		self.translated(vec2(translation, 0.0))
	}

	/// Returns the rectangle shifted along the Y axis by the specified amount.
	pub fn translated_y(self, translation: f32) -> Self {
		self.translated(vec2(0.0, translation))
	}

	/// Returns the rectangle shifted along the X and Y axes by the specified amount.
	pub fn translated(self, translation: impl Into<Vec2>) -> Self {
		Self {
			top_left: self.top_left + translation.into(),
			size: self.size,
		}
	}

	/// Returns the rectangle positioned so that the horizontal `anchor` is now at `x`.
	/// - An anchor of `0.0` yields a rectangle with the left edge set to `x`.
	/// - An anchor of `0.5` yields a rectangle with the horizontal center set to `x`.
	/// - An anchor of `1.0` yields a rectangle with the right edge set to `x`.
	pub fn positioned_x(self, x: f32, anchor: f32) -> Self {
		let left = x - self.size.x * anchor;
		Self {
			top_left: Vec2::new(left, self.top()),
			size: self.size,
		}
	}

	/// Returns the rectangle positioned so that the vertical `anchor` is now at `y`.
	/// - An anchor of `0.0` yields a rectangle with the top edge set to `y`.
	/// - An anchor of `0.5` yields a rectangle with the vertical center set to `y`.
	/// - An anchor of `1.0` yields a rectangle with the bottom edge set to `y`.
	pub fn positioned_y(self, y: f32, anchor: f32) -> Self {
		let top = y - self.size.y * anchor;
		Self {
			top_left: Vec2::new(self.left(), top),
			size: self.size,
		}
	}

	/// Returns the rectangle positioned so that the `anchor` is now at `position`.
	pub fn positioned(self, position: impl Into<Vec2>, anchor: impl Into<Vec2>) -> Self {
		Self {
			top_left: position.into() - self.size * anchor.into(),
			size: self.size,
		}
	}

	/// Returns this rectangle with a new width and repositioned so the `anchor`
	/// stays in the same place.
	/// - An anchor of `0.0` holds the left edge in place.
	/// - An anchor of `0.5` holds the horizontal center in place.
	/// - An anchor of `1.0` holds the right edge in place.
	pub fn resized_x(self, width: f32, anchor: f32) -> Self {
		let left = self.left() - (width - self.size.x) * anchor;
		Self {
			top_left: Vec2::new(left, self.top()),
			size: Vec2::new(width, self.size.y),
		}
	}

	/// Returns this rectangle with a new height and repositioned so the `anchor`
	/// stays in the same place.
	/// - An anchor of `0.0` holds the top edge in place.
	/// - An anchor of `0.5` holds the vertical center in place.
	/// - An anchor of `1.0` holds the bottom edge in place.
	pub fn resized_y(self, height: f32, anchor: f32) -> Self {
		let top = self.top() - (height - self.size.y) * anchor;
		Self {
			top_left: Vec2::new(self.left(), top),
			size: Vec2::new(self.size.x, height),
		}
	}

	/// Returns this rectangle with a new size and repositioned so the `anchor`
	/// stays in the same place.
	pub fn resized(self, size: impl Into<Vec2>, anchor: impl Into<Vec2>) -> Self {
		let size = size.into();
		let anchor = anchor.into();
		Self {
			top_left: self.top_left - (size - self.size) * anchor,
			size,
		}
	}

	/// Returns this rectangle with the width changed by the specified `amount`
	/// and repositioned so the `anchor` stays in the same place.
	/// - An anchor of `0.0` holds the left edge in place.
	/// - An anchor of `0.5` holds the horizontal center in place.
	/// - An anchor of `1.0` holds the right edge in place.
	pub fn expanded_x(self, amount: f32, anchor: f32) -> Self {
		self.resized_x(self.size.x + amount, anchor)
	}

	/// Returns this rectangle with the height changed by the specified `amount`
	/// and repositioned so the `anchor` stays in the same place.
	/// - An anchor of `0.0` holds the top edge in place.
	/// - An anchor of `0.5` holds the vertical center in place.
	/// - An anchor of `1.0` holds the bottom edge in place.
	pub fn expanded_y(self, amount: f32, anchor: f32) -> Self {
		self.resized_y(self.size.y + amount, anchor)
	}

	/// Returns this rectangle with the size changed by the specified `amount`
	/// and repositioned so the `anchor` stays in the same place.
	pub fn expanded(self, amount: impl Into<Vec2>, anchor: impl Into<Vec2>) -> Self {
		self.resized(self.size + amount.into(), anchor.into())
	}

	/// Returns this rectangle with the width multiplied by the specified `amount`
	/// and repositioned so the `anchor` stays in the same place.
	/// - An anchor of `0.0` holds the left edge in place.
	/// - An anchor of `0.5` holds the horizontal center in place.
	/// - An anchor of `1.0` holds the right edge in place.
	pub fn scaled_x(self, scale: f32, anchor: f32) -> Self {
		self.resized_x(self.size.x * scale, anchor)
	}

	/// Returns this rectangle with the height multiplied by the specified `amount`
	/// and repositioned so the `anchor` stays in the same place.
	/// - An anchor of `0.0` holds the top edge in place.
	/// - An anchor of `0.5` holds the vertical center in place.
	/// - An anchor of `1.0` holds the bottom edge in place.
	pub fn scaled_y(self, scale: f32, anchor: f32) -> Self {
		self.resized_y(self.size.y * scale, anchor)
	}

	/// Returns this rectangle with the size multiplied by the specified `amount`
	/// and repositioned so the `anchor` stays in the same place.
	pub fn scaled(self, scale: impl Into<Vec2>, anchor: impl Into<Vec2>) -> Self {
		self.resized(self.size * scale.into(), anchor.into())
	}

	/// Returns the rectangle with the given padding added to both the left and right sides,
	/// increasing the size of the rectangle while maintaining the same center.
	pub fn padded_x(self, padding: f32) -> Self {
		self.padded(vec2(padding, 0.0))
	}

	/// Returns the rectangle with the given padding added to both the top and bottom sides,
	/// increasing the size of the rectangle while maintaining the same center.
	pub fn padded_y(self, padding: f32) -> Self {
		self.padded(vec2(0.0, padding))
	}

	/// Pads the rectangle both horizontally and vertically, increasing the size of the
	/// rectangle while maintaining the same center.
	pub fn padded(self, padding: impl Into<Vec2>) -> Self {
		let padding = padding.into();
		Self {
			top_left: self.top_left - padding,
			size: self.size + padding * 2.0,
		}
	}

	/// Returns a rectangle that tightly hugs both this rectangle and the specified
	/// other rectangle.
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

	/// Returns `true` if a point lies within (or on the edge of) this rectangle.
	pub fn contains_point(self, point: impl Into<Vec2>) -> bool {
		let point = point.into();
		point.x >= self.left()
			&& point.x <= self.right()
			&& point.y >= self.top()
			&& point.y <= self.bottom()
	}

	/// Returns `true` if this rectangle touches the specified other rectangle.
	pub fn overlaps(self, other: Self) -> bool {
		self.left() < other.right()
			&& other.left() < self.right()
			&& self.top() < other.bottom()
			&& other.top() < self.bottom()
	}

	/// Returns `true` if this rectangle touches the specified circle.
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

impl From<Rect> for lyon_tessellation::math::Box2D {
	fn from(value: Rect) -> Self {
		lyon_tessellation::math::Box2D {
			min: lyon_tessellation::math::point(value.top_left.x, value.top_left.y),
			max: lyon_tessellation::math::point(
				value.top_left.x + value.size.x,
				value.top_left.y + value.size.y,
			),
		}
	}
}
