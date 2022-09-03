use glam::{Mat4, Vec2};

use crate::graphics::color::Rgba;

#[derive(Debug, Clone, Copy)]
pub struct SpriteParams {
	pub position: Vec2,
	pub rotation: f32,
	pub scale: Vec2,
	pub origin: Vec2,
	pub color: Rgba,
}

impl SpriteParams {
	pub fn new() -> Self {
		Self {
			position: Vec2::ZERO,
			rotation: 0.0,
			scale: Vec2::ONE,
			origin: Vec2::ZERO,
			color: Rgba::WHITE,
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

	pub fn color(self, color: impl Into<Rgba>) -> Self {
		Self {
			color: color.into(),
			..self
		}
	}

	pub fn transform(&self) -> Mat4 {
		Mat4::from_translation(self.position.extend(0.0))
			* Mat4::from_rotation_z(self.rotation)
			* Mat4::from_scale(self.scale.extend(1.0))
			* Mat4::from_translation((-self.origin).extend(0.0))
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

impl From<Rgba> for SpriteParams {
	fn from(color: Rgba) -> Self {
		Self::new().color(color)
	}
}
