use glam::UVec2;
use wgpu::{
	BindGroup, BindGroupDescriptor, BindGroupEntry, BufferUsages, Device, Queue,
	util::{BufferInitDescriptor, DeviceExt},
};

use crate::{
	context::graphics::Layouts,
	graphics::{
		Shader,
		texture::{InternalTextureSettings, Texture, TextureSettings},
	},
};

const DEFAULT_SHADER_SOURCE: &str = include_str!("shader.glsl");

pub(crate) struct DefaultResources {
	pub(crate) default_texture: Texture,
	pub(crate) default_shader: Shader,
	pub(crate) default_shader_params_bind_group: BindGroup,
}

impl DefaultResources {
	pub(crate) fn new(device: &Device, queue: &Queue, layouts: &Layouts) -> Self {
		let default_texture = Texture::new(
			device,
			queue,
			UVec2::new(1, 1),
			Some(&[255, 255, 255, 255]),
			TextureSettings::default(),
			InternalTextureSettings::default(),
		);
		let default_shader = Shader::from_string("Default Shader", DEFAULT_SHADER_SOURCE);
		let default_shader_params_bind_group = {
			let buffer = device.create_buffer_init(&BufferInitDescriptor {
				label: Some("Default Shader - Shader Params Buffer"),
				contents: bytemuck::cast_slice(&[0]),
				usage: BufferUsages::UNIFORM,
			});
			device.create_bind_group(&BindGroupDescriptor {
				label: Some("Default Shader - Shader Params Bind Group"),
				layout: &layouts.shader_params_bind_group_layout,
				entries: &[BindGroupEntry {
					binding: 0,
					resource: buffer.as_entire_binding(),
				}],
			})
		};
		Self {
			default_texture,
			default_shader,
			default_shader_params_bind_group,
		}
	}
}
