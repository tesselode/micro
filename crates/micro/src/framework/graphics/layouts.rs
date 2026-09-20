use std::collections::HashMap;

use wgpu::{
	BindGroupLayout, BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingType,
	BufferBindingType, Device, SamplerBindingType, ShaderStages, TextureSampleType,
	TextureViewDimension,
};

pub(crate) struct Layouts {
	pub(crate) mesh_bind_group_layouts: HashMap<TextureViewDimension, BindGroupLayout>,
	pub(crate) shader_params_bind_group_layout: BindGroupLayout,
}

impl Layouts {
	pub(crate) fn new(device: &Device) -> Self {
		let shader_params_bind_group_layout =
			device.create_bind_group_layout(&BindGroupLayoutDescriptor {
				label: Some("Shader Params Bind Group Layout"),
				entries: &[BindGroupLayoutEntry {
					binding: 0,
					visibility: ShaderStages::VERTEX_FRAGMENT,
					ty: BindingType::Buffer {
						ty: BufferBindingType::Uniform,
						has_dynamic_offset: false,
						min_binding_size: None,
					},
					count: None,
				}],
			});
		Self {
			mesh_bind_group_layouts: HashMap::new(),
			shader_params_bind_group_layout,
		}
	}

	pub(crate) fn mesh_bind_group_layout(
		&mut self,
		texture_view_dimension: TextureViewDimension,
		device: &Device,
	) -> BindGroupLayout {
		self.mesh_bind_group_layouts
			.entry(texture_view_dimension)
			.or_insert_with(|| {
				device.create_bind_group_layout(&BindGroupLayoutDescriptor {
					label: Some("Mesh Bind Group Layout"),
					entries: &[
						BindGroupLayoutEntry {
							binding: 0,
							visibility: ShaderStages::VERTEX,
							ty: BindingType::Buffer {
								ty: BufferBindingType::Uniform,
								has_dynamic_offset: false,
								min_binding_size: None,
							},
							count: None,
						},
						BindGroupLayoutEntry {
							binding: 1,
							visibility: ShaderStages::FRAGMENT,
							ty: BindingType::Texture {
								sample_type: TextureSampleType::Float { filterable: true },
								view_dimension: texture_view_dimension,
								multisampled: false,
							},
							count: None,
						},
						BindGroupLayoutEntry {
							binding: 2,
							visibility: ShaderStages::FRAGMENT,
							ty: BindingType::Sampler(SamplerBindingType::Filtering),
							count: None,
						},
					],
				})
			})
			.clone()
	}
}
