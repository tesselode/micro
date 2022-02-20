use std::{path::Path, sync::Arc};

use glow::{HasContext, NativeProgram};
use thiserror::Error;

use crate::context::Context;

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

pub(crate) struct RawShader {
	gl: Arc<glow::Context>,
	pub(crate) program: NativeProgram,
}

impl RawShader {
	pub(crate) fn new(
		gl: Arc<glow::Context>,
		vertex: &str,
		fragment: &str,
	) -> Result<Self, String> {
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

pub struct Shader {
	pub(crate) raw_shader: Arc<RawShader>,
}

impl Shader {
	pub fn new(
		ctx: &Context,
		vertex: impl AsRef<Path>,
		fragment: impl AsRef<Path>,
	) -> Result<Self, LoadShaderError> {
		Ok(Self {
			raw_shader: Arc::new(RawShader::new(
				ctx.gl.clone(),
				&std::fs::read_to_string(vertex)?,
				&std::fs::read_to_string(fragment)?,
			)?),
		})
	}
}
