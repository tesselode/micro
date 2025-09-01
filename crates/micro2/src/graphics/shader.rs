use std::{borrow::Cow, path::Path};

use wgpu::{Device, ShaderModule, ShaderModuleDescriptor, ShaderSource, naga::ShaderStage};

use crate::Context;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Shader {
	name: String,
	pub(crate) vertex: ShaderModule,
	pub(crate) fragment: ShaderModule,
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
		}
	}
}
