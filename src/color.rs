#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rgba {
	pub red: f32,
	pub green: f32,
	pub blue: f32,
	pub alpha: f32,
}

impl Rgba {
	pub const WHITE: Self = Self::new(1.0, 1.0, 1.0, 1.0);
	pub const BLACK: Self = Self::new(0.0, 0.0, 0.0, 1.0);
	pub const RED: Self = Self::new(1.0, 0.0, 0.0, 1.0);
	pub const GREEN: Self = Self::new(0.0, 1.0, 0.0, 1.0);
	pub const BLUE: Self = Self::new(0.0, 0.0, 1.0, 1.0);

	pub const fn new(red: f32, green: f32, blue: f32, alpha: f32) -> Self {
		Self {
			red,
			green,
			blue,
			alpha,
		}
	}

	pub const fn with_alpha(self, alpha: f32) -> Self {
		Self { alpha, ..self }
	}
}

impl Default for Rgba {
	fn default() -> Self {
		Self {
			red: 1.0,
			green: 1.0,
			blue: 1.0,
			alpha: 1.0,
		}
	}
}
