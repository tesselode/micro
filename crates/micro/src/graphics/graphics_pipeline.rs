mod builder;

pub use builder::*;
use bytemuck::Pod;

use std::{fmt::Debug, hash::Hash, marker::PhantomData};

use wgpu::{
	BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, Buffer, BufferAddress,
	BufferUsages, ColorTargetState, ColorWrites, CompareFunction, DepthBiasState,
	DepthStencilState, Device, FragmentState, MultisampleState, PipelineCompilationOptions,
	PipelineLayoutDescriptor, PrimitiveState, PrimitiveTopology, RenderPipeline,
	RenderPipelineDescriptor, TextureFormat, VertexBufferLayout, VertexState, VertexStepMode,
	util::{BufferInitDescriptor, DeviceExt},
};

use crate::Context;

use super::{
	DefaultShader, HasVertexAttributes, InstanceSettings, Instanced, NonInstanced, Shader,
	ShaderKind, drawable::Drawable,
};

pub struct GraphicsPipeline<S: Shader = DefaultShader> {
	pub label: String,
	pub(crate) render_pipeline: RenderPipeline,
	pub(crate) shader_params_buffer: Buffer,
	pub(crate) shader_params_bind_group: BindGroup,
	_shader: PhantomData<S>,
}

impl<S: Shader> GraphicsPipeline<S> {
	pub fn set_shader_params(&self, ctx: &Context, params: S::Params) {
		ctx.graphics.queue.write_buffer(
			&self.shader_params_buffer,
			0,
			bytemuck::cast_slice(&[params]),
		);
	}

	pub fn with_shader_params(&self, ctx: &Context, params: S::Params) -> Self {
		let device = &ctx.graphics.device;
		let shader_params_buffer = device.create_buffer_init(&BufferInitDescriptor {
			label: Some(&format!("{} - Shader Params Buffer", &self.label)),
			contents: bytemuck::cast_slice(&[params]),
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

	pub(crate) fn new_internal(
		device: &Device,
		mesh_bind_group_layout: &BindGroupLayout,
		shader_params_bind_group_layout: &BindGroupLayout,
		builder: GraphicsPipelineBuilder<S>,
	) -> Self {
		let shader = device.create_shader_module(S::DESCRIPTOR);
		let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
			label: Some(&format!("{} - Pipeline Layout", &builder.label)),
			bind_group_layouts: &[mesh_bind_group_layout, shader_params_bind_group_layout],
			push_constant_ranges: &[],
		});
		let shader_params_buffer = device.create_buffer_init(&BufferInitDescriptor {
			label: Some(&format!("{} - Shader Params Buffer", &builder.label)),
			contents: bytemuck::cast_slice(&[builder.shader_params]),
			usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
		});
		let shader_params_bind_group = device.create_bind_group(&BindGroupDescriptor {
			label: Some(&format!("{} - Shader Params Bind Group", &builder.label)),
			layout: shader_params_bind_group_layout,
			entries: &[BindGroupEntry {
				binding: 0,
				resource: shader_params_buffer.as_entire_binding(),
			}],
		});
		let render_pipeline = if let Some(InstanceSettings {
			array_stride,
			attributes,
		}) = S::Kind::instance_settings()
		{
			device.create_render_pipeline(&RenderPipelineDescriptor {
				label: Some(&builder.label),
				layout: Some(&pipeline_layout),
				vertex: VertexState {
					module: &shader,
					entry_point: Some("vs_main"),
					compilation_options: PipelineCompilationOptions::default(),
					buffers: &[
						VertexBufferLayout {
							array_stride: std::mem::size_of::<S::Vertex>() as BufferAddress,
							step_mode: VertexStepMode::Vertex,
							attributes: &S::Vertex::attributes(),
						},
						VertexBufferLayout {
							array_stride,
							step_mode: VertexStepMode::Instance,
							attributes: &attributes,
						},
					],
				},
				primitive: PrimitiveState {
					topology: PrimitiveTopology::TriangleList,
					..Default::default()
				},
				depth_stencil: Some(DepthStencilState {
					format: TextureFormat::Depth24PlusStencil8,
					depth_write_enabled: builder.enable_depth_testing,
					depth_compare: if builder.enable_depth_testing {
						CompareFunction::Less
					} else {
						CompareFunction::Always
					},
					stencil: builder.stencil_state.as_wgpu_stencil_state(),
					bias: DepthBiasState::default(),
				}),
				multisample: MultisampleState {
					count: builder.sample_count,
					..Default::default()
				},
				fragment: Some(FragmentState {
					module: &shader,
					entry_point: Some("fs_main"),
					compilation_options: PipelineCompilationOptions::default(),
					targets: &[Some(ColorTargetState {
						format: builder.format,
						blend: Some(builder.blend_mode.to_blend_state()),
						write_mask: if builder.enable_color_writes {
							ColorWrites::ALL
						} else {
							ColorWrites::empty()
						},
					})],
				}),
				multiview: None,
				cache: None,
			})
		} else {
			device.create_render_pipeline(&RenderPipelineDescriptor {
				label: Some(&builder.label),
				layout: Some(&pipeline_layout),
				vertex: VertexState {
					module: &shader,
					entry_point: Some("vs_main"),
					compilation_options: PipelineCompilationOptions::default(),
					buffers: &[VertexBufferLayout {
						array_stride: std::mem::size_of::<S::Vertex>() as BufferAddress,
						step_mode: VertexStepMode::Vertex,
						attributes: &S::Vertex::attributes(),
					}],
				},
				primitive: PrimitiveState {
					topology: PrimitiveTopology::TriangleList,
					..Default::default()
				},
				depth_stencil: Some(DepthStencilState {
					format: TextureFormat::Depth24PlusStencil8,
					depth_write_enabled: builder.enable_depth_testing,
					depth_compare: if builder.enable_depth_testing {
						CompareFunction::Less
					} else {
						CompareFunction::Always
					},
					stencil: builder.stencil_state.as_wgpu_stencil_state(),
					bias: DepthBiasState::default(),
				}),
				multisample: MultisampleState {
					count: builder.sample_count,
					..Default::default()
				},
				fragment: Some(FragmentState {
					module: &shader,
					entry_point: Some("fs_main"),
					compilation_options: PipelineCompilationOptions::default(),
					targets: &[Some(ColorTargetState {
						format: builder.format,
						blend: Some(builder.blend_mode.to_blend_state()),
						write_mask: if builder.enable_color_writes {
							ColorWrites::ALL
						} else {
							ColorWrites::empty()
						},
					})],
				}),
				multiview: None,
				cache: None,
			})
		};
		Self {
			label: builder.label.clone(),
			render_pipeline,
			shader_params_buffer,
			shader_params_bind_group,
			_shader: PhantomData,
		}
	}

	pub(crate) fn raw(&self) -> RawGraphicsPipeline {
		RawGraphicsPipeline {
			render_pipeline: self.render_pipeline.clone(),
			shader_params_bind_group: self.shader_params_bind_group.clone(),
		}
	}
}

impl<S: Shader<Kind = NonInstanced>> GraphicsPipeline<S> {
	pub fn draw(&self, ctx: &mut Context, drawable: impl Drawable<Vertex = S::Vertex>) {
		for settings in drawable.draw(ctx).into_iter() {
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
		drawable: impl Drawable<Vertex = S::Vertex>,
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
		for settings in drawable.draw(ctx).into_iter() {
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
			.field("render_pipeline", &self.render_pipeline)
			.field("shader_params_buffer", &self.shader_params_buffer)
			.field("shader_params_bind_group", &self.shader_params_bind_group)
			.field("_shader", &self._shader)
			.finish()
	}
}

impl<S: Shader> Clone for GraphicsPipeline<S> {
	fn clone(&self) -> Self {
		Self {
			label: self.label.clone(),
			render_pipeline: self.render_pipeline.clone(),
			shader_params_buffer: self.shader_params_buffer.clone(),
			shader_params_bind_group: self.shader_params_bind_group.clone(),
			_shader: self._shader,
		}
	}
}

impl<S: Shader> PartialEq for GraphicsPipeline<S> {
	fn eq(&self, other: &Self) -> bool {
		self.render_pipeline == other.render_pipeline
			&& self.shader_params_buffer == other.shader_params_buffer
			&& self.shader_params_bind_group == other.shader_params_bind_group
			&& self._shader == other._shader
	}
}

impl<S: Shader> Eq for GraphicsPipeline<S> {}

impl<S: Shader> Hash for GraphicsPipeline<S> {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		self.render_pipeline.hash(state);
		self.shader_params_buffer.hash(state);
		self.shader_params_bind_group.hash(state);
		self._shader.hash(state);
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct RawGraphicsPipeline {
	pub(crate) render_pipeline: RenderPipeline,
	pub(crate) shader_params_bind_group: BindGroup,
}
