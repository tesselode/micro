use glam::Mat4;

use crate::{
	graphics::{Shader, StencilState},
	math::URect,
};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Push {
	pub transform: Option<Mat4>,
	pub shader: Option<Shader>,
	pub stencil_state: Option<StencilState>,
	pub enable_depth_testing: Option<bool>,
	pub scissor_rect: Option<Option<URect>>,
}

impl From<Mat4> for Push {
	fn from(transform: Mat4) -> Self {
		Self {
			transform: Some(transform),
			..Default::default()
		}
	}
}

impl From<&Shader> for Push {
	fn from(shader: &Shader) -> Self {
		Self {
			shader: Some(shader.clone()),
			..Default::default()
		}
	}
}

impl From<StencilState> for Push {
	fn from(stencil_state: StencilState) -> Self {
		Self {
			stencil_state: Some(stencil_state),
			..Default::default()
		}
	}
}
