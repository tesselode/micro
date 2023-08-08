use glam::{Affine2, Vec2};
use palette::LinSrgba;

use crate::graphics::{blend_mode::BlendMode, shader::Shader};

use super::color_constants::ColorConstants;

#[derive(Debug, Clone, Copy)]
pub struct DrawParams<'a> {
	pub shader: Option<&'a Shader>,
	pub transform: Affine2,
	pub color: LinSrgba,
	pub blend_mode: BlendMode,
}

impl<'a> DrawParams<'a> {
	pub fn new() -> Self {
		Self {
			shader: None,
			transform: Affine2::IDENTITY,
			color: LinSrgba::WHITE,
			blend_mode: BlendMode::default(),
		}
	}

	pub fn shader(self, shader: &'a Shader) -> Self {
		Self {
			shader: Some(shader),
			..self
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

	pub fn blend_mode(self, blend_mode: BlendMode) -> Self {
		Self { blend_mode, ..self }
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
		Self::new().translated(position)
	}
}

impl<'a> From<Affine2> for DrawParams<'a> {
	fn from(transform: Affine2) -> Self {
		Self::new().transformed(transform)
	}
}

impl<'a> From<LinSrgba> for DrawParams<'a> {
	fn from(color: LinSrgba) -> Self {
		Self::new().color(color)
	}
}

impl<'a> From<BlendMode> for DrawParams<'a> {
	fn from(blend_mode: BlendMode) -> Self {
		Self::new().blend_mode(blend_mode)
	}
}
