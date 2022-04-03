#[cfg(test)]
mod test;

use vek::Vec2;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rect {
	pub top_left: Vec2<f32>,
	pub bottom_right: Vec2<f32>,
}

impl Rect {
	pub fn new(top_left: Vec2<f32>, bottom_right: Vec2<f32>) -> Self {
		Self {
			top_left,
			bottom_right,
		}
	}

	pub fn from_top_left_and_size(top_left: Vec2<f32>, size: Vec2<f32>) -> Self {
		Self::new(top_left, top_left + size)
	}

	pub fn xywh(x: f32, y: f32, width: f32, height: f32) -> Self {
		Self::new(Vec2::new(x, y), Vec2::new(x + width, y + height))
	}

	pub fn size(&self) -> Vec2<f32> {
		self.bottom_right - self.top_left
	}

	pub fn relative_x(&self, fraction: f32) -> f32 {
		self.top_left.x + (self.bottom_right.x - self.top_left.x) * fraction
	}

	pub fn relative_y(&self, fraction: f32) -> f32 {
		self.top_left.y + (self.bottom_right.y - self.top_left.y) * fraction
	}

	pub fn relative_point(&self, fraction: Vec2<f32>) -> Vec2<f32> {
		Vec2::new(self.relative_x(fraction.x), self.relative_y(fraction.y))
	}

	pub fn left(&self) -> f32 {
		self.relative_x(0.0)
	}

	pub fn center_x(&self) -> f32 {
		self.relative_x(0.5)
	}

	pub fn right(&self) -> f32 {
		self.relative_x(1.0)
	}

	pub fn top(&self) -> f32 {
		self.relative_y(0.0)
	}

	pub fn center_y(&self) -> f32 {
		self.relative_y(0.5)
	}

	pub fn bottom(&self) -> f32 {
		self.relative_y(1.0)
	}

	pub fn top_left(&self) -> Vec2<f32> {
		self.top_left
	}

	pub fn top_right(&self) -> Vec2<f32> {
		self.relative_point(Vec2::new(1.0, 0.0))
	}

	pub fn bottom_left(&self) -> Vec2<f32> {
		self.relative_point(Vec2::new(0.0, 1.0))
	}

	pub fn bottom_right(&self) -> Vec2<f32> {
		self.bottom_right
	}

	pub fn center(&self) -> Vec2<f32> {
		self.relative_point(Vec2::new(0.5, 0.5))
	}

	pub fn padded(&self, padding: Vec2<f32>) -> Self {
		Self {
			top_left: self.top_left - padding,
			bottom_right: self.bottom_right + padding,
		}
	}

	pub fn combined_with(&self, other: Self) -> Self {
		Self {
			top_left: Vec2::new(
				self.top_left.x.min(other.top_left.x),
				self.top_left.y.min(other.top_left.y),
			),
			bottom_right: Vec2::new(
				self.bottom_right.x.max(other.bottom_right.x),
				self.bottom_right.y.max(other.bottom_right.y),
			),
		}
	}

	pub fn overlaps(&self, other: Self) -> bool {
		self.left() < other.right()
			&& other.left() < self.right()
			&& self.top() < other.bottom()
			&& other.top() < self.bottom()
	}
}
