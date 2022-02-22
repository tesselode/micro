use glam::Mat4;

use crate::{blend_mode::BlendMode, color::Rgba, shader::Shader};

#[derive(Debug, Clone, Default)]
pub struct DrawParams {
	pub shader: Option<Shader>,
	pub transform: Mat4,
	pub color: Rgba,
	pub blend_mode: BlendMode,
}

impl DrawParams {
	pub fn new() -> Self {
		Self::default()
	}

	pub fn shader(&self, shader: impl Into<Option<Shader>>) -> Self {
		Self {
			shader: shader.into(),
			..self.clone()
		}
	}

	pub fn transform(&self, transform: impl Into<Mat4>) -> Self {
		Self {
			transform: transform.into(),
			..self.clone()
		}
	}

	pub fn color(&self, color: impl Into<Rgba>) -> Self {
		Self {
			color: color.into(),
			..self.clone()
		}
	}

	pub fn blend_mode(&self, blend_mode: BlendMode) -> Self {
		Self {
			blend_mode,
			..self.clone()
		}
	}
}

impl From<Shader> for DrawParams {
	fn from(shader: Shader) -> Self {
		Self::new().shader(shader)
	}
}

impl From<Mat4> for DrawParams {
	fn from(transform: Mat4) -> Self {
		Self::new().transform(transform)
	}
}

impl From<Rgba> for DrawParams {
	fn from(color: Rgba) -> Self {
		Self::new().color(color)
	}
}

impl From<BlendMode> for DrawParams {
	fn from(blend_mode: BlendMode) -> Self {
		Self::new().blend_mode(blend_mode)
	}
}
