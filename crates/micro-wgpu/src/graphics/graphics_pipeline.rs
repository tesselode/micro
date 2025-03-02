use std::{borrow::Cow, marker::PhantomData};

use wgpu::{
	BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, Buffer, BufferAddress,
	BufferUsages, ColorTargetState, ColorWrites, Device, FragmentState, MultisampleState,
	PipelineCompilationOptions, PipelineLayoutDescriptor, PrimitiveState, PrimitiveTopology,
	RenderPipeline, RenderPipelineDescriptor, ShaderModuleDescriptor, ShaderSource, TextureFormat,
	VertexBufferLayout, VertexState, VertexStepMode,
	util::{BufferInitDescriptor, DeviceExt},
};

use crate::Context;

use super::{BlendMode, DefaultShader, Shader, Vertex, Vertex2d};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GraphicsPipeline<S = DefaultShader, V = Vertex2d>
where
	V: Vertex,
	S: Shader<Vertex = V>,
{
	pub(crate) render_pipeline: RenderPipeline,
	pub(crate) shader_params_buffer: Buffer,
	pub(crate) shader_params_bind_group: BindGroup,
	_vertex: PhantomData<V>,
	_shader: PhantomData<S>,
}

impl<S, V> GraphicsPipeline<S, V>
where
	V: Vertex,
	S: Shader<Vertex = V>,
{
	pub fn new(ctx: &Context, settings: GraphicsPipelineSettings<S>) -> Self {
		Self::new_internal(
			&ctx.graphics.device,
			&ctx.graphics.mesh_bind_group_layout,
			&ctx.graphics.shader_params_bind_group_layout,
			settings,
		)
	}

	pub fn set_shader_params(&self, ctx: &Context, params: S::Params) {
		ctx.graphics.queue.write_buffer(
			&self.shader_params_buffer,
			0,
			bytemuck::cast_slice(&[params]),
		);
	}

	pub(crate) fn new_internal(
		device: &Device,
		mesh_bind_group_layout: &BindGroupLayout,
		shader_params_bind_group_layout: &BindGroupLayout,
		settings: GraphicsPipelineSettings<S>,
	) -> Self {
		let shader = device.create_shader_module(ShaderModuleDescriptor {
			label: None,
			source: ShaderSource::Wgsl(Cow::Borrowed(S::SOURCE)),
		});
		let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
			label: None,
			bind_group_layouts: &[mesh_bind_group_layout, shader_params_bind_group_layout],
			push_constant_ranges: &[],
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
			shader_params_buffer,
			shader_params_bind_group,
			_vertex: PhantomData,
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

pub struct GraphicsPipelineSettings<S: Shader> {
	// pub label: String,
	pub blend_mode: BlendMode,
	pub shader_params: S::Params,
	// pub stencil_state: StencilState,
	// pub enable_color_writes: bool,
	pub sample_count: u32,
	// pub textures: Vec<MeshTexture>,
}

impl<S: Shader> Default for GraphicsPipelineSettings<S>
where
	S::Params: Default,
{
	fn default() -> Self {
		Self {
			// label: "Graphics Pipeline".into(),
			blend_mode: Default::default(),
			shader_params: Default::default(),
			// stencil_state: Default::default(),
			// enable_color_writes: true,
			sample_count: 1,
			// textures: vec![],
		}
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct RawGraphicsPipeline {
	pub(crate) render_pipeline: RenderPipeline,
	pub(crate) shader_params_bind_group: BindGroup,
}
