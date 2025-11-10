use std::path::Path;

use bytemuck::Pod;
use wgpu::{
	BindGroup, BindGroupDescriptor, BindGroupEntry, BufferUsages,
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
	pub fn from_file(name: impl Into<String>, path: impl AsRef<Path>) -> std::io::Result<Self> {
		let source = std::fs::read_to_string(path.as_ref())?;
		Ok(Self::from_string(name, &source))
	}

	/// Replaces the source code of this shader.
	pub fn set_source(&mut self, source: String) {
		self.source = source;
	}

	/// Loads a shader from a string.
	pub fn from_string(name: impl Into<String>, source: impl Into<String>) -> Self {
		Self {
			name: name.into(),
			source: source.into(),
			params_bind_group: None,
			storage_buffers: vec![],
			textures: vec![],
		}
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
}
