// based on https://github.com/17cupsofcoffee/tetra/blob/main/src/graphics.rs#L704

use glow::HasContext;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BlendMode {
	Alpha(BlendAlphaMode),
	Add(BlendAlphaMode),
	Subtract(BlendAlphaMode),
	Multiply,
}

impl BlendMode {
	pub(crate) unsafe fn apply(&self, gl: &glow::Context) {
		self.as_state().apply(gl);
	}

	fn as_state(&self) -> BlendState {
		match self {
			BlendMode::Alpha(alpha_mode) => BlendState {
				color_operation: glow::FUNC_ADD,
				color_source: match alpha_mode {
					BlendAlphaMode::AlphaMultiply => glow::SRC_ALPHA,
					BlendAlphaMode::Premultiplied => glow::ONE,
				},
				color_destination: glow::ONE_MINUS_SRC_ALPHA,
				alpha_operation: glow::FUNC_ADD,
				alpha_source: glow::ONE,
				alpha_destination: glow::ONE_MINUS_SRC_ALPHA,
			},
			BlendMode::Add(alpha_mode) => BlendState {
				color_operation: glow::FUNC_ADD,
				color_source: match alpha_mode {
					BlendAlphaMode::AlphaMultiply => glow::SRC_ALPHA,
					BlendAlphaMode::Premultiplied => glow::ONE,
				},
				color_destination: glow::ONE,
				alpha_operation: glow::FUNC_ADD,
				alpha_source: glow::ZERO,
				alpha_destination: glow::ONE,
			},
			BlendMode::Subtract(alpha_mode) => BlendState {
				color_operation: glow::FUNC_REVERSE_SUBTRACT,
				color_source: match alpha_mode {
					BlendAlphaMode::AlphaMultiply => glow::SRC_ALPHA,
					BlendAlphaMode::Premultiplied => glow::ONE,
				},
				color_destination: glow::ONE,
				alpha_operation: glow::FUNC_REVERSE_SUBTRACT,
				alpha_source: glow::ZERO,
				alpha_destination: glow::ONE,
			},
			BlendMode::Multiply => BlendState {
				color_operation: glow::FUNC_ADD,
				color_source: glow::DST_COLOR,
				color_destination: glow::ZERO,
				alpha_operation: glow::FUNC_ADD,
				alpha_source: glow::DST_ALPHA,
				alpha_destination: glow::ZERO,
			},
		}
	}
}

impl Default for BlendMode {
	fn default() -> Self {
		Self::Alpha(BlendAlphaMode::AlphaMultiply)
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BlendAlphaMode {
	AlphaMultiply,
	Premultiplied,
}

impl Default for BlendAlphaMode {
	fn default() -> Self {
		Self::AlphaMultiply
	}
}

struct BlendState {
	pub(crate) color_operation: u32,
	pub(crate) color_source: u32,
	pub(crate) color_destination: u32,
	pub(crate) alpha_operation: u32,
	pub(crate) alpha_source: u32,
	pub(crate) alpha_destination: u32,
}

impl BlendState {
	unsafe fn apply(self, gl: &glow::Context) {
		gl.blend_func_separate(
			self.color_source,
			self.color_destination,
			self.alpha_source,
			self.alpha_destination,
		);
		gl.blend_equation_separate(self.color_operation, self.alpha_operation);
	}
}
