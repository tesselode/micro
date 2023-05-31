use std::{marker::PhantomData, rc::Rc};

use wgpu::{
	util::{BufferInitDescriptor, DeviceExt},
	BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor,
	BindGroupLayoutEntry, BindingResource, BindingType, Buffer, BufferBindingType, BufferUsages,
	ColorTargetState, ColorWrites, CompareFunction, DepthBiasState, DepthStencilState, Device,
	FragmentState, MultisampleState, PipelineLayoutDescriptor, PrimitiveState, RenderPipeline,
	RenderPipelineDescriptor, SamplerBindingType, ShaderModuleDescriptor, ShaderSource,
	ShaderStages, SurfaceConfiguration, TextureFormat, TextureSampleType, TextureViewDimension,
	VertexState,
};

use crate::Context;

use super::{
	mesh::{MeshTexture, Vertex},
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
			&ctx.graphics_ctx.texture_bind_group_layout,
			&ctx.graphics_ctx.draw_params_bind_group_layout,
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
		texture_bind_group_layout: &BindGroupLayout,
		draw_params_bind_group_layout: &BindGroupLayout,
		config: &SurfaceConfiguration,
	) -> Self {
		let shader_module = device.create_shader_module(ShaderModuleDescriptor {
			label: Some("Shader module"),
			source: ShaderSource::Wgsl(S::SOURCE.into()),
		});
		let shader_params_bind_group_layout =
			create_shader_params_bind_group_layout(device, S::NUM_TEXTURES);
		let render_pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
			label: Some(&format!("{} layout", settings.label)),
			bind_group_layouts: &[
				texture_bind_group_layout,
				draw_params_bind_group_layout,
				&shader_params_bind_group_layout,
			],
			push_constant_ranges: &[],
		});
		let render_pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
			label: Some(&settings.label),
			layout: Some(&render_pipeline_layout),
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
		let shader_params_bind_group = create_shader_params_bind_group(
			device,
			shader_params_bind_group_layout,
			&shader_params_buffer,
			&settings.textures,
		);
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
	pub label: String,
	pub blend_mode: BlendMode,
	pub shader_params: S::Params,
	pub stencil_state: StencilState,
	pub enable_color_writes: bool,
	pub sample_count: u32,
	pub textures: Vec<MeshTexture>,
}

impl<S: Shader> Default for GraphicsPipelineSettings<S>
where
	S::Params: Default,
{
	fn default() -> Self {
		Self {
			label: "Graphics Pipeline".into(),
			blend_mode: Default::default(),
			shader_params: Default::default(),
			stencil_state: Default::default(),
			enable_color_writes: true,
			sample_count: 1,
			textures: vec![],
		}
	}
}

pub(crate) struct GraphicsPipelineInner {
	pub(crate) shader_params_buffer: Buffer,
	pub(crate) shader_params_bind_group: BindGroup,
	pub(crate) render_pipeline: RenderPipeline,
}

fn create_shader_params_bind_group_layout(device: &Device, num_textures: u32) -> BindGroupLayout {
	let mut entries = vec![BindGroupLayoutEntry {
		binding: 0,
		visibility: ShaderStages::VERTEX | ShaderStages::FRAGMENT,
		ty: BindingType::Buffer {
			ty: BufferBindingType::Uniform,
			has_dynamic_offset: false,
			min_binding_size: None,
		},
		count: None,
	}];
	for _ in 0..num_textures {
		entries.push(BindGroupLayoutEntry {
			binding: entries
				.len()
				.try_into()
				.expect("could not convert usize to u32"),
			visibility: ShaderStages::FRAGMENT,
			ty: BindingType::Texture {
				multisampled: false,
				view_dimension: TextureViewDimension::D2,
				sample_type: TextureSampleType::Float { filterable: true },
			},
			count: None,
		});
		entries.push(BindGroupLayoutEntry {
			binding: entries
				.len()
				.try_into()
				.expect("could not convert usize to u32"),
			visibility: ShaderStages::FRAGMENT,
			ty: BindingType::Sampler(SamplerBindingType::Filtering),
			count: None,
		});
	}
	device.create_bind_group_layout(&BindGroupLayoutDescriptor {
		label: Some("Shader Params Bind Group Layout"),
		entries: &entries,
	})
}

fn create_shader_params_bind_group(
	device: &Device,
	shader_params_bind_group_layout: BindGroupLayout,
	shader_params_buffer: &Buffer,
	textures: &[MeshTexture],
) -> BindGroup {
	let mut entries = vec![BindGroupEntry {
		binding: 0,
		resource: shader_params_buffer.as_entire_binding(),
	}];
	for texture in textures {
		let (view, sampler) = match texture {
			MeshTexture::Texture(texture) => (&texture.0.view, &texture.0.sampler),
			MeshTexture::Canvas(canvas) => (
				canvas
					.0
					.multisample_resolve_texture_view
					.as_ref()
					.unwrap_or(&canvas.0.view),
				&canvas.0.sampler,
			),
		};
		entries.push(BindGroupEntry {
			binding: entries
				.len()
				.try_into()
				.expect("could not convert usize to u32"),
			resource: BindingResource::TextureView(view),
		});
		entries.push(BindGroupEntry {
			binding: entries
				.len()
				.try_into()
				.expect("could not convert usize to u32"),
			resource: BindingResource::Sampler(sampler),
		});
	}
	device.create_bind_group(&BindGroupDescriptor {
		label: Some("Shader Params Bind Group"),
		layout: &shader_params_bind_group_layout,
		entries: &entries,
	})
}
