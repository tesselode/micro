use std::{marker::PhantomData, rc::Rc};

use wgpu::{
	util::{BufferInitDescriptor, DeviceExt},
	BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, Buffer, BufferUsages,
	ColorTargetState, ColorWrites, CompareFunction, DepthBiasState, DepthStencilState, Device,
	FragmentState, MultisampleState, PipelineLayout, PrimitiveState, RenderPipeline,
	RenderPipelineDescriptor, ShaderModuleDescriptor, ShaderSource, SurfaceConfiguration,
	TextureFormat, VertexState,
};

use crate::Context;

use super::{
	mesh::Vertex,
	shader::{DefaultShader, Shader},
	stencil::StencilState,
	BlendMode,
};

#[derive(Clone)]
pub struct GraphicsPipeline<S: Shader = DefaultShader> {
	_phantom_data: PhantomData<S>,
	pub(crate) inner: Rc<GraphicsPipelineInner>,
}

impl<S: Shader> GraphicsPipeline<S> {
	pub fn new(ctx: &Context, settings: GraphicsPipelineSettings<S>) -> Self {
		Self::new_internal(
			settings,
			&ctx.graphics_ctx.device,
			&ctx.graphics_ctx.render_pipeline_layout,
			&ctx.graphics_ctx.shader_params_bind_group_layout,
			&ctx.graphics_ctx.config,
		)
	}

	pub fn set_shader_params(&self, ctx: &Context, shader_params: S::Params) {
		ctx.graphics_ctx.queue.write_buffer(
			&self.inner.shader_params_buffer,
			0,
			bytemuck::cast_slice(&[shader_params]),
		);
	}

	pub fn new_internal(
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
					blend: Some(settings.blend_mode.to_blend_state()),
					write_mask: if settings.enable_color_writes {
						ColorWrites::ALL
					} else {
						ColorWrites::empty()
					},
				})],
			}),
			primitive: PrimitiveState::default(),
			depth_stencil: Some(DepthStencilState {
				format: TextureFormat::Depth24PlusStencil8,
				depth_write_enabled: false,
				depth_compare: CompareFunction::Always,
				stencil: settings.stencil_state.to_wgpu_stencil_state(),
				bias: DepthBiasState::default(),
			}),
			multisample: MultisampleState {
				count: settings.sample_count,
				mask: !0,
				alpha_to_coverage_enabled: false,
			},
			multiview: None,
		});
		let shader_params_buffer = device.create_buffer_init(&BufferInitDescriptor {
			label: Some("Shader Params Buffer"),
			contents: bytemuck::cast_slice(&[settings.shader_params]),
			usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
		});
		let shader_params_bind_group = device.create_bind_group(&BindGroupDescriptor {
			label: Some("Shader Params Bind Group"),
			layout: shader_params_bind_group_layout,
			entries: &[BindGroupEntry {
				binding: 0,
				resource: shader_params_buffer.as_entire_binding(),
			}],
		});
		Self {
			_phantom_data: PhantomData,
			inner: Rc::new(GraphicsPipelineInner {
				shader_params_buffer,
				shader_params_bind_group,
				render_pipeline,
			}),
		}
	}
}

pub struct GraphicsPipelineSettings<S: Shader> {
	pub blend_mode: BlendMode,
	pub shader_params: S::Params,
	pub stencil_state: StencilState,
	pub enable_color_writes: bool,
	pub sample_count: u32,
}

impl<S: Shader> Default for GraphicsPipelineSettings<S>
where
	S::Params: Default,
{
	fn default() -> Self {
		Self {
			blend_mode: Default::default(),
			shader_params: Default::default(),
			stencil_state: Default::default(),
			enable_color_writes: true,
			sample_count: 1,
		}
	}
}

pub(crate) struct GraphicsPipelineInner {
	pub(crate) shader_params_buffer: Buffer,
	pub(crate) shader_params_bind_group: BindGroup,
	pub(crate) render_pipeline: RenderPipeline,
}
