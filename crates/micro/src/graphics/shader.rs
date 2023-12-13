use std::{collections::HashMap, path::Path, sync::mpsc::Sender};

use glam::{Mat3, Mat4, Vec2, Vec3};
use glow::{HasContext, NativeProgram};
use palette::LinSrgba;
use thiserror::Error;

use crate::context::Context;

use super::{texture::Texture, unused_resource::UnusedGraphicsResource};

const TEXTURE_UNITS: u32 = 32;
pub(crate) const DEFAULT_FRAGMENT_SHADER: &str = include_str!("shader/fragment.glsl");
pub(crate) const DEFAULT_VERTEX_SHADER: &str = include_str!("shader/vertex.glsl");

#[derive(Debug)]
pub struct Shader {
	pub(crate) program: NativeProgram,
	sent_textures: HashMap<String, SentTextureInfo>,
	unused_resource_sender: Sender<UnusedGraphicsResource>,
}

impl Shader {
	pub fn from_file(
		ctx: &Context,
		vertex: impl AsRef<Path>,
		fragment: impl AsRef<Path>,
	) -> Result<Self, LoadShaderError> {
		Self::from_str(
			ctx,
			&std::fs::read_to_string(vertex)?,
			&std::fs::read_to_string(fragment)?,
		)
	}

	pub fn from_vertex_file(
		ctx: &Context,
		vertex: impl AsRef<Path>,
	) -> Result<Self, LoadShaderError> {
		Self::from_vertex_str(ctx, &std::fs::read_to_string(vertex)?)
	}

	pub fn from_fragment_file(
		ctx: &Context,
		fragment: impl AsRef<Path>,
	) -> Result<Self, LoadShaderError> {
		Self::from_fragment_str(ctx, &std::fs::read_to_string(fragment)?)
	}

	pub fn from_str(ctx: &Context, vertex: &str, fragment: &str) -> Result<Self, LoadShaderError> {
		Self::new_from_gl(
			&ctx.graphics.gl,
			ctx.graphics.unused_resource_sender.clone(),
			vertex,
			fragment,
		)
		.map_err(LoadShaderError::ShaderError)
	}

	pub fn from_vertex_str(ctx: &Context, vertex: &str) -> Result<Self, LoadShaderError> {
		Self::from_str(ctx, vertex, DEFAULT_FRAGMENT_SHADER)
	}

	pub fn from_fragment_str(ctx: &Context, fragment: &str) -> Result<Self, LoadShaderError> {
		Self::from_str(ctx, DEFAULT_VERTEX_SHADER, fragment)
	}

	pub(crate) fn new_from_gl(
		gl: &glow::Context,
		unused_resource_sender: Sender<UnusedGraphicsResource>,
		vertex: &str,
		fragment: &str,
	) -> Result<Self, String> {
		let vertex_shader = unsafe {
			gl.create_shader(glow::VERTEX_SHADER)
				.expect("error creating vertex shader")
		};
		unsafe {
			gl.shader_source(vertex_shader, vertex);
			gl.compile_shader(vertex_shader);
			if !gl.get_shader_compile_status(vertex_shader) {
				return Err(gl.get_shader_info_log(vertex_shader));
			}
		}
		let fragment_shader = unsafe {
			gl.create_shader(glow::FRAGMENT_SHADER)
				.expect("error creating fragment shader")
		};
		unsafe {
			gl.shader_source(fragment_shader, fragment);
			gl.compile_shader(fragment_shader);
			if !gl.get_shader_compile_status(fragment_shader) {
				return Err(gl.get_shader_info_log(fragment_shader));
			}
		}
		let program = unsafe { gl.create_program().expect("error creating shader program") };
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
			unused_resource_sender,
			program,
			sent_textures: HashMap::new(),
		})
	}

	pub fn send_bool(&self, ctx: &Context, name: &str, value: bool) -> Result<(), UniformNotFound> {
		let gl = &ctx.graphics.gl;
		unsafe {
			gl.use_program(Some(self.program));
			let location = gl
				.get_uniform_location(self.program, name)
				.ok_or_else(|| UniformNotFound(name.to_string()))?;
			gl.uniform_1_i32(Some(&location), value.into());
		}
		Ok(())
	}

	pub fn send_i32(&self, ctx: &Context, name: &str, value: i32) -> Result<(), UniformNotFound> {
		let gl = &ctx.graphics.gl;
		unsafe {
			gl.use_program(Some(self.program));
			let location = gl
				.get_uniform_location(self.program, name)
				.ok_or_else(|| UniformNotFound(name.to_string()))?;
			gl.uniform_1_i32(Some(&location), value);
		}
		Ok(())
	}

	pub fn send_f32(&self, ctx: &Context, name: &str, value: f32) -> Result<(), UniformNotFound> {
		let gl = &ctx.graphics.gl;
		unsafe {
			gl.use_program(Some(self.program));
			let location = gl
				.get_uniform_location(self.program, name)
				.ok_or_else(|| UniformNotFound(name.to_string()))?;
			gl.uniform_1_f32(Some(&location), value);
		}
		Ok(())
	}

	pub fn send_vec2(&self, ctx: &Context, name: &str, vec2: Vec2) -> Result<(), UniformNotFound> {
		let gl = &ctx.graphics.gl;
		unsafe {
			gl.use_program(Some(self.program));
			let location = gl
				.get_uniform_location(self.program, name)
				.ok_or_else(|| UniformNotFound(name.to_string()))?;
			gl.uniform_2_f32(Some(&location), vec2.x, vec2.y);
		}
		Ok(())
	}

	pub fn send_vec3(&self, ctx: &Context, name: &str, vec3: Vec3) -> Result<(), UniformNotFound> {
		let gl = &ctx.graphics.gl;
		unsafe {
			gl.use_program(Some(self.program));
			let location = gl
				.get_uniform_location(self.program, name)
				.ok_or_else(|| UniformNotFound(name.to_string()))?;
			gl.uniform_3_f32(Some(&location), vec3.x, vec3.y, vec3.z);
		}
		Ok(())
	}

	pub fn send_mat3(&self, ctx: &Context, name: &str, mat3: Mat3) -> Result<(), UniformNotFound> {
		let gl = &ctx.graphics.gl;
		unsafe {
			gl.use_program(Some(self.program));
			let location = gl
				.get_uniform_location(self.program, name)
				.ok_or_else(|| UniformNotFound(name.to_string()))?;
			gl.uniform_matrix_3_f32_slice(Some(&location), false, &mat3.to_cols_array());
		}
		Ok(())
	}

	pub fn send_mat4(&self, ctx: &Context, name: &str, mat4: Mat4) -> Result<(), UniformNotFound> {
		let gl = &ctx.graphics.gl;
		unsafe {
			gl.use_program(Some(self.program));
			let location = gl
				.get_uniform_location(self.program, name)
				.ok_or_else(|| UniformNotFound(name.to_string()))?;
			gl.uniform_matrix_4_f32_slice(Some(&location), false, &mat4.to_cols_array());
		}
		Ok(())
	}

	pub fn send_color(
		&self,
		ctx: &Context,
		name: &str,
		color: LinSrgba,
	) -> Result<(), UniformNotFound> {
		let gl = &ctx.graphics.gl;
		unsafe {
			gl.use_program(Some(self.program));
			let location = gl
				.get_uniform_location(self.program, name)
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

	pub fn send_texture(
		&mut self,
		ctx: &Context,
		name: &str,
		texture: &Texture,
	) -> Result<(), SendTextureError> {
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
			self.send_i32(ctx, name, unit as i32)
				.map_err(|_| SendTextureError::UniformNotFound(name.to_string()))?;
		}
		Ok(())
	}

	pub(crate) fn bind_sent_textures(&self, ctx: &Context) {
		let gl = &ctx.graphics.gl;
		unsafe {
			for SentTextureInfo { texture, unit } in self.sent_textures.values() {
				gl.active_texture(glow::TEXTURE0 + *unit);
				gl.bind_texture(glow::TEXTURE_2D, Some(texture.inner.texture));
			}
			gl.active_texture(glow::TEXTURE0);
		}
	}
}

impl Drop for Shader {
	fn drop(&mut self) {
		self.unused_resource_sender
			.send(UnusedGraphicsResource::Program(self.program))
			.ok();
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
