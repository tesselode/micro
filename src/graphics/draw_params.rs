use vek::{Mat4, Vec2, Vec3};

use crate::graphics::{blend_mode::BlendMode, color::Rgba, shader::Shader};

#[derive(Debug, Clone, Copy)]
pub struct DrawParams<'a> {
	pub shader: Option<&'a Shader>,
	pub position: Vec2<f32>,
	pub rotation: f32,
	pub scale: Vec2<f32>,
	pub origin: Vec2<f32>,
	pub color: Rgba,
	pub blend_mode: BlendMode,
}

impl<'a> DrawParams<'a> {
	pub fn new() -> Self {
		Self {
			shader: None,
			position: Vec2::zero(),
			rotation: 0.0,
			scale: Vec2::one(),
			origin: Vec2::zero(),
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

	pub fn blend_mode(self, blend_mode: BlendMode) -> Self {
		Self { blend_mode, ..self }
	}

	pub fn transform(&self) -> Mat4<f32> {
		Mat4::translation_2d(-self.origin)
			.scaled_3d(Vec3::new(self.scale.x, self.scale.y, 1.0))
			.rotated_z(self.rotation)
			.translated_2d(self.position)
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

impl<'a> From<Vec2<f32>> for DrawParams<'a> {
	fn from(position: Vec2<f32>) -> Self {
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
