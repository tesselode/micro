use glam::{Mat3, Vec2};

use crate::graphics::{blend_mode::BlendMode, color::Rgba, shader::Shader};

#[derive(Debug, Clone, Copy)]
pub struct DrawParams<'a> {
	pub shader: Option<&'a Shader>,
	pub position: Vec2,
	pub rotation: f32,
	pub scale: Vec2,
	pub origin: Vec2,
	pub color: Rgba,
	pub blend_mode: BlendMode,
}

impl<'a> DrawParams<'a> {
	pub fn new() -> Self {
		Self {
			shader: None,
			position: Vec2::ZERO,
			rotation: 0.0,
			scale: Vec2::ONE,
			origin: Vec2::ZERO,
			color: Rgba::WHITE,
			blend_mode: BlendMode::default(),
		}
	}

	pub fn shader(self, shader: &'a Shader) -> Self {
		Self {
			shader: Some(shader),
			..self
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

	pub fn blend_mode(self, blend_mode: BlendMode) -> Self {
		Self { blend_mode, ..self }
	}

	pub fn transform(&self) -> Mat3 {
		Mat3::from_translation(self.position)
			* Mat3::from_rotation_z(self.rotation)
			* Mat3::from_scale(self.scale)
			* Mat3::from_translation(-self.origin)
	}
}

impl<'a> Default for DrawParams<'a> {
	fn default() -> Self {
		Self::new()
	}
}

impl<'a> From<&'a Shader> for DrawParams<'a> {
	fn from(shader: &'a Shader) -> Self {
		Self::new().shader(shader)
	}
}

impl<'a> From<Vec2> for DrawParams<'a> {
	fn from(position: Vec2) -> Self {
		Self::new().position(position)
	}
}

impl<'a> From<Rgba> for DrawParams<'a> {
	fn from(color: Rgba) -> Self {
		Self::new().color(color)
	}
}

impl<'a> From<BlendMode> for DrawParams<'a> {
	fn from(blend_mode: BlendMode) -> Self {
		Self::new().blend_mode(blend_mode)
	}
}
