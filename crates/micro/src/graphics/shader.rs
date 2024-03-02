use std::{
	collections::HashMap,
	path::Path,
	sync::{mpsc::Sender, Mutex},
};

use glam::{Mat3, Mat4, Vec2, Vec3, Vec4};
use glow::{HasContext, NativeProgram};
use indexmap::IndexMap;
use once_cell::sync::Lazy;
use palette::LinSrgba;
use regex_lite::Regex;
use thiserror::Error;

use crate::context::Context;

use super::{texture::Texture, unused_resource::UnusedGraphicsResource};

const TEXTURE_UNITS: u32 = 32;
pub(crate) const DEFAULT_FRAGMENT_SHADER: &str = include_str!("shader/fragment.glsl");
pub(crate) const DEFAULT_VERTEX_SHADER: &str = include_str!("shader/vertex.glsl");
static COMBINED_SHADER_SECTION_REGEX: Lazy<Regex> =
	Lazy::new(|| Regex::new(r"// @(\w*)").expect("error compiling combined shader section regex"));
const COMBINED_SHADER_VERTEX_SECTION_NAME: &str = "VERTEX";
const COMBINED_SHADER_FRAGMENT_SECTION_NAME: &str = "FRAGMENT";

#[derive(Debug)]
pub struct Shader {
	pub(crate) program: NativeProgram,
	sent_textures: HashMap<String, SentTextureInfo>,
	uniform_values: Mutex<IndexMap<String, UniformValue>>,
	unused_resource_sender: Sender<UnusedGraphicsResource>,
}

impl Shader {
	pub fn from_files(
		vertex: impl AsRef<Path>,
		fragment: impl AsRef<Path>,
	) -> Result<Self, LoadShaderError> {
		Self::from_strs(
			&std::fs::read_to_string(vertex)?,
			&std::fs::read_to_string(fragment)?,
		)
	}

	pub fn from_combined_file(combined: impl AsRef<Path>) -> Result<Self, LoadShaderError> {
		Self::from_combined_str(&std::fs::read_to_string(combined)?)
	}

	pub fn from_vertex_file(vertex: impl AsRef<Path>) -> Result<Self, LoadShaderError> {
		Self::from_vertex_str(&std::fs::read_to_string(vertex)?)
	}

	pub fn from_fragment_file(fragment: impl AsRef<Path>) -> Result<Self, LoadShaderError> {
		Self::from_fragment_str(&std::fs::read_to_string(fragment)?)
	}

	pub fn from_strs(vertex: &str, fragment: &str) -> Result<Self, LoadShaderError> {
		Context::with(|ctx| {
			Self::new_from_gl(
				&ctx.graphics.gl,
				ctx.graphics.unused_resource_sender.clone(),
				vertex,
				fragment,
			)
			.map_err(LoadShaderError::ShaderError)
		})
	}

	pub fn from_combined_str(combined: &str) -> Result<Self, LoadShaderError> {
		let split_code = COMBINED_SHADER_SECTION_REGEX
			.captures_iter(combined)
			.zip(COMBINED_SHADER_SECTION_REGEX.split(combined).skip(1))
			.map(|(section_delimiter_captures, code)| {
				let section_name = section_delimiter_captures[1].to_string();
				(section_name, code.to_string())
			})
			.collect::<HashMap<_, _>>();
		Self::from_strs(
			split_code
				.get(COMBINED_SHADER_VERTEX_SECTION_NAME)
				.map(String::as_str)
				.unwrap_or(DEFAULT_VERTEX_SHADER),
			split_code
				.get(COMBINED_SHADER_FRAGMENT_SECTION_NAME)
				.map(String::as_str)
				.unwrap_or(DEFAULT_FRAGMENT_SHADER),
		)
	}

	pub fn from_vertex_str(vertex: &str) -> Result<Self, LoadShaderError> {
		Self::from_strs(vertex, DEFAULT_FRAGMENT_SHADER)
	}

	pub fn from_fragment_str(fragment: &str) -> Result<Self, LoadShaderError> {
		Self::from_strs(DEFAULT_VERTEX_SHADER, fragment)
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
			program,
			sent_textures: HashMap::new(),
			uniform_values: Mutex::new(IndexMap::new()),
			unused_resource_sender,
		})
	}

	pub fn uniform_value(&self, name: &str) -> Option<UniformValue> {
		self.uniform_values
			.lock()
			.expect("uniform mutex poisoned")
			.get(name)
			.cloned()
	}

	pub fn send_bool(&self, name: &str, value: bool) -> Result<(), UniformNotFound> {
		Context::with(|ctx| {
			let gl = &ctx.graphics.gl;
			unsafe {
				gl.use_program(Some(self.program));
				let location = gl
					.get_uniform_location(self.program, name)
					.ok_or_else(|| UniformNotFound(name.to_string()))?;
				gl.uniform_1_i32(Some(&location), value.into());
			}
			Ok(())
		})?;
		self.uniform_values
			.lock()
			.expect("uniform mutex poisoned")
			.insert(name.to_string(), UniformValue::Bool(value));
		Ok(())
	}

	pub fn send_i32(&self, name: &str, value: i32) -> Result<(), UniformNotFound> {
		Context::with(|ctx| {
			let gl = &ctx.graphics.gl;
			unsafe {
				gl.use_program(Some(self.program));
				let location = gl
					.get_uniform_location(self.program, name)
					.ok_or_else(|| UniformNotFound(name.to_string()))?;
				gl.uniform_1_i32(Some(&location), value);
			}
			Ok(())
		})?;
		self.uniform_values
			.lock()
			.expect("uniform mutex poisoned")
			.insert(name.to_string(), UniformValue::I32(value));
		Ok(())
	}

	pub fn send_f32(&self, name: &str, value: f32) -> Result<(), UniformNotFound> {
		Context::with(|ctx| {
			let gl = &ctx.graphics.gl;
			unsafe {
				gl.use_program(Some(self.program));
				let location = gl
					.get_uniform_location(self.program, name)
					.ok_or_else(|| UniformNotFound(name.to_string()))?;
				gl.uniform_1_f32(Some(&location), value);
			}
			Ok(())
		})?;
		self.uniform_values
			.lock()
			.expect("uniform mutex poisoned")
			.insert(name.to_string(), UniformValue::F32(value));
		Ok(())
	}

	pub fn send_vec2(&self, name: &str, vec2: Vec2) -> Result<(), UniformNotFound> {
		Context::with(|ctx| {
			let gl = &ctx.graphics.gl;
			unsafe {
				gl.use_program(Some(self.program));
				let location = gl
					.get_uniform_location(self.program, name)
					.ok_or_else(|| UniformNotFound(name.to_string()))?;
				gl.uniform_2_f32(Some(&location), vec2.x, vec2.y);
			}
			Ok(())
		})?;
		self.uniform_values
			.lock()
			.expect("uniform mutex poisoned")
			.insert(name.to_string(), UniformValue::Vec2(vec2));

		Ok(())
	}

	pub fn send_vec3(&self, name: &str, vec3: Vec3) -> Result<(), UniformNotFound> {
		Context::with(|ctx| {
			let gl = &ctx.graphics.gl;
			unsafe {
				gl.use_program(Some(self.program));
				let location = gl
					.get_uniform_location(self.program, name)
					.ok_or_else(|| UniformNotFound(name.to_string()))?;
				gl.uniform_3_f32(Some(&location), vec3.x, vec3.y, vec3.z);
			}
			Ok(())
		})?;
		self.uniform_values
			.lock()
			.expect("uniform mutex poisoned")
			.insert(name.to_string(), UniformValue::Vec3(vec3));
		Ok(())
	}

	pub fn send_vec4(&self, name: &str, vec4: Vec4) -> Result<(), UniformNotFound> {
		Context::with(|ctx| {
			let gl = &ctx.graphics.gl;
			unsafe {
				gl.use_program(Some(self.program));
				let location = gl
					.get_uniform_location(self.program, name)
					.ok_or_else(|| UniformNotFound(name.to_string()))?;
				gl.uniform_4_f32(Some(&location), vec4.x, vec4.y, vec4.z, vec4.w);
			}
			Ok(())
		})?;
		self.uniform_values
			.lock()
			.expect("uniform mutex poisoned")
			.insert(name.to_string(), UniformValue::Vec4(vec4));
		Ok(())
	}

	pub fn send_mat3(&self, name: &str, mat3: Mat3) -> Result<(), UniformNotFound> {
		Context::with(|ctx| {
			let gl = &ctx.graphics.gl;
			unsafe {
				gl.use_program(Some(self.program));
				let location = gl
					.get_uniform_location(self.program, name)
					.ok_or_else(|| UniformNotFound(name.to_string()))?;
				gl.uniform_matrix_3_f32_slice(Some(&location), false, &mat3.to_cols_array());
			}
			Ok(())
		})?;
		self.uniform_values
			.lock()
			.expect("uniform mutex poisoned")
			.insert(name.to_string(), UniformValue::Mat3(mat3));
		Ok(())
	}

	pub fn send_mat4(&self, name: &str, mat4: Mat4) -> Result<(), UniformNotFound> {
		Context::with(|ctx| {
			let gl = &ctx.graphics.gl;
			unsafe {
				gl.use_program(Some(self.program));
				let location = gl
					.get_uniform_location(self.program, name)
					.ok_or_else(|| UniformNotFound(name.to_string()))?;
				gl.uniform_matrix_4_f32_slice(Some(&location), false, &mat4.to_cols_array());
			}
			Ok(())
		})?;
		self.uniform_values
			.lock()
			.expect("uniform mutex poisoned")
			.insert(name.to_string(), UniformValue::Mat4(mat4));
		Ok(())
	}

	pub fn send_color(&self, name: &str, color: LinSrgba) -> Result<(), UniformNotFound> {
		Context::with(|ctx| {
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
		})?;
		self.uniform_values
			.lock()
			.expect("uniform mutex poisoned")
			.insert(name.to_string(), UniformValue::Color(color));
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
		self.uniform_values
			.get_mut()
			.expect("uniform mutex poisoned")
			.insert(name.to_string(), UniformValue::Texture(texture.clone()));
		Ok(())
	}

	pub(crate) fn bind_sent_textures(&self) {
		Context::with(|ctx| {
			let gl = &ctx.graphics.gl;
			unsafe {
				for SentTextureInfo { texture, unit } in self.sent_textures.values() {
					gl.active_texture(glow::TEXTURE0 + *unit);
					gl.bind_texture(glow::TEXTURE_2D, Some(texture.inner.texture));
				}
				gl.active_texture(glow::TEXTURE0);
			}
		});
	}

	pub(crate) fn import_uniforms(&mut self, other: &Self) {
		for (name, value) in other
			.uniform_values
			.lock()
			.expect("uniform mutex poisoned")
			.iter()
		{
			match value {
				UniformValue::Bool(value) => self.send_bool(name, *value).ok(),
				UniformValue::I32(value) => self.send_i32(name, *value).ok(),
				UniformValue::F32(value) => self.send_f32(name, *value).ok(),
				UniformValue::Vec2(value) => self.send_vec2(name, *value).ok(),
				UniformValue::Vec3(value) => self.send_vec3(name, *value).ok(),
				UniformValue::Vec4(value) => self.send_vec4(name, *value).ok(),
				UniformValue::Mat3(value) => self.send_mat3(name, *value).ok(),
				UniformValue::Mat4(value) => self.send_mat4(name, *value).ok(),
				UniformValue::Color(value) => self.send_color(name, *value).ok(),
				UniformValue::Texture(value) => self.send_texture(name, value).ok(),
			};
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
pub enum UniformValue {
	Bool(bool),
	I32(i32),
	F32(f32),
	Vec2(Vec2),
	Vec3(Vec3),
	Vec4(Vec4),
	Mat3(Mat3),
	Mat4(Mat4),
	Color(LinSrgba),
	Texture(Texture),
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

#[derive(Debug, Clone)]
struct SentTextureInfo {
	pub texture: Texture,
	pub unit: u32,
}
