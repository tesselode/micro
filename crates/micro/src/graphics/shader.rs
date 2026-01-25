use std::{borrow::Cow, collections::HashMap, path::Path};

use bytemuck::Pod;
use derive_more::{Display, Error, From};
use wgpu::{
	BindGroup, BindGroupDescriptor, BindGroupEntry, BufferUsages, Device, ErrorFilter,
	ShaderModule, ShaderModuleDescriptor, ShaderSource,
	naga::ShaderStage,
	util::{BufferInitDescriptor, DeviceExt},
};

use crate::{
	Context,
	graphics::{storage_buffer::StorageBuffer, texture::Texture},
};

/// A shader program that can be used to draw
/// [`Mesh`](crate::graphics::mesh::Mesh)es on the GPU.
#[derive(Debug, Clone, PartialEq)]
pub struct Shader {
	pub(crate) name: String,
	pub(crate) source: String,
	pub(crate) params_bind_group: Option<BindGroup>,
	pub(crate) storage_buffers: Vec<StorageBuffer>,
	pub(crate) textures: Vec<Texture>,
}

impl Shader {
	/// Loads a shader from a file.
	pub fn from_file(
		ctx: &mut Context,
		name: impl Into<String>,
		path: impl AsRef<Path>,
	) -> Result<Shader, LoadShaderError> {
		let source = std::fs::read_to_string(path.as_ref())?;
		Ok(Self::from_string(ctx, name, &source)?)
	}

	/// Loads a shader from a string.
	pub fn from_string(
		ctx: &mut Context,
		name: impl Into<String>,
		source: impl Into<String>,
	) -> Result<Self, wgpu::Error> {
		Self::new(
			name,
			source,
			&ctx.graphics.device,
			&mut ctx.graphics.compiled_shaders,
		)
	}

	/// Returns a clone of this shader with the specified source code.
	pub fn with_source(&mut self, ctx: &mut Context, source: String) -> Result<Self, wgpu::Error> {
		let compiled = CompiledShader::new(&ctx.graphics.device, &self.name, &source)?;
		ctx.graphics
			.compiled_shaders
			.insert(source.clone(), compiled);
		Ok(Self {
			source,
			..self.clone()
		})
	}

	/// Returns a clone of this shader with the specified set of uniform values.
	pub fn with_params(&self, ctx: &Context, params: impl Pod) -> Self {
		let buffer = ctx
			.graphics
			.device
			.create_buffer_init(&BufferInitDescriptor {
				label: Some(&format!("{} - Shader Params Buffer", &self.name)),
				contents: bytemuck::cast_slice(&[params]),
				usage: BufferUsages::UNIFORM,
			});
		let params_bind_group = ctx.graphics.device.create_bind_group(&BindGroupDescriptor {
			label: Some(&format!("{} - Shader Params Bind Group", &self.name)),
			layout: &ctx.graphics.layouts.shader_params_bind_group_layout,
			entries: &[BindGroupEntry {
				binding: 0,
				resource: buffer.as_entire_binding(),
			}],
		});
		Self {
			params_bind_group: Some(params_bind_group),
			..self.clone()
		}
	}

	/// Sets the uniforms to be used with this shader.
	pub fn set_params(&mut self, ctx: &Context, params: impl Pod) {
		*self = self.with_params(ctx, params);
	}

	/// Returns a clone of this shader with the specified set of storage buffers.
	pub fn with_storage_buffers(&self, buffers: Vec<StorageBuffer>) -> Self {
		Self {
			storage_buffers: buffers,
			..self.clone()
		}
	}

	/// Sets the storage buffers to be used with this shader.
	pub fn set_storage_buffers(&mut self, buffers: Vec<StorageBuffer>) {
		*self = self.with_storage_buffers(buffers);
	}

	/// Returns a clone of this shader with the specified set of textures.
	pub fn with_textures(&self, textures: Vec<Texture>) -> Self {
		Self {
			textures,
			..self.clone()
		}
	}

	/// Sets the textures to be used with this shader.
	pub fn set_textures(&mut self, textures: Vec<Texture>) {
		*self = self.with_textures(textures);
	}

	pub(crate) fn new(
		name: impl Into<String>,
		source: impl Into<String>,
		device: &Device,
		compiled_shaders: &mut HashMap<String, CompiledShader>,
	) -> Result<Self, wgpu::Error> {
		let name = name.into();
		let source = source.into();
		let compiled = CompiledShader::new(device, &name, &source)?;
		compiled_shaders.insert(source.clone(), compiled);
		Ok(Self {
			name,
			source,
			params_bind_group: None,
			storage_buffers: vec![],
			textures: vec![],
		})
	}
}

#[derive(Debug, Error, Display, From)]
pub enum LoadShaderError {
	IoError(std::io::Error),
	WgpuError(wgpu::Error),
}

pub(crate) struct CompiledShader {
	pub(crate) vertex: ShaderModule,
	pub(crate) fragment: ShaderModule,
}

impl CompiledShader {
	fn new(device: &Device, name: &str, source: &str) -> Result<Self, wgpu::Error> {
		let span = tracy_client::span!();
		span.emit_text(name);
		let error_scope = device.push_error_scope(ErrorFilter::Validation);
		let vertex = device.create_shader_module(ShaderModuleDescriptor {
			label: Some(&format!("{} - Vertex Shader", &name)),
			source: ShaderSource::Glsl {
				shader: Cow::Borrowed(source),
				stage: ShaderStage::Vertex,
				defines: &[("VERTEX", "1")],
			},
		});
		let fragment = device.create_shader_module(ShaderModuleDescriptor {
			label: Some(&format!("{} - Fragment Shader", &name)),
			source: ShaderSource::Glsl {
				shader: Cow::Borrowed(source),
				stage: ShaderStage::Fragment,
				defines: &[("FRAGMENT", "1")],
			},
		});
		if let Some(error) = pollster::block_on(error_scope.pop()) {
			return Err(error);
		}
		Ok(Self { vertex, fragment })
	}
}
