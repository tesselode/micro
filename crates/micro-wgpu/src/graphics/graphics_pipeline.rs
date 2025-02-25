use std::marker::PhantomData;

use wgpu::{
	BindGroupLayoutDescriptor, BufferAddress, ColorTargetState, ColorWrites, Device, FragmentState,
	MultisampleState, PipelineCompilationOptions, PipelineLayoutDescriptor, PrimitiveState,
	PrimitiveTopology, RenderPipeline, RenderPipelineDescriptor, TextureFormat, VertexBufferLayout,
	VertexState, VertexStepMode, include_wgsl,
};

use crate::Context;

use super::{Vertex, Vertex2d};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GraphicsPipeline<V: Vertex = Vertex2d> {
	pub(crate) render_pipeline: RenderPipeline,
	_phantom_data: PhantomData<V>,
}

impl<V: Vertex> GraphicsPipeline<V> {
	pub fn new(ctx: &mut Context) -> Self {
		Self::new_from_device(&ctx.graphics.device)
	}

	pub(crate) fn new_from_device(device: &Device) -> Self {
		let shader = device.create_shader_module(include_wgsl!("shader.wgsl"));
		let bind_group_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
			label: None,
			entries: &[],
		});
		let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
			label: None,
			bind_group_layouts: &[&bind_group_layout],
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
				multisample: MultisampleState::default(),
				fragment: Some(FragmentState {
					module: &shader,
					entry_point: Some("fs_main"),
					compilation_options: PipelineCompilationOptions::default(),
					targets: &[Some(ColorTargetState {
						format: TextureFormat::Rgba8UnormSrgb,
						blend: None,
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
