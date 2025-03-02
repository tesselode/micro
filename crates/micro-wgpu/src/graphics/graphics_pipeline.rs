use std::marker::PhantomData;

use wgpu::{
	BindGroupLayout, BufferAddress, ColorTargetState, ColorWrites, Device, FragmentState,
	MultisampleState, PipelineCompilationOptions, PipelineLayoutDescriptor, PrimitiveState,
	PrimitiveTopology, RenderPipeline, RenderPipelineDescriptor, TextureFormat, VertexBufferLayout,
	VertexState, VertexStepMode, include_wgsl,
};

use crate::Context;

use super::{BlendMode, Vertex, Vertex2d};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GraphicsPipeline<V: Vertex = Vertex2d> {
	pub(crate) render_pipeline: RenderPipeline,
	_phantom_data: PhantomData<V>,
}

impl<V: Vertex> GraphicsPipeline<V> {
	pub fn new(ctx: &mut Context, settings: GraphicsPipelineSettings) -> Self {
		Self::new_internal(
			&ctx.graphics.device,
			&ctx.graphics.mesh_bind_group_layout,
			settings,
		)
	}

	pub(crate) fn new_internal(
		device: &Device,
		transform_bind_group_layout: &BindGroupLayout,
		settings: GraphicsPipelineSettings,
	) -> Self {
		let shader = device.create_shader_module(include_wgsl!("shader.wgsl"));
		let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
			label: None,
			bind_group_layouts: &[transform_bind_group_layout],
			push_constant_ranges: &[],
		});
		Self {
			render_pipeline: device.create_render_pipeline(&RenderPipelineDescriptor {
				label: None,
				layout: Some(&pipeline_layout),
				vertex: VertexState {
					module: &shader,
					entry_point: Some("vs_main"),
					compilation_options: PipelineCompilationOptions::default(),
					buffers: &[VertexBufferLayout {
						array_stride: std::mem::size_of::<V>() as BufferAddress,
						step_mode: VertexStepMode::Vertex,
						attributes: &V::attributes(),
					}],
				},
				primitive: PrimitiveState {
					topology: PrimitiveTopology::TriangleList,
					..Default::default()
				},
				depth_stencil: None,
				multisample: MultisampleState {
					count: settings.sample_count,
					..Default::default()
				},
				fragment: Some(FragmentState {
					module: &shader,
					entry_point: Some("fs_main"),
					compilation_options: PipelineCompilationOptions::default(),
					targets: &[Some(ColorTargetState {
						format: TextureFormat::Rgba8UnormSrgb,
						blend: Some(settings.blend_mode.to_blend_state()),
						write_mask: ColorWrites::ALL,
					})],
				}),
				multiview: None,
				cache: None,
			}),
			_phantom_data: PhantomData,
		}
	}
}

pub struct GraphicsPipelineSettings {
	// pub label: String,
	pub blend_mode: BlendMode,
	// pub shader_params: S::Params,
	// pub stencil_state: StencilState,
	// pub enable_color_writes: bool,
	pub sample_count: u32,
	// pub textures: Vec<MeshTexture>,
}

impl Default for GraphicsPipelineSettings {
	fn default() -> Self {
		Self {
			// label: "Graphics Pipeline".into(),
			blend_mode: Default::default(),
			// shader_params: Default::default(),
			// stencil_state: Default::default(),
			// enable_color_writes: true,
			sample_count: 1,
			// textures: vec![],
		}
	}
}
