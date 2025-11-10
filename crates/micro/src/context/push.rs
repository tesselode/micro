use glam::Mat4;

use crate::{
	graphics::{Shader, StencilState},
	math::URect,
};

/// Graphics settings to be temporarily changed.
///
/// Each field is an `Option`. A value of `Some` means the option should be
/// changed from its previous value. A value of `None` means the option
/// is left the same.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Push {
	/// The global transformation for drawing operations.
	pub transform: Option<Mat4>,
	/// The shader to be used for drawing operations.
	pub shader: Option<Shader>,
	/// Controls how drawing operations interact with the stencil buffer.
	pub stencil_state: Option<StencilState>,
	/// Whether the depth buffer is used to occlude fragments.
	pub enable_depth_testing: Option<bool>,
	/// A rectangular region to crop all drawing operations to.
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
