use glam::{Affine2, Vec2};
use palette::LinSrgba;

use crate::{color::ColorConstants, graphics::IntoScale2d};

/// Settings for a sprite in a [`SpriteBatch`](super::SpriteBatch).
#[derive(Debug, Clone, Copy)]
pub struct SpriteParams {
	/// The 2D transform of this sprite.
	pub transform: Affine2,
	/// The blend color of this sprite.
	pub color: LinSrgba,
}

impl SpriteParams {
	/// Creates a new [`SpriteParams`] with the default settings.
	pub fn new() -> Self {
		Self {
			transform: Affine2::IDENTITY,
			color: LinSrgba::WHITE,
		}
	}

	/// Applies the specified `transform` on top of the existing transform.
	pub fn transformed(self, transform: Affine2) -> Self {
		Self {
			transform: transform * self.transform,
			..self
		}
	}

	/// Moves the sprite by the specified `translation` vector.
	pub fn translated(self, translation: impl Into<Vec2>) -> Self {
		Self {
			transform: Affine2::from_translation(translation.into()) * self.transform,
			..self
		}
	}

	/// Scales the sprite by the specified amount along the X and Y axes.
	pub fn scaled(self, scale: impl IntoScale2d) -> Self {
		Self {
			transform: Affine2::from_scale(scale.into_scale_2d()) * self.transform,
			..self
		}
	}

	/// Rotates the sprite by the specified amount (in radians).
	pub fn rotated(self, rotation: f32) -> Self {
		Self {
			transform: Affine2::from_angle(rotation) * self.transform,
			..self
		}
	}

	/// Sets the blend color of the sprite.
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
