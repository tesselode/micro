use glam::Vec2;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rect {
	pub top_left: Vec2,
	pub size: Vec2,
}

impl Rect {
	pub fn new(top_left: Vec2, size: Vec2) -> Self {
		Self { top_left, size }
	}

	pub fn top_right(&self) -> Vec2 {
		self.top_left + Vec2::new(self.size.x, 0.0)
	}

	pub fn bottom_left(&self) -> Vec2 {
		self.top_left + Vec2::new(0.0, self.size.y)
	}

	pub fn bottom_right(&self) -> Vec2 {
		self.top_left + self.size
	}
}
