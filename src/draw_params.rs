use crate::{color::Rgba, shader::Shader};

#[derive(Debug, Clone, Default)]
pub struct DrawParams {
	pub shader: Option<Shader>,
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

	pub fn color(&self, color: impl Into<Rgba>) -> Self {
		Self {
			color: color.into(),
			..self.clone()
		}
	}
}

impl From<Rgba> for DrawParams {
    fn from(color: Rgba) -> Self {
        Self::new().color(color)
    }
}
