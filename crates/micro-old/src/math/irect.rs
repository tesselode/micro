use glam::IVec2;

use super::Rect;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct IRect {
	pub top_left: IVec2,
	pub bottom_right: IVec2,
}

impl IRect {
	pub fn new(top_left: IVec2, bottom_right: IVec2) -> Self {
		Self {
			top_left,
			bottom_right,
		}
	}

	pub fn as_rect(self) -> Rect {
		Rect::new(self.top_left.as_vec2(), self.bottom_right.as_vec2())
	}

	pub fn from_top_left_and_size(top_left: IVec2, size: IVec2) -> Self {
		Self::new(top_left, top_left + size)
	}

	pub fn xywh(x: i32, y: i32, width: i32, height: i32) -> Self {
		Self::new(IVec2::new(x, y), IVec2::new(x + width, y + height))
	}

	pub fn size(&self) -> IVec2 {
		self.bottom_right - self.top_left
	}

	pub fn left(&self) -> i32 {
		self.top_left.x
	}

	pub fn right(&self) -> i32 {
		self.bottom_right.x
	}

	pub fn top(&self) -> i32 {
		self.top_left.y
	}

	pub fn bottom(&self) -> i32 {
		self.bottom_right.y
	}

	pub fn top_right(&self) -> IVec2 {
		IVec2::new(self.bottom_right.x, self.top_left.y)
	}

	pub fn bottom_left(&self) -> IVec2 {
		IVec2::new(self.top_left.x, self.bottom_right.y)
	}

	pub fn fractional_x(&self, fraction: i32) -> i32 {
		self.left() + (self.right() - self.left()) * fraction
	}

	pub fn fractional_y(&self, fraction: i32) -> i32 {
		self.top() + (self.bottom() - self.top()) * fraction
	}

	pub fn fractional_point(&self, fraction: IVec2) -> IVec2 {
		IVec2::new(self.fractional_x(fraction.x), self.fractional_y(fraction.y))
	}

	pub fn corners(&self) -> [IVec2; 4] {
		[
			self.bottom_right,
			self.top_right(),
			self.top_left,
			self.bottom_left(),
		]
	}

	pub fn padded(&self, padding: IVec2) -> Self {
		Self {
			top_left: self.top_left - padding,
			bottom_right: self.bottom_right + padding,
		}
	}

	pub fn union(&self, other: Self) -> Self {
		Self {
			top_left: IVec2::new(
				self.top_left.x.min(other.top_left.x),
				self.top_left.y.min(other.top_left.y),
			),
			bottom_right: IVec2::new(
				self.bottom_right.x.max(other.bottom_right.x),
				self.bottom_right.y.max(other.bottom_right.y),
			),
		}
	}

	pub fn contains_point(&self, point: IVec2) -> bool {
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
