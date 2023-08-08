use glam::{Affine2, Vec2};
use palette::LinSrgba;

use crate::graphics::color_constants::ColorConstants;

#[derive(Debug, Clone, Copy)]
pub struct SpriteParams {
	pub position: Vec2,
	pub rotation: f32,
	pub scale: Vec2,
	pub origin: Vec2,
	pub color: LinSrgba,
}

impl SpriteParams {
	pub fn new() -> Self {
		Self {
			position: Vec2::ZERO,
			rotation: 0.0,
			scale: Vec2::ONE,
			origin: Vec2::ZERO,
			color: LinSrgba::WHITE,
		}
	}

	pub fn position(self, position: Vec2) -> Self {
		Self { position, ..self }
	}

	pub fn rotation(self, rotation: f32) -> Self {
		Self { rotation, ..self }
	}

	pub fn scale(self, scale: Vec2) -> Self {
		Self { scale, ..self }
	}

	pub fn origin(self, origin: Vec2) -> Self {
		Self { origin, ..self }
	}

	pub fn color(self, color: impl Into<LinSrgba>) -> Self {
		Self {
			color: color.into(),
			..self
		}
	}

	pub fn transform(&self) -> Affine2 {
		Affine2::from_translation(self.position)
			* Affine2::from_angle(self.rotation)
			* Affine2::from_scale(self.scale)
			* Affine2::from_translation(-self.origin)
	}
}

impl Default for SpriteParams {
	fn default() -> Self {
		Self::new()
	}
}

impl From<Vec2> for SpriteParams {
	fn from(position: Vec2) -> Self {
		Self::new().position(position)
	}
}

impl From<LinSrgba> for SpriteParams {
	fn from(color: LinSrgba) -> Self {
		Self::new().color(color)
	}
}
