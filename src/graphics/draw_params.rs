use glam::Mat4;

use crate::graphics::{blend_mode::BlendMode, color::Rgba, shader::Shader};

#[derive(Debug, Clone, Copy, Default)]
pub struct DrawParams<'a> {
	pub shader: Option<&'a Shader>,
	pub transform: Mat4,
	pub color: Rgba,
	pub blend_mode: BlendMode,
}

impl<'a> DrawParams<'a> {
	pub fn new() -> Self {
		Self::default()
	}

	pub fn shader(self, shader: impl Into<Option<&'a Shader>>) -> Self {
		Self {
			shader: shader.into(),
			..self
		}
	}

	pub fn transform(self, transform: impl Into<Mat4>) -> Self {
		Self {
			transform: transform.into(),
			..self
		}
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
}

impl<'a> From<&'a Shader> for DrawParams<'a> {
	fn from(shader: &'a Shader) -> Self {
		Self::new().shader(shader)
	}
}

impl<'a> From<Mat4> for DrawParams<'a> {
	fn from(transform: Mat4) -> Self {
		Self::new().transform(transform)
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
