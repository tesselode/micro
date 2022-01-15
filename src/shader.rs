use std::{error::Error, fmt::Display, rc::Rc};

use glow::{HasContext, NativeProgram};

use crate::color::Rgba;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UniformNotFound(pub String);

impl Display for UniformNotFound {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_fmt(format_args!(
			"Could not find a uniform with the name {}",
			self.0
		))
	}
}

impl Error for UniformNotFound {}

pub(crate) struct RawShader {
	gl: Rc<glow::Context>,
	native_program: NativeProgram,
}

impl RawShader {
	pub(crate) fn new(gl: Rc<glow::Context>, vertex: &str, fragment: &str) -> Result<Self, String> {
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

	fn send_color(&self, name: &str, color: Rgba) -> Result<(), UniformNotFound> {
		unsafe {
			let location = self
				.gl
				.get_uniform_location(self.native_program, name)
				.ok_or_else(|| UniformNotFound(name.to_string()))?;
			self.gl.uniform_4_f32(
				Some(&location),
				color.red,
				color.green,
				color.blue,
				color.alpha,
			);
		}
		Ok(())
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

	pub fn send_color(&self, name: &str, color: Rgba) -> Result<(), UniformNotFound> {
		self.raw.send_color(name, color)
	}
}
