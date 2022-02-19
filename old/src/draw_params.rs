use glam::Mat4;

use crate::color::Rgba;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DrawParams {
	pub transform: Mat4,
	pub color: Rgba,
}

impl DrawParams {
	pub fn new() -> Self {
		Self {
			transform: Mat4::IDENTITY,
			color: Rgba::WHITE,
		}
	}
}

impl Default for DrawParams {
	fn default() -> Self {
		Self::new()
	}
}
