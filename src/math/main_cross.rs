use derive_more::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use glam::Vec2;

use super::Axis;

#[derive(
	Debug,
	Clone,
	Copy,
	PartialEq,
	Add,
	AddAssign,
	Sub,
	SubAssign,
	Mul,
	MulAssign,
	Div,
	DivAssign,
	Neg,
)]
pub struct MainCross {
	pub main: f32,
	pub cross: f32,
}

impl MainCross {
	pub const ZERO: Self = Self::new(0.0, 0.0);

	pub const fn new(main: f32, cross: f32) -> Self {
		Self { main, cross }
	}

	pub const fn from_vec2(main_axis: Axis, vec2: Vec2) -> Self {
		match main_axis {
			Axis::Horizontal => Self::new(vec2.x, vec2.y),
			Axis::Vertical => Self::new(vec2.y, vec2.x),
		}
	}

	pub const fn into_vec2(self, main_axis: Axis) -> Vec2 {
		match main_axis {
			Axis::Horizontal => Vec2::new(self.main, self.cross),
			Axis::Vertical => Vec2::new(self.cross, self.main),
		}
	}
}
