use std::{error::Error, rc::Rc};

use glow::{HasContext, NativeProgram};

use crate::color::Rgba;

pub(crate) struct RawShader {
	gl: Rc<glow::Context>,
	native_program: NativeProgram,
}

impl RawShader {
	pub(crate) fn new(
		gl: Rc<glow::Context>,
		vertex: &str,
		fragment: &str,
	) -> Result<Self, Box<dyn Error>> {
		let native_program;
		unsafe {
			let vertex_shader = gl.create_shader(glow::VERTEX_SHADER)?;
			gl.shader_source(vertex_shader, vertex);
			gl.compile_shader(vertex_shader);
			let fragment_shader = gl.create_shader(glow::FRAGMENT_SHADER)?;
			gl.shader_source(fragment_shader, fragment);
			gl.compile_shader(fragment_shader);
			native_program = gl.create_program()?;
			gl.attach_shader(native_program, vertex_shader);
			gl.attach_shader(native_program, fragment_shader);
			gl.link_program(native_program);
			gl.delete_shader(vertex_shader);
			gl.delete_shader(fragment_shader);
			gl.use_program(Some(native_program));
		}
		Ok(Self { gl, native_program })
	}

	fn send_color(&self, name: &str, color: Rgba) {
		unsafe {
			let location = self
				.gl
				.get_uniform_location(self.native_program, name)
				.unwrap();
			self.gl.uniform_4_f32(
				Some(&location),
				color.red,
				color.green,
				color.blue,
				color.alpha,
			);
		}
	}
}

impl Drop for RawShader {
	fn drop(&mut self) {
		unsafe {
			self.gl.delete_program(self.native_program);
		}
	}
}

pub struct Shader {
	raw: Rc<RawShader>,
}

impl Shader {
	pub(crate) fn from_raw(raw: RawShader) -> Self {
		Self { raw: Rc::new(raw) }
	}

	pub fn send_color(&self, name: &str, color: Rgba) {
		self.raw.send_color(name, color);
	}
}