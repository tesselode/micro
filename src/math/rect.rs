use glam::Vec2;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rect {
	pub top_left: Vec2,
	pub bottom_right: Vec2,
}

impl Rect {
	pub fn new(top_left: Vec2, bottom_right: Vec2) -> Self {
		Self {
			top_left,
			bottom_right,
		}
	}

	pub fn from_top_left_and_size(top_left: Vec2, size: Vec2) -> Self {
		Self::new(top_left, top_left + size)
	}

	pub fn xywh(x: f32, y: f32, width: f32, height: f32) -> Self {
		Self::new(Vec2::new(x, y), Vec2::new(x + width, y + height))
	}

	pub fn size(&self) -> Vec2 {
		self.bottom_right - self.top_left
	}

	pub fn left(&self) -> f32 {
		self.top_left.x
	}

	pub fn right(&self) -> f32 {
		self.bottom_right.x
	}

	pub fn top(&self) -> f32 {
		self.top_left.y
	}

	pub fn bottom(&self) -> f32 {
		self.bottom_right.y
	}

	pub fn top_right(&self) -> Vec2 {
		Vec2::new(self.bottom_right.x, self.top_left.y)
	}

	pub fn bottom_left(&self) -> Vec2 {
		Vec2::new(self.top_left.x, self.bottom_right.y)
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
			self.bottom_right,
			self.top_right(),
			self.top_left,
			self.bottom_left(),
		]
	}

	pub fn padded(&self, padding: Vec2) -> Self {
		Self {
			top_left: self.top_left - padding,
			bottom_right: self.bottom_right + padding,
		}
	}

	pub fn union(&self, other: Self) -> Self {
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
