use crate::color::Rgba;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DrawParams {
	pub color: Rgba,
}

impl DrawParams {
	pub fn new() -> Self {
		Self { color: Rgba::WHITE }
	}
}

impl Default for DrawParams {
	fn default() -> Self {
		Self::new()
	}
}
