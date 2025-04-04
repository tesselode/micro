mod builder;

pub use builder::*;
use bytemuck::Pod;

use std::{fmt::Debug, hash::Hash, marker::PhantomData};

use wgpu::{
	BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, Buffer, BufferAddress,
	BufferUsages, ColorTargetState, ColorWrites, CompareFunction, DepthBiasState,
	DepthStencilState, Device, FragmentState, MultisampleState, PipelineCompilationOptions,
	PipelineLayoutDescriptor, PrimitiveState, PrimitiveTopology, RenderPipeline,
	RenderPipelineDescriptor, ShaderModuleDescriptor, TextureFormat, VertexAttribute,
	VertexBufferLayout, VertexState, VertexStepMode,
	util::{BufferInitDescriptor, DeviceExt},
};

use crate::Context;

use super::{
	BlendMode, DefaultShader, HasVertexAttributes, InstanceSettings, Instanced, NonInstanced,
	Shader, ShaderKind, StencilState, drawable::Drawable,
};

pub struct GraphicsPipeline<S: Shader = DefaultShader> {
	raw: RawGraphicsPipeline,
	_shader: PhantomData<S>,
}

impl<S: Shader> GraphicsPipeline<S> {
	pub fn set_shader_params(&self, ctx: &Context, params: S::Params) {
		ctx.graphics.queue.write_buffer(
			&self.raw.shader_params_buffer,
			0,
			bytemuck::cast_slice(&[params]),
		);
	}

	pub fn with_shader_params(&self, ctx: &Context, params: S::Params) -> Self {
		Self {
			raw: self
				.raw
				.with_shader_params(ctx, bytemuck::cast_slice(&[params])),
			..self.clone()
		}
	}

	pub(crate) fn new_internal(
		device: &Device,
		mesh_bind_group_layout: &BindGroupLayout,
		shader_params_bind_group_layout: &BindGroupLayout,
		builder: GraphicsPipelineBuilder<S>,
	) -> Self {
		Self {
			raw: RawGraphicsPipeline::new(
				device,
				mesh_bind_group_layout,
				shader_params_bind_group_layout,
				RawGraphicsPipelineSettings {
					label: builder.label,
					blend_mode: builder.blend_mode,
					shader_module_descriptor: S::DESCRIPTOR,
					shader_params: bytemuck::cast_slice(&[builder.shader_params]),
					vertex_size: std::mem::size_of::<S::Vertex>(),
					vertex_attributes: S::Vertex::attributes(),
					instance_settings: S::Kind::instance_settings(),
					enable_depth_testing: builder.enable_depth_testing,
					stencil_state: builder.stencil_state,
					enable_color_writes: builder.enable_color_writes,
					sample_count: builder.sample_count,
					format: builder.format,
				},
			),
			_shader: PhantomData,
		}
	}

	pub(crate) fn raw(&self) -> RawGraphicsPipeline {
		self.raw.clone()
	}
}

impl<S: Shader<Kind = NonInstanced>> GraphicsPipeline<S> {
	pub fn draw(&self, ctx: &mut Context, drawable: &impl Drawable<Vertex = S::Vertex>) {
		for settings in drawable.draw_instructions(ctx).into_iter() {
			ctx.graphics
				.queue_draw_command(settings, self.raw(), 1, None);
		}
	}
}

impl<I, S> GraphicsPipeline<S>
where
	I: Pod + HasVertexAttributes,
	S: Shader<Kind = Instanced<I>>,
{
	pub fn draw_instanced(
		&self,
		ctx: &mut Context,
		drawable: &impl Drawable<Vertex = S::Vertex>,
		instances: &[I],
	) {
		let instance_buffer = Some(
			ctx.graphics
				.device
				.create_buffer_init(&BufferInitDescriptor {
					label: None,
					contents: bytemuck::cast_slice(instances),
					usage: BufferUsages::VERTEX,
				}),
		);
		for settings in drawable.draw_instructions(ctx).into_iter() {
			ctx.graphics.queue_draw_command(
				settings,
				self.raw(),
				instances.len() as u32,
				instance_buffer.clone(),
			);
		}
	}
}

impl<S: Shader> Debug for GraphicsPipeline<S> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("GraphicsPipeline")
			.field("raw", &self.raw)
			.field("_shader", &self._shader)
			.finish()
	}
}

impl<S: Shader> Clone for GraphicsPipeline<S> {
	fn clone(&self) -> Self {
		Self {
			raw: self.raw.clone(),
			_shader: self._shader,
		}
	}
}

impl<S: Shader> PartialEq for GraphicsPipeline<S> {
	fn eq(&self, other: &Self) -> bool {
		self.raw == other.raw && self._shader == other._shader
	}
}

impl<S: Shader> Eq for GraphicsPipeline<S> {}

impl<S: Shader> Hash for GraphicsPipeline<S> {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		self.raw.hash(state);
		self._shader.hash(state);
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct RawGraphicsPipeline {
	pub label: String,
	pub render_pipeline: RenderPipeline,
	pub shader_params_buffer: Buffer,
	pub shader_params_bind_group: BindGroup,
}

impl RawGraphicsPipeline {
	fn new(
		device: &Device,
		mesh_bind_group_layout: &BindGroupLayout,
		shader_params_bind_group_layout: &BindGroupLayout,
		settings: RawGraphicsPipelineSettings,
	) -> Self {
		let shader = device.create_shader_module(settings.shader_module_descriptor);
		let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
			label: Some(&format!("{} - Pipeline Layout", &settings.label)),
			bind_group_layouts: &[mesh_bind_group_layout, shader_params_bind_group_layout],
			push_constant_ranges: &[],
		});
		let shader_params_buffer = device.create_buffer_init(&BufferInitDescriptor {
			label: Some(&format!("{} - Shader Params Buffer", &settings.label)),
			contents: settings.shader_params,
			usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
		});
		let shader_params_bind_group = device.create_bind_group(&BindGroupDescriptor {
			label: Some(&format!("{} - Shader Params Bind Group", &settings.label)),
			layout: shader_params_bind_group_layout,
			entries: &[BindGroupEntry {
				binding: 0,
				resource: shader_params_buffer.as_entire_binding(),
			}],
		});
		let vertex_attributes = settings.vertex_attributes;
		let instance_attributes = settings
			.instance_settings
			.as_ref()
			.map(|instance_settings| instance_settings.attributes.clone())
			.unwrap_or_default();
		let mut vertex_buffers = vec![VertexBufferLayout {
			array_stride: settings.vertex_size as BufferAddress,
			step_mode: VertexStepMode::Vertex,
			attributes: &vertex_attributes,
		}];
		if let Some(InstanceSettings { array_stride, .. }) = settings.instance_settings {
			vertex_buffers.push(VertexBufferLayout {
				array_stride,
				step_mode: VertexStepMode::Instance,
				attributes: &instance_attributes,
			});
		}
		let render_pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
			label: Some(&settings.label),
			layout: Some(&pipeline_layout),
			vertex: VertexState {
				module: &shader,
				entry_point: Some("vs_main"),
				compilation_options: PipelineCompilationOptions::default(),
				buffers: &vertex_buffers,
			},
			primitive: PrimitiveState {
				topology: PrimitiveTopology::TriangleList,
				..Default::default()
			},
			depth_stencil: Some(DepthStencilState {
				format: TextureFormat::Depth24PlusStencil8,
				depth_write_enabled: settings.enable_depth_testing,
				depth_compare: if settings.enable_depth_testing {
					CompareFunction::Less
				} else {
					CompareFunction::Always
				},
				stencil: settings.stencil_state.as_wgpu_stencil_state(),
				bias: DepthBiasState::default(),
			}),
			multisample: MultisampleState {
				count: settings.sample_count,
				..Default::default()
			},
			fragment: Some(FragmentState {
				module: &shader,
				entry_point: Some("fs_main"),
				compilation_options: PipelineCompilationOptions::default(),
				targets: &[Some(ColorTargetState {
					format: settings.format,
					blend: Some(settings.blend_mode.to_blend_state()),
					write_mask: if settings.enable_color_writes {
						ColorWrites::ALL
					} else {
						ColorWrites::empty()
					},
				})],
			}),
			multiview: None,
			cache: None,
		});
		Self {
			label: settings.label.clone(),
			render_pipeline,
			shader_params_buffer,
			shader_params_bind_group,
		}
	}

	fn with_shader_params(&self, ctx: &Context, params: &[u8]) -> Self {
		let device = &ctx.graphics.device;
		let shader_params_buffer = device.create_buffer_init(&BufferInitDescriptor {
			label: Some(&format!("{} - Shader Params Buffer", &self.label)),
			contents: params,
			usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
		});
		let shader_params_bind_group = device.create_bind_group(&BindGroupDescriptor {
			label: Some(&format!("{} - Shader Params Bind Group", &self.label)),
			layout: &ctx.graphics.shader_params_bind_group_layout,
			entries: &[BindGroupEntry {
				binding: 0,
				resource: shader_params_buffer.as_entire_binding(),
			}],
		});
		Self {
			shader_params_buffer,
			shader_params_bind_group,
			..self.clone()
		}
	}
}

struct RawGraphicsPipelineSettings<'a> {
	label: String,
	blend_mode: BlendMode,
	shader_module_descriptor: ShaderModuleDescriptor<'a>,
	shader_params: &'a [u8],
	vertex_size: usize,
	vertex_attributes: Vec<VertexAttribute>,
	instance_settings: Option<InstanceSettings>,
	enable_depth_testing: bool,
	stencil_state: StencilState,
	enable_color_writes: bool,
	sample_count: u32,
	format: TextureFormat,
}
