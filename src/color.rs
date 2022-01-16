#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rgba {
	pub red: f32,
	pub green: f32,
	pub blue: f32,
	pub alpha: f32,
}

impl Rgba {
	pub const WHITE: Self = Self {
		red: 1.0,
		green: 1.0,
		blue: 1.0,
		alpha: 1.0,
	};

	pub const BLACK: Self = Self {
		red: 0.0,
		green: 0.0,
		blue: 0.0,
		alpha: 0.0,
	};
}
