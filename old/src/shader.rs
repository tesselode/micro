use std::{error::Error, fmt::Display, rc::Rc};

use glam::Mat4;
use glow::{HasContext, NativeProgram};

use crate::color::Rgba;

const REQUIRED_UNIFORMS: &[&str] = &["BlendColor", "GlobalTransform", "LocalTransform"];

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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CreateShaderError {
	MissingUniform(String),
	GlError(String),
}

impl Display for CreateShaderError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			CreateShaderError::MissingUniform(name) => {
				f.write_fmt(format_args!("The shader is missing a {} uniform", name))
			}
			CreateShaderError::GlError(error) => f.write_str(error),
		}
	}
}

impl Error for CreateShaderError {}

pub(crate) struct RawShader {
	gl: Rc<glow::Context>,
	native_program: NativeProgram,
}

impl RawShader {
	pub(crate) fn new(
		gl: Rc<glow::Context>,
		vertex: &str,
		fragment: &str,
	) -> Result<Self, CreateShaderError> {
		let native_program;
		unsafe {
			let vertex_shader = gl
				.create_shader(glow::VERTEX_SHADER)
				.map_err(CreateShaderError::GlError)?;
			gl.shader_source(vertex_shader, vertex);
			gl.compile_shader(vertex_shader);
			if !gl.get_shader_compile_status(vertex_shader) {
				return Err(CreateShaderError::GlError(
					gl.get_shader_info_log(vertex_shader),
				));
			}
			let fragment_shader = gl
				.create_shader(glow::FRAGMENT_SHADER)
				.map_err(CreateShaderError::GlError)?;
			gl.shader_source(fragment_shader, fragment);
			gl.compile_shader(fragment_shader);
			if !gl.get_shader_compile_status(fragment_shader) {
				return Err(CreateShaderError::GlError(
					gl.get_shader_info_log(fragment_shader),
				));
			}
			native_program = gl.create_program().map_err(CreateShaderError::GlError)?;
			gl.attach_shader(native_program, vertex_shader);
			gl.attach_shader(native_program, fragment_shader);
			gl.link_program(native_program);
			gl.delete_shader(vertex_shader);
			gl.delete_shader(fragment_shader);
			gl.use_program(Some(native_program));
			for name in REQUIRED_UNIFORMS {
				if gl.get_uniform_location(native_program, name).is_none() {
					return Err(CreateShaderError::MissingUniform(name.to_string()));
				}
			}
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

	fn send_mat4(&self, name: &str, mat4: Mat4) -> Result<(), UniformNotFound> {
		unsafe {
			let location = self
				.gl
				.get_uniform_location(self.native_program, name)
				.ok_or_else(|| UniformNotFound(name.to_string()))?;
			self.gl
				.uniform_matrix_4_f32_slice(Some(&location), false, &mat4.to_cols_array());
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

	pub fn send_mat4(&self, name: &str, mat4: Mat4) -> Result<(), UniformNotFound> {
		self.raw.send_mat4(name, mat4)
	}
}