use vek::{Mat4, Vec2, Vec3};

use crate::graphics::color::Rgba;

#[derive(Debug, Clone, Copy)]
pub struct SpriteParams {
	pub position: Vec2<f32>,
	pub rotation: f32,
	pub scale: Vec2<f32>,
	pub origin: Vec2<f32>,
	pub color: Rgba,
}

impl SpriteParams {
	pub fn new() -> Self {
		Self {
			position: Vec2::zero(),
			rotation: 0.0,
			scale: Vec2::one(),
			origin: Vec2::zero(),
			color: Rgba::WHITE,
		}
	}

	pub fn position(self, position: Vec2<f32>) -> Self {
		Self { position, ..self }
	}

	pub fn rotation(self, rotation: f32) -> Self {
		Self { rotation, ..self }
	}

	pub fn scale(self, scale: Vec2<f32>) -> Self {
		Self { scale, ..self }
	}

	pub fn origin(self, origin: Vec2<f32>) -> Self {
		Self { origin, ..self }
	}

	pub fn color(self, color: impl Into<Rgba>) -> Self {
		Self {
			color: color.into(),
			..self
		}
	}

	pub fn transform(&self) -> Mat4<f32> {
		Mat4::translation_2d(-self.origin)
			.scaled_3d(Vec3::new(self.scale.x, self.scale.y, 1.0))
			.rotated_z(self.rotation)
			.translated_2d(self.position)
	}
}

impl Default for SpriteParams {
	fn default() -> Self {
		Self::new()
	}
}

impl From<Vec2<f32>> for SpriteParams {
	fn from(position: Vec2<f32>) -> Self {
		Self::new().position(position)
	}
}

impl From<Rgba> for SpriteParams {
	fn from(color: Rgba) -> Self {
		Self::new().color(color)
	}
}
