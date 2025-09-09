use wgpu::{
	BindGroupLayout, BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingType,
	BufferBindingType, Device, PipelineLayout, PipelineLayoutDescriptor, SamplerBindingType,
	ShaderStages, TextureSampleType, TextureViewDimension,
};

pub(crate) struct Layouts {
	pub(crate) mesh_bind_group_layout: BindGroupLayout,
	pub(crate) shader_params_bind_group_layout: BindGroupLayout,
	pub(crate) pipeline_layout: PipelineLayout,
}

impl Layouts {
	pub(crate) fn new(device: &Device) -> Self {
		let mesh_bind_group_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
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
						view_dimension: TextureViewDimension::D2,
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
		});
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
		let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
			bind_group_layouts: &[&mesh_bind_group_layout, &shader_params_bind_group_layout],
			..Default::default()
		});
		Self {
			mesh_bind_group_layout,
			shader_params_bind_group_layout,
			pipeline_layout,
		}
	}
}
