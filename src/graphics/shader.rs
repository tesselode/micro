use std::{collections::HashMap, path::Path, rc::Rc};

use glam::Mat4;
use glow::{HasContext, NativeProgram};
use thiserror::Error;

use crate::{context::Context, graphics::color::Rgba};

use super::texture::Texture;

const TEXTURE_UNITS: u32 = 32;

#[derive(Debug)]
pub struct Shader {
	gl: Rc<glow::Context>,
	pub(crate) program: NativeProgram,
	sent_textures: HashMap<String, SentTextureInfo>,
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
		.map_err(LoadShaderError::ShaderError)
	}

	pub(crate) fn new_from_gl(
		gl: Rc<glow::Context>,
		vertex: &str,
		fragment: &str,
	) -> Result<Self, String> {
		let vertex_shader = unsafe { gl.create_shader(glow::VERTEX_SHADER).unwrap() };
		unsafe {
			gl.shader_source(vertex_shader, vertex);
			gl.compile_shader(vertex_shader);
			if !gl.get_shader_compile_status(vertex_shader) {
				return Err(gl.get_shader_info_log(vertex_shader));
			}
		}
		let fragment_shader = unsafe { gl.create_shader(glow::FRAGMENT_SHADER).unwrap() };
		unsafe {
			gl.shader_source(fragment_shader, fragment);
			gl.compile_shader(fragment_shader);
			if !gl.get_shader_compile_status(fragment_shader) {
				return Err(gl.get_shader_info_log(fragment_shader));
			}
		}
		let program = unsafe { gl.create_program().unwrap() };
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
		Ok(Self {
			gl,
			program,
			sent_textures: HashMap::new(),
		})
	}

	pub fn send_i32(&self, name: &str, value: i32) -> Result<(), UniformNotFound> {
		unsafe {
			self.gl.use_program(Some(self.program));
			let location = self
				.gl
				.get_uniform_location(self.program, name)
				.ok_or_else(|| UniformNotFound(name.to_string()))?;
			self.gl.uniform_1_i32(Some(&location), value);
		}
		Ok(())
	}

	pub fn send_mat4(&self, name: &str, mat4: Mat4) -> Result<(), UniformNotFound> {
		unsafe {
			self.gl.use_program(Some(self.program));
			let location = self
				.gl
				.get_uniform_location(self.program, name)
				.ok_or_else(|| UniformNotFound(name.to_string()))?;
			self.gl
				.uniform_matrix_4_f32_slice(Some(&location), false, &mat4.to_cols_array());
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

	pub fn send_texture(&mut self, name: &str, texture: &Texture) -> Result<(), SendTextureError> {
		if let Some(SentTextureInfo { texture, .. }) = self.sent_textures.get_mut(name) {
			*texture = texture.clone();
		} else {
			let unit = self.sent_textures.len() as u32 + 1;
			if unit >= TEXTURE_UNITS {
				return Err(SendTextureError::TooManyTextures);
			}
			self.sent_textures.insert(
				name.to_string(),
				SentTextureInfo {
					texture: texture.clone(),
					unit,
				},
			);
			self.send_i32(name, unit as i32)
				.map_err(|_| SendTextureError::UniformNotFound(name.to_string()))?;
		}
		Ok(())
	}

	pub(crate) fn bind_sent_textures(&self) {
		unsafe {
			for SentTextureInfo { texture, unit } in self.sent_textures.values() {
				self.gl.active_texture(glow::TEXTURE0 + *unit);
				self.gl
					.bind_texture(glow::TEXTURE_2D, Some(texture.inner.texture));
			}
			self.gl.active_texture(glow::TEXTURE0);
		}
	}
}

impl Drop for Shader {
	fn drop(&mut self) {
		unsafe {
			self.gl.delete_program(self.program);
		}
	}
}

#[derive(Debug, Clone)]
struct SentTextureInfo {
	pub texture: Texture,
	pub unit: u32,
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

#[derive(Debug, Error)]
pub enum SendTextureError {
	#[error("Cannot send more than {TEXTURE_UNITS} textures")]
	TooManyTextures,
	#[error("This shader does not have a uniform called {0}")]
	UniformNotFound(String),
}
