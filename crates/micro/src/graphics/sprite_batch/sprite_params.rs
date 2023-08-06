use glam::{Affine2, Vec2};
use palette::LinSrgba;

use crate::graphics::ColorConstants;

#[derive(Debug, Clone, Copy)]
pub struct SpriteParams {
	pub transform: Affine2,
	pub color: LinSrgba,
}

impl SpriteParams {
	pub fn new() -> Self {
		Self {
			transform: Affine2::IDENTITY,
			color: LinSrgba::WHITE,
		}
	}

	pub fn transformed(self, transform: Affine2) -> Self {
		Self {
			transform: transform * self.transform,
			..self
		}
	}

	pub fn translated(self, translation: Vec2) -> Self {
		Self {
			transform: Affine2::from_translation(translation) * self.transform,
			..self
		}
	}

	pub fn scaled(self, scale: Vec2) -> Self {
		Self {
			transform: Affine2::from_scale(scale) * self.transform,
			..self
		}
	}

	pub fn rotated(self, rotation: f32) -> Self {
		Self {
			transform: Affine2::from_angle(rotation) * self.transform,
			..self
		}
	}

	pub fn color(self, color: impl Into<LinSrgba>) -> Self {
		Self {
			color: color.into(),
			..self
		}
	}
}

impl Default for SpriteParams {
	fn default() -> Self {
		Self::new()
	}
}

impl From<Vec2> for SpriteParams {
	fn from(translation: Vec2) -> Self {
		Self::new().translated(translation)
	}
}

impl From<LinSrgba> for SpriteParams {
	fn from(color: LinSrgba) -> Self {
		Self::new().color(color)
	}
}
