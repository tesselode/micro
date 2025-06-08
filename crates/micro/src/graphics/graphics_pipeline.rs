mod builder;

pub use builder::*;
use bytemuck::Pod;
use derive_more::From;

use std::{fmt::Debug, marker::PhantomData};

use wgpu::{
	BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor,
	BindGroupLayoutEntry, BindingResource, BindingType, Buffer, BufferAddress, BufferBindingType,
	BufferUsages, ColorTargetState, ColorWrites, CompareFunction, DepthBiasState,
	DepthStencilState, Device, FragmentState, MultisampleState, PipelineCompilationOptions,
	PipelineLayoutDescriptor, PrimitiveState, PrimitiveTopology, RenderPipeline,
	RenderPipelineDescriptor, SamplerBindingType, ShaderModuleDescriptor, ShaderStages,
	TextureFormat, TextureSampleType, TextureViewDimension, VertexAttribute, VertexBufferLayout,
	VertexState, VertexStepMode,
	util::{BufferInitDescriptor, DeviceExt},
};

use crate::{
	Context,
	graphics::{Canvas, texture::Texture},
};

use super::{
	BlendMode, DefaultShader, HasVertexAttributes, Shader, StencilState, drawable::Drawable,
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

	pub fn set_storage_buffer<T: Pod>(&self, ctx: &Context, index: u32, data: &[T]) {
		ctx.graphics.queue.write_buffer(
			&self.raw.storage_buffers[index as usize],
			0,
			bytemuck::cast_slice(data),
		);
	}

	pub fn with_storage_buffer<T: Pod>(&self, ctx: &Context, index: u32, data: &[T]) -> Self {
		Self {
			raw: self
				.raw
				.with_storage_buffer(ctx, index, bytemuck::cast_slice(data)),
			..self.clone()
		}
	}

	pub fn set_texture(
		&mut self,
		ctx: &Context,
		index: u32,
		texture: impl Into<GraphicsPipelineTexture>,
	) {
		self.raw.set_texture(ctx, index, texture);
	}

	pub fn with_texture(
		&self,
		ctx: &Context,
		index: u32,
		texture: impl Into<GraphicsPipelineTexture>,
	) -> Self {
		Self {
			raw: self.raw.with_texture(ctx, index, texture),
			..self.clone()
		}
	}

	pub fn draw(&self, ctx: &mut Context, drawable: &impl Drawable<Vertex = S::Vertex>) {
		self.draw_instanced(ctx, 1, drawable);
	}

	pub fn draw_instanced(
		&self,
		ctx: &mut Context,
		num_instances: u32,
		drawable: &impl Drawable<Vertex = S::Vertex>,
	) {
		for settings in drawable.draw_instructions(ctx).into_iter() {
			ctx.graphics
				.queue_draw_command(settings, self.raw(), num_instances);
		}
	}

	pub(crate) fn new_internal(
		device: &Device,
		mesh_bind_group_layout: &BindGroupLayout,
		shader_params_bind_group_layout: &BindGroupLayout,
		default_texture: &Texture,
		builder: GraphicsPipelineBuilder<S>,
	) -> Self {
		Self {
			raw: RawGraphicsPipeline::new(
				device,
				mesh_bind_group_layout,
				shader_params_bind_group_layout,
				default_texture,
				RawGraphicsPipelineSettings {
					label: builder.label,
					blend_mode: builder.blend_mode,
					shader_module_descriptor: S::DESCRIPTOR,
					shader_params: bytemuck::cast_slice(&[builder.shader_params]),
					num_storage_buffers: S::NUM_STORAGE_BUFFERS,
					storage_buffers: builder.storage_buffers,
					num_textures: S::NUM_TEXTURES,
					textures: builder.textures,
					vertex_size: std::mem::size_of::<S::Vertex>(),
					vertex_attributes: S::Vertex::attributes(),
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

#[derive(Debug, Clone, PartialEq, From)]
pub enum GraphicsPipelineTexture {
	Texture(Texture),
	Canvas(Canvas),
}

impl GraphicsPipelineTexture {
	fn drawable_texture(&self) -> Texture {
		match self {
			Self::Texture(texture) => texture.clone(),
			Self::Canvas(canvas) => canvas.drawable_texture(),
		}
	}
}

impl From<&Texture> for GraphicsPipelineTexture {
	fn from(value: &Texture) -> Self {
		Self::Texture(value.clone())
	}
}

impl From<&Canvas> for GraphicsPipelineTexture {
	fn from(value: &Canvas) -> Self {
		Self::Canvas(value.clone())
	}
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct RawGraphicsPipeline {
	pub label: String,
	pub sample_count: u32,
	pub format: TextureFormat,
	pub render_pipeline: RenderPipeline,
	pub shader_params_buffer: Buffer,
	pub shader_params_bind_group: BindGroup,
	pub storage_buffers_bind_group_layout: BindGroupLayout,
	pub storage_buffers: Vec<Buffer>,
	pub storage_buffers_bind_group: BindGroup,
	pub textures_bind_group_layout: BindGroupLayout,
	pub textures: Vec<GraphicsPipelineTexture>,
	pub textures_bind_group: BindGroup,
}

impl RawGraphicsPipeline {
	fn new(
		device: &Device,
		mesh_bind_group_layout: &BindGroupLayout,
		shader_params_bind_group_layout: &BindGroupLayout,
		default_texture: &Texture,
		mut settings: RawGraphicsPipelineSettings,
	) -> Self {
		let shader = device.create_shader_module(settings.shader_module_descriptor);
		let storage_buffers_bind_group_layout =
			device.create_bind_group_layout(&BindGroupLayoutDescriptor {
				label: Some(&format!(
					"{} - Storage Buffers Bind Group Layout",
					&settings.label
				)),
				entries: &(0..settings.num_storage_buffers)
					.map(|i| BindGroupLayoutEntry {
						binding: i,
						visibility: ShaderStages::VERTEX_FRAGMENT,
						ty: BindingType::Buffer {
							ty: BufferBindingType::Storage { read_only: true },
							has_dynamic_offset: false,
							min_binding_size: None,
						},
						count: None,
					})
					.collect::<Vec<_>>(),
			});
		let textures_bind_group_layout =
			device.create_bind_group_layout(&BindGroupLayoutDescriptor {
				label: Some(&format!("{} - Textures Bind Group Layout", &settings.label)),
				entries: &{
					let mut entries = vec![];
					for i in 0..settings.num_textures {
						entries.push(BindGroupLayoutEntry {
							binding: i * 2,
							visibility: ShaderStages::FRAGMENT,
							ty: BindingType::Texture {
								sample_type: TextureSampleType::Float { filterable: true },
								view_dimension: TextureViewDimension::D2,
								multisampled: false,
							},
							count: None,
						});
						entries.push(BindGroupLayoutEntry {
							binding: i * 2 + 1,
							visibility: ShaderStages::FRAGMENT,
							ty: BindingType::Sampler(SamplerBindingType::Filtering),
							count: None,
						});
					}
					entries
				},
			});
		let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
			label: Some(&format!("{} - Pipeline Layout", &settings.label)),
			bind_group_layouts: &[
				mesh_bind_group_layout,
				shader_params_bind_group_layout,
				&storage_buffers_bind_group_layout,
				&textures_bind_group_layout,
			],
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
		settings
			.storage_buffers
			.resize(settings.num_storage_buffers as usize, vec![0]);
		let storage_buffers = settings
			.storage_buffers
			.iter()
			.map(|contents| {
				device.create_buffer_init(&BufferInitDescriptor {
					label: None,
					contents,
					usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
				})
			})
			.collect::<Vec<_>>();
		let storage_buffers_bind_group = device.create_bind_group(&BindGroupDescriptor {
			label: Some(&format!("{} - Storage Buffers Bind Group", &settings.label)),
			layout: &storage_buffers_bind_group_layout,
			entries: &storage_buffers
				.iter()
				.enumerate()
				.map(|(i, buffer)| BindGroupEntry {
					binding: i as u32,
					resource: buffer.as_entire_binding(),
				})
				.collect::<Vec<_>>(),
		});
		settings
			.textures
			.resize(settings.num_textures as usize, default_texture.into());
		let textures_bind_group = create_textures_bind_group(
			device,
			&settings.textures,
			&settings.label,
			&textures_bind_group_layout,
		);
		let render_pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
			label: Some(&settings.label),
			layout: Some(&pipeline_layout),
			vertex: VertexState {
				module: &shader,
				entry_point: Some("vs_main"),
				compilation_options: PipelineCompilationOptions::default(),
				buffers: &[VertexBufferLayout {
					array_stride: settings.vertex_size as BufferAddress,
					step_mode: VertexStepMode::Vertex,
					attributes: &settings.vertex_attributes,
				}],
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
			sample_count: settings.sample_count,
			format: settings.format,
			render_pipeline,
			shader_params_buffer,
			shader_params_bind_group,
			storage_buffers_bind_group_layout,
			storage_buffers,
			storage_buffers_bind_group,
			textures_bind_group_layout,
			textures: settings.textures,
			textures_bind_group,
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

	fn with_storage_buffer(&self, ctx: &Context, index: u32, contents: &[u8]) -> Self {
		let device = &ctx.graphics.device;
		let storage_buffer = device.create_buffer_init(&BufferInitDescriptor {
			label: None,
			contents,
			usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
		});
		let mut storage_buffers = self.storage_buffers.clone();
		storage_buffers[index as usize] = storage_buffer;
		let storage_buffers_bind_group = device.create_bind_group(&BindGroupDescriptor {
			label: Some(&format!("{} - Storage Buffers Bind Group", &self.label)),
			layout: &self.storage_buffers_bind_group_layout,
			entries: &storage_buffers
				.iter()
				.enumerate()
				.map(|(i, buffer)| BindGroupEntry {
					binding: i as u32,
					resource: buffer.as_entire_binding(),
				})
				.collect::<Vec<_>>(),
		});
		Self {
			storage_buffers,
			storage_buffers_bind_group,
			..self.clone()
		}
	}

	fn set_texture(
		&mut self,
		ctx: &Context,
		index: u32,
		texture: impl Into<GraphicsPipelineTexture>,
	) {
		let device = &ctx.graphics.device;
		self.textures[index as usize] = texture.into();
		self.textures_bind_group = create_textures_bind_group(
			device,
			&self.textures,
			&self.label,
			&self.textures_bind_group_layout,
		);
	}

	fn with_texture(
		&self,
		ctx: &Context,
		index: u32,
		texture: impl Into<GraphicsPipelineTexture>,
	) -> RawGraphicsPipeline {
		let mut new = self.clone();
		new.set_texture(ctx, index, texture);
		new
	}
}

struct RawGraphicsPipelineSettings<'a> {
	label: String,
	blend_mode: BlendMode,
	shader_module_descriptor: ShaderModuleDescriptor<'a>,
	shader_params: &'a [u8],
	vertex_size: usize,
	vertex_attributes: Vec<VertexAttribute>,
	num_storage_buffers: u32,
	storage_buffers: Vec<Vec<u8>>,
	num_textures: u32,
	textures: Vec<GraphicsPipelineTexture>,
	enable_depth_testing: bool,
	stencil_state: StencilState,
	enable_color_writes: bool,
	sample_count: u32,
	format: TextureFormat,
}

fn create_textures_bind_group(
	device: &Device,
	graphics_pipeline_textures: &[GraphicsPipelineTexture],
	graphics_pipeline_label: &str,
	textures_bind_group_layout: &BindGroupLayout,
) -> BindGroup {
	let mut texture_bind_group_entries = vec![];
	let textures = graphics_pipeline_textures
		.iter()
		.map(|graphics_pipeline_texture| graphics_pipeline_texture.drawable_texture())
		.collect::<Vec<_>>();
	for (i, texture) in textures.iter().enumerate() {
		texture_bind_group_entries.push(BindGroupEntry {
			binding: (i * 2) as u32,
			resource: BindingResource::TextureView(&texture.view),
		});
		texture_bind_group_entries.push(BindGroupEntry {
			binding: (i * 2 + 1) as u32,
			resource: BindingResource::Sampler(&texture.sampler),
		});
	}
	device.create_bind_group(&BindGroupDescriptor {
		label: Some(&format!(
			"{} - Textures Bind Group",
			graphics_pipeline_label
		)),
		layout: textures_bind_group_layout,
		entries: &texture_bind_group_entries,
	})
}
