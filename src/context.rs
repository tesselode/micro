use std::sync::Arc;

use glow::HasContext;
use sdl2::VideoSubsystem;

use crate::{
	color::Rgba,
	shader::{RawShader, Shader},
};

pub struct Context {
	pub(crate) gl: Arc<glow::Context>,
	default_shader: Shader,
}

impl Context {
	pub fn new(video: &VideoSubsystem) -> Self {
		let gl = Arc::new(unsafe {
			glow::Context::from_loader_function(|name| video.gl_get_proc_address(name) as *const _)
		});
		let default_shader = Shader {
			raw_shader: Arc::new(
				RawShader::new(
					gl.clone(),
					include_str!("vertex.glsl"),
					include_str!("fragment.glsl"),
				)
				.expect("Error compiling default shader"),
			),
		};
		unsafe {
			gl.use_program(Some(default_shader.raw_shader.program));
		}
		Self { gl, default_shader }
	}

	pub fn clear(&self, color: Rgba) {
		unsafe {
			self.gl
				.clear_color(color.red, color.green, color.blue, color.alpha);
			self.gl.clear(glow::COLOR_BUFFER_BIT);
		}
	}
}
