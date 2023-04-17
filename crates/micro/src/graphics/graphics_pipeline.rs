use std::rc::Rc;

use wgpu::{
	util::{BufferInitDescriptor, DeviceExt},
	BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BlendState, Buffer,
	BufferUsages, ColorTargetState, ColorWrites, Device, FragmentState, MultisampleState,
	PipelineLayout, PrimitiveState, RenderPipeline, RenderPipelineDescriptor, ShaderModule,
	ShaderModuleDescriptor, ShaderSource, SurfaceConfiguration, VertexState,
};

use crate::Context;

use super::{mesh::Vertex, shader::Shader};

#[derive(Clone)]
pub struct GraphicsPipeline(pub(crate) Rc<GraphicsPipelineInner>);

impl GraphicsPipeline {
	pub fn new<S: Shader>(ctx: &Context, settings: GraphicsPipelineSettings<S>) -> Self {
		Self::new_internal(
			settings,
			&ctx.graphics_ctx.device,
			&ctx.graphics_ctx.render_pipeline_layout,
			&ctx.graphics_ctx.shader_params_bind_group_layout,
			&ctx.graphics_ctx.config,
		)
	}

	pub fn set_shader_params<S: Shader>(&self, ctx: &Context, shader_params: S::Params) {
		ctx.graphics_ctx.queue.write_buffer(
			&self.0.shader_params_buffer,
			0,
			bytemuck::cast_slice(&[shader_params]),
		);
	}

	pub fn new_internal<S: Shader>(
		settings: GraphicsPipelineSettings<S>,
		device: &Device,
		render_pipeline_layout: &PipelineLayout,
		shader_params_bind_group_layout: &BindGroupLayout,
		config: &SurfaceConfiguration,
	) -> Self {
		let shader_module = device.create_shader_module(ShaderModuleDescriptor {
			label: Some("Shader module"),
			source: ShaderSource::Wgsl(S::SOURCE.into()),
		});
		let render_pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
			label: Some("Render Pipeline"),
			layout: Some(render_pipeline_layout),
			vertex: VertexState {
				module: &shader_module,
				entry_point: "vs_main",
				buffers: &[Vertex::buffer_layout()],
			},
			fragment: Some(FragmentState {
				module: &shader_module,
				entry_point: "fs_main",
				targets: &[Some(ColorTargetState {
					format: config.format,
					blend: Some(BlendState::ALPHA_BLENDING),
					write_mask: ColorWrites::ALL,
				})],
			}),
			primitive: PrimitiveState::default(),
			depth_stencil: None,
			multisample: MultisampleState::default(),
			multiview: None,
		});
		let shader_params_buffer = device.create_buffer_init(&BufferInitDescriptor {
			label: Some("Shader Params Buffer"),
			contents: bytemuck::cast_slice(&[settings.shader_params]),
			usage: BufferUsages::UNIFORM,
		});
		let shader_params_bind_group = device.create_bind_group(&BindGroupDescriptor {
			label: Some("Shader Params Bind Group"),
			layout: shader_params_bind_group_layout,
			entries: &[BindGroupEntry {
				binding: 0,
				resource: shader_params_buffer.as_entire_binding(),
			}],
		});
		Self(Rc::new(GraphicsPipelineInner {
			shader_module,
			shader_params_buffer,
			shader_params_bind_group,
			render_pipeline,
		}))
	}
}

#[derive(Default)]
pub struct GraphicsPipelineSettings<S: Shader> {
	pub shader_params: S::Params,
}

pub(crate) struct GraphicsPipelineInner {
	pub(crate) shader_module: ShaderModule,
	pub(crate) shader_params_buffer: Buffer,
	pub(crate) shader_params_bind_group: BindGroup,
	pub(crate) render_pipeline: RenderPipeline,
}
