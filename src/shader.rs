use std::{path::Path, rc::Rc};

use glow::{HasContext, NativeProgram};
use thiserror::Error;

use crate::{color::Rgba, context::Context};

#[derive(Debug, Clone)]
pub struct Shader {
	pub(crate) raw_shader: Rc<RawShader>,
}

impl Shader {
	pub fn new(
		ctx: &Context,
		vertex: impl AsRef<Path>,
		fragment: impl AsRef<Path>,
	) -> Result<Self, LoadShaderError> {
		Ok(Self {
			raw_shader: Rc::new(RawShader::new(
				ctx.gl.clone(),
				&std::fs::read_to_string(vertex)?,
				&std::fs::read_to_string(fragment)?,
			)?),
		})
	}

	pub fn send_color(
		&self,
		ctx: &Context,
		name: &str,
		color: Rgba,
	) -> Result<(), UniformNotFound> {
		let gl = &ctx.gl;
		unsafe {
			gl.use_program(Some(self.raw_shader.program));
			let location = gl
				.get_uniform_location(self.raw_shader.program, name)
				.ok_or_else(|| UniformNotFound(name.to_string()))?;
			gl.uniform_4_f32(
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

#[derive(Debug)]
pub(crate) struct RawShader {
	gl: Rc<glow::Context>,
	pub(crate) program: NativeProgram,
}

impl RawShader {
	pub(crate) fn new(gl: Rc<glow::Context>, vertex: &str, fragment: &str) -> Result<Self, String> {
		let vertex_shader = unsafe { gl.create_shader(glow::VERTEX_SHADER) }?;
		unsafe {
			gl.shader_source(vertex_shader, vertex);
			gl.compile_shader(vertex_shader);
			if !gl.get_shader_compile_status(vertex_shader) {
				return Err(gl.get_shader_info_log(vertex_shader));
			}
		}
		let fragment_shader = unsafe { gl.create_shader(glow::FRAGMENT_SHADER) }?;
		unsafe {
			gl.shader_source(fragment_shader, fragment);
			gl.compile_shader(fragment_shader);
			if !gl.get_shader_compile_status(fragment_shader) {
				return Err(gl.get_shader_info_log(fragment_shader));
			}
		}
		let program = unsafe { gl.create_program()? };
		unsafe {
			gl.attach_shader(program, vertex_shader);
			gl.attach_shader(program, fragment_shader);
			gl.link_program(program);
			if !gl.get_program_link_status(program) {
				return Err(gl.get_program_info_log(program));
			}
			gl.delete_shader(vertex_shader);
			gl.delete_shader(fragment_shader);
		}
		Ok(Self { gl, program })
	}
}

impl Drop for RawShader {
	fn drop(&mut self) {
		unsafe {
			self.gl.delete_program(self.program);
		}
	}
}

#[derive(Debug, Error)]
pub enum LoadShaderError {
	#[error("{0}")]
	IoError(#[from] std::io::Error),
	#[error("{0}")]
	ShaderError(String),
}

impl From<String> for LoadShaderError {
	fn from(v: String) -> Self {
		Self::ShaderError(v)
	}
}

#[derive(Debug, Clone, Error)]
#[error("This shader does not have a uniform called {0}")]
pub struct UniformNotFound(pub String);
