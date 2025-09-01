use std::{borrow::Cow, path::Path};

use bytemuck::Pod;
use wgpu::{
	BindGroup, BindGroupDescriptor, BindGroupEntry, BufferUsages, Device, ShaderModule,
	ShaderModuleDescriptor, ShaderSource,
	naga::ShaderStage,
	util::{BufferInitDescriptor, DeviceExt},
};

use crate::Context;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Shader {
	name: String,
	pub(crate) vertex: ShaderModule,
	pub(crate) fragment: ShaderModule,
	pub(crate) params_bind_group: Option<BindGroup>,
}

impl Shader {
	pub fn from_file(
		ctx: &Context,
		name: impl Into<String>,
		path: impl AsRef<Path>,
	) -> std::io::Result<Self> {
		let source = std::fs::read_to_string(path.as_ref())?;
		Ok(Self::from_string(ctx, name, &source))
	}

	pub fn from_string(ctx: &Context, name: impl Into<String>, source: &str) -> Self {
		Self::new_internal(&ctx.graphics.device, name, source)
	}

	pub fn with_params(&self, ctx: &Context, params: impl Pod) -> Self {
		let buffer = ctx
			.graphics
			.device
			.create_buffer_init(&BufferInitDescriptor {
				label: Some(&format!("{} - Shader Params Buffer", self.name)),
				contents: bytemuck::cast_slice(&[params]),
				usage: BufferUsages::UNIFORM,
			});
		let params_bind_group = ctx.graphics.device.create_bind_group(&BindGroupDescriptor {
			label: Some(&format!("{} - Shader Params Bind Group", self.name)),
			layout: &ctx.graphics.shader_params_bind_group_layout,
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

	pub fn set_params(&mut self, ctx: &Context, params: impl Pod) {
		*self = self.with_params(ctx, params);
	}

	pub(crate) fn new_internal(device: &Device, name: impl Into<String>, source: &str) -> Self {
		let name = name.into();
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
		Self {
			name,
			vertex,
			fragment,
			params_bind_group: None,
		}
	}
}
