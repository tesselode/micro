use glam::Mat4;

use crate::{color::Rgba, shader::Shader};

#[derive(Debug, Clone, Default)]
pub struct DrawParams {
	pub shader: Option<Shader>,
	pub transform: Mat4,
	pub color: Rgba,
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
