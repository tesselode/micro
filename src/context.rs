use std::sync::Arc;

use glow::HasContext;

use crate::color::Rgba;

pub struct Context {
	pub(crate) gl: Arc<glow::Context>,
}

impl Context {
	pub fn clear(&self, color: Rgba) {
		unsafe {
			self.gl
				.clear_color(color.red, color.green, color.blue, color.alpha);
			self.gl.clear(glow::COLOR_BUFFER_BIT);
		}
	}
}
