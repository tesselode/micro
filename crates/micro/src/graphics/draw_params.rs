use glam::{Mat4, Vec2, Vec3};
use palette::LinSrgba;

use crate::graphics::{blend_mode::BlendMode, shader::Shader};

use super::{color_constants::ColorConstants, Culling};

#[derive(Debug, Clone, Copy)]
pub struct DrawParams<'a> {
	pub shader: Option<&'a Shader>,
	pub transform: Mat4,
	pub color: LinSrgba,
	pub blend_mode: BlendMode,
	pub culling: Culling,
}

impl<'a> DrawParams<'a> {
	pub fn new() -> Self {
		Self {
			shader: None,
			transform: Mat4::IDENTITY,
			color: LinSrgba::WHITE,
			blend_mode: BlendMode::default(),
			culling: Culling::default(),
		}
	}

	pub fn shader(self, shader: &'a Shader) -> Self {
		Self {
			shader: Some(shader),
			..self
		}
	}

	pub fn transformed(self, transform: Mat4) -> Self {
		Self {
			transform: transform * self.transform,
			..self
		}
	}

	pub fn translated_2d(self, translation: Vec2) -> Self {
		Self {
			transform: Mat4::from_translation(translation.extend(0.0)) * self.transform,
			..self
		}
	}

	pub fn translated_3d(self, translation: Vec3) -> Self {
		Self {
			transform: Mat4::from_translation(translation) * self.transform,
			..self
		}
	}

	pub fn scaled_2d(self, scale: Vec2) -> Self {
		Self {
			transform: Mat4::from_scale(scale.extend(1.0)) * self.transform,
			..self
		}
	}

	pub fn scaled_3d(self, scale: Vec3) -> Self {
		Self {
			transform: Mat4::from_scale(scale) * self.transform,
			..self
		}
	}

	pub fn rotated_x(self, rotation: f32) -> Self {
		Self {
			transform: Mat4::from_rotation_x(rotation) * self.transform,
			..self
		}
	}

	pub fn rotated_y(self, rotation: f32) -> Self {
		Self {
			transform: Mat4::from_rotation_y(rotation) * self.transform,
			..self
		}
	}

	pub fn rotated_z(self, rotation: f32) -> Self {
		Self {
			transform: Mat4::from_rotation_z(rotation) * self.transform,
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

	pub fn culling(self, culling: Culling) -> Self {
		Self { culling, ..self }
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
		Self::new().translated_2d(position)
	}
}

impl<'a> From<Vec3> for DrawParams<'a> {
	fn from(position: Vec3) -> Self {
		Self::new().translated_3d(position)
	}
}

impl<'a> From<Mat4> for DrawParams<'a> {
	fn from(transform: Mat4) -> Self {
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

impl<'a> From<Culling> for DrawParams<'a> {
	fn from(culling: Culling) -> Self {
		Self::new().culling(culling)
	}
}
