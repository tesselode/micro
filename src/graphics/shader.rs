use std::{path::Path, rc::Rc};

use glow::{HasContext, NativeProgram};
use thiserror::Error;
use vek::Mat4;

use crate::{context::Context, error::GlError, graphics::color::Rgba};

#[derive(Debug)]
pub struct Shader {
	gl: Rc<glow::Context>,
	pub(crate) program: NativeProgram,
}

impl Shader {
	pub fn new(
		ctx: &Context,
		vertex: impl AsRef<Path>,
		fragment: impl AsRef<Path>,
	) -> Result<Self, LoadShaderError> {
		Self::new_from_gl(
			ctx.gl.clone(),
			&std::fs::read_to_string(vertex)?,
			&std::fs::read_to_string(fragment)?,
		)
		.map_err(|error| LoadShaderError::ShaderError(error.0))
	}

	pub(crate) fn new_from_gl(
		gl: Rc<glow::Context>,
		vertex: &str,
		fragment: &str,
	) -> Result<Self, GlError> {
		let vertex_shader = unsafe { gl.create_shader(glow::VERTEX_SHADER) }.map_err(GlError)?;
		unsafe {
			gl.shader_source(vertex_shader, vertex);
			gl.compile_shader(vertex_shader);
			if !gl.get_shader_compile_status(vertex_shader) {
				return Err(GlError(gl.get_shader_info_log(vertex_shader)));
			}
		}
		let fragment_shader =
			unsafe { gl.create_shader(glow::FRAGMENT_SHADER) }.map_err(GlError)?;
		unsafe {
			gl.shader_source(fragment_shader, fragment);
			gl.compile_shader(fragment_shader);
			if !gl.get_shader_compile_status(fragment_shader) {
				return Err(GlError(gl.get_shader_info_log(fragment_shader)));
			}
		}
		let program = unsafe { gl.create_program() }.map_err(GlError)?;
		unsafe {
			gl.attach_shader(program, vertex_shader);
			gl.attach_shader(program, fragment_shader);
			gl.link_program(program);
			if !gl.get_program_link_status(program) {
				return Err(GlError(gl.get_program_info_log(program)));
			}
			gl.delete_shader(vertex_shader);
			gl.delete_shader(fragment_shader);
		}
		Ok(Self { gl, program })
	}

	pub fn send_mat4(&self, name: &str, mat4: Mat4<f32>) -> Result<(), UniformNotFound> {
		unsafe {
			self.gl.use_program(Some(self.program));
			let location = self
				.gl
				.get_uniform_location(self.program, name)
				.ok_or_else(|| UniformNotFound(name.to_string()))?;
			self.gl
				.uniform_matrix_4_f32_slice(Some(&location), false, &mat4.into_col_array());
		}
		Ok(())
	}

	pub fn send_color(&self, name: &str, color: Rgba) -> Result<(), UniformNotFound> {
		unsafe {
			self.gl.use_program(Some(self.program));
			let location = self
				.gl
				.get_uniform_location(self.program, name)
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

impl Drop for Shader {
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
