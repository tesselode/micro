use std::collections::HashMap;

use bytemuck::{Pod, Zeroable};
use glam::{Mat4, UVec2, Vec3, uvec2};
use palette::{LinSrgb, LinSrgba};
use sdl3::video::Window;
use wgpu::{
	BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor,
	BindGroupLayoutEntry, BindingResource, BindingType, Buffer, BufferBindingType, BufferUsages,
	ColorTargetState, ColorWrites, CompareFunction, CompositeAlphaMode, DepthBiasState,
	DepthStencilState, Device, DeviceDescriptor, FragmentState, IndexFormat, Instance, LoadOp,
	MultisampleState, Operations, PipelineCompilationOptions, PipelineLayout,
	PipelineLayoutDescriptor, PowerPreference, PrimitiveState, Queue, RenderPassColorAttachment,
	RenderPassDepthStencilAttachment, RenderPassDescriptor, RenderPipeline,
	RenderPipelineDescriptor, RequestAdapterOptions, SamplerBindingType, ShaderModule,
	ShaderStages, StoreOp, Surface, SurfaceConfiguration, SurfaceTargetUnsafe, TextureFormat,
	TextureSampleType, TextureUsages, TextureViewDescriptor, TextureViewDimension,
	VertexBufferLayout, VertexState, VertexStepMode,
	util::{BufferInitDescriptor, DeviceExt},
};

use crate::{
	ContextSettings,
	color::{ColorConstants, lin_srgb_to_wgpu_color},
	graphics::{
		BlendMode, HasVertexAttributes, Shader, StencilState, Vertex2d,
		texture::{InternalTextureSettings, Texture, TextureSettings},
	},
	math::URect,
};

const DEFAULT_SHADER_SOURCE: &str = include_str!("shader.glsl");

pub(crate) struct GraphicsContext {
	pub(crate) device: Device,
	pub(crate) queue: Queue,
	config: SurfaceConfiguration,
	surface: Surface<'static>,
	main_surface_depth_stencil_texture: Texture,
	mesh_bind_group_layout: BindGroupLayout,
	pub(crate) shader_params_bind_group_layout: BindGroupLayout,
	pipeline_layout: PipelineLayout,
	pub(crate) default_texture: Texture,
	default_shader: Shader,
	pub(crate) clear_color: LinSrgb,
	pub(crate) transform_stack: Vec<Mat4>,
	pub(crate) stencil_state_stack: Vec<StencilState>,
	render_pipelines: HashMap<RenderPipelineSettings, RenderPipeline>,
	main_surface_draw_commands: Vec<DrawCommand>,
}

impl GraphicsContext {
	pub(crate) fn new(window: &Window, settings: &ContextSettings) -> Self {
		let instance = Instance::new(&Default::default());
		let surface = unsafe {
			instance.create_surface_unsafe(
				SurfaceTargetUnsafe::from_window(window)
					.expect("error creating surface target from window"),
			)
		}
		.expect("error creating surface");
		let adapter = pollster::block_on(instance.request_adapter(&RequestAdapterOptions {
			power_preference: PowerPreference::HighPerformance,
			compatible_surface: Some(&surface),
			..Default::default()
		}))
		.expect("error getting graphics adapter");
		let (device, queue) = pollster::block_on(adapter.request_device(&DeviceDescriptor {
			required_features: settings.required_graphics_features,
			..Default::default()
		}))
		.expect("error getting graphics device");
		let surface_capabilities = surface.get_capabilities(&adapter);
		let surface_format = surface_capabilities
			.formats
			.iter()
			.copied()
			.find(|f| f.is_srgb())
			.unwrap_or(surface_capabilities.formats[0]);
		let (width, height) = window.size();
		let config = SurfaceConfiguration {
			usage: TextureUsages::RENDER_ATTACHMENT,
			format: surface_format,
			width,
			height,
			present_mode: settings.present_mode,
			desired_maximum_frame_latency: settings.max_queued_frames,
			alpha_mode: CompositeAlphaMode::Auto,
			view_formats: vec![],
		};
		surface.configure(&device, &config);
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
		let main_surface_depth_stencil_texture = Texture::new(
			&device,
			&queue,
			uvec2(width, height),
			None,
			TextureSettings::default(),
			InternalTextureSettings {
				format: TextureFormat::Depth24PlusStencil8,
				sample_count: 1,
			},
		);
		let default_texture = Texture::new(
			&device,
			&queue,
			UVec2::new(1, 1),
			Some(&[255, 255, 255, 255]),
			TextureSettings::default(),
			InternalTextureSettings::default(),
		);
		let default_shader = Shader::new_internal(
			&device,
			&shader_params_bind_group_layout,
			"Default Shader",
			DEFAULT_SHADER_SOURCE,
		);
		Self {
			device,
			queue,
			config,
			surface,
			main_surface_depth_stencil_texture,
			mesh_bind_group_layout,
			shader_params_bind_group_layout,
			pipeline_layout,
			default_texture,
			default_shader,
			clear_color: LinSrgb::BLACK,
			transform_stack: vec![],
			stencil_state_stack: vec![],
			render_pipelines: HashMap::new(),
			main_surface_draw_commands: vec![],
		}
	}

	pub(crate) fn resize(&mut self, size: UVec2) {
		self.config.width = size.x;
		self.config.height = size.y;
		self.surface.configure(&self.device, &self.config);
		self.main_surface_depth_stencil_texture = Texture::new(
			&self.device,
			&self.queue,
			size,
			None,
			TextureSettings::default(),
			InternalTextureSettings {
				format: TextureFormat::Depth24PlusStencil8,
				sample_count: 1,
			},
		);
	}

	pub(crate) fn global_transform(&self) -> Mat4 {
		let current_render_target_size = self.current_render_target_size();
		let coordinate_system_transform = Mat4::from_translation(Vec3::new(-1.0, 1.0, 0.0))
			* Mat4::from_scale(Vec3::new(
				2.0 / current_render_target_size.x as f32,
				-2.0 / current_render_target_size.y as f32,
				1.0,
			));
		self.transform_stack
			.iter()
			.fold(coordinate_system_transform, |previous, transform| {
				previous * *transform
			})
	}

	pub(crate) fn queue_draw_command(&mut self, settings: QueueDrawCommandSettings) {
		let shader = settings.shader.unwrap_or(self.default_shader.clone());
		let stencil_state = self.stencil_state_stack.last().copied().unwrap_or_default();
		self.main_surface_draw_commands.push(DrawCommand {
			vertex_buffer: settings.vertex_buffer,
			index_buffer: settings.index_buffer,
			range: settings.range,
			texture: settings.texture,
			draw_params: DrawParams {
				global_transform: self.global_transform() * settings.transform,
				local_transform: settings.transform,
				color: settings.color,
			},
			scissor_rect: settings.scissor_rect,
			shader_params_bind_group: shader.params_bind_group,
			stencil_reference: stencil_state.reference,
			render_pipeline_settings: RenderPipelineSettings {
				vertex_shader: shader.vertex,
				fragment_shader: shader.fragment,
				blend_mode: settings.blend_mode,
				enable_color_writes: stencil_state.enable_color_writes,
				wgpu_stencil_state: stencil_state.as_wgpu_stencil_state(),
			},
		});
	}

	pub(crate) fn present(&mut self) {
		self.create_render_pipelines();
		let default_scissor_rect = self.default_scissor_rect();
		let mut encoder = self.device.create_command_encoder(&Default::default());
		let frame = self
			.surface
			.get_current_texture()
			.expect("error getting surface texture");
		let output = frame.texture.create_view(&TextureViewDescriptor::default());
		{
			let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
				label: Some("Main Surface Render Pass"),
				color_attachments: &[Some(RenderPassColorAttachment {
					view: &output,
					resolve_target: None,
					ops: Operations {
						load: LoadOp::Clear(lin_srgb_to_wgpu_color(self.clear_color)),
						store: StoreOp::Store,
					},
					depth_slice: None,
				})],
				depth_stencil_attachment: Some(RenderPassDepthStencilAttachment {
					view: &self.main_surface_depth_stencil_texture.view,
					depth_ops: Some(Operations {
						load: LoadOp::Clear(1.0),
						store: StoreOp::Store,
					}),
					stencil_ops: Some(Operations {
						load: LoadOp::Clear(0),
						store: StoreOp::Store,
					}),
				}),
				timestamp_writes: None,
				occlusion_query_set: None,
			});
			for DrawCommand {
				vertex_buffer,
				index_buffer,
				range,
				texture,
				draw_params,
				scissor_rect,
				shader_params_bind_group,
				stencil_reference,
				render_pipeline_settings,
			} in self.main_surface_draw_commands.drain(..)
			{
				let draw_params_buffer = self.device.create_buffer_init(&BufferInitDescriptor {
					label: Some("Draw Params Buffer"),
					contents: bytemuck::cast_slice(&[draw_params]),
					usage: BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
				});
				let mesh_bind_group = self.device.create_bind_group(&BindGroupDescriptor {
					label: Some("Draw Params Bind Group"),
					layout: &self.mesh_bind_group_layout,
					entries: &[
						BindGroupEntry {
							binding: 0,
							resource: draw_params_buffer.as_entire_binding(),
						},
						BindGroupEntry {
							binding: 1,
							resource: BindingResource::TextureView(&texture.view),
						},
						BindGroupEntry {
							binding: 2,
							resource: BindingResource::Sampler(&texture.sampler),
						},
					],
				});
				render_pass.set_pipeline(&self.render_pipelines[&render_pipeline_settings]);
				render_pass.set_bind_group(0, &mesh_bind_group, &[]);
				render_pass.set_bind_group(1, &shader_params_bind_group, &[]);
				render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
				render_pass.set_index_buffer(index_buffer.slice(..), IndexFormat::Uint32);
				let scissor_rect = scissor_rect.unwrap_or(default_scissor_rect);
				render_pass.set_scissor_rect(
					scissor_rect.left(),
					scissor_rect.top(),
					scissor_rect.size.x,
					scissor_rect.size.y,
				);
				render_pass.set_stencil_reference(stencil_reference as u32);
				render_pass.draw_indexed(range.0..range.1, 0, 0..1);
			}
		}
		self.queue.submit([encoder.finish()]);
		frame.present();
	}

	fn current_render_target_size(&self) -> UVec2 {
		/* if let Some(CanvasRenderPass { canvas, .. }) = &self.current_canvas_render_pass {
			canvas.size()
		} else { */
		uvec2(self.config.width, self.config.height)
		// }
	}

	fn default_scissor_rect(&self) -> URect {
		let size = /* if let Some(CanvasRenderPass { canvas, .. }) = &self.current_canvas_render_pass {
			canvas.size()
		} else { */
			uvec2(self.config.width, self.config.height)
		// }
		;
		URect::new(UVec2::ZERO, size)
	}

	fn create_render_pipelines(&mut self) {
		for DrawCommand {
			render_pipeline_settings,
			..
		} in &self.main_surface_draw_commands
		{
			self.render_pipelines
				.entry(render_pipeline_settings.clone())
				.or_insert_with(|| {
					create_render_pipeline(
						&self.device,
						&self.config,
						render_pipeline_settings,
						&self.pipeline_layout,
					)
				});
		}
	}
}

pub(crate) struct QueueDrawCommandSettings {
	pub(crate) vertex_buffer: Buffer,
	pub(crate) index_buffer: Buffer,
	pub(crate) range: (u32, u32),
	pub(crate) texture: Texture,
	pub(crate) transform: Mat4,
	pub(crate) color: LinSrgba,
	pub(crate) scissor_rect: Option<URect>,
	pub(crate) shader: Option<Shader>,
	pub(crate) blend_mode: BlendMode,
}

#[derive(Debug, Clone, PartialEq)]
struct DrawCommand {
	vertex_buffer: Buffer,
	index_buffer: Buffer,
	range: (u32, u32),
	texture: Texture,
	draw_params: DrawParams,
	scissor_rect: Option<URect>,
	shader_params_bind_group: BindGroup,
	stencil_reference: u8,
	render_pipeline_settings: RenderPipelineSettings,
}

#[derive(Debug, Clone, Copy, PartialEq, Pod, Zeroable)]
#[repr(C)]
struct DrawParams {
	global_transform: Mat4,
	local_transform: Mat4,
	color: LinSrgba,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct RenderPipelineSettings {
	vertex_shader: ShaderModule,
	fragment_shader: ShaderModule,
	blend_mode: BlendMode,
	enable_color_writes: bool,
	wgpu_stencil_state: wgpu::StencilState,
}

fn create_render_pipeline(
	device: &Device,
	config: &SurfaceConfiguration,
	settings: &RenderPipelineSettings,
	pipeline_layout: &PipelineLayout,
) -> RenderPipeline {
	device.create_render_pipeline(&RenderPipelineDescriptor {
		label: None,
		layout: Some(pipeline_layout),
		vertex: VertexState {
			module: &settings.vertex_shader,
			entry_point: Some("main"),
			compilation_options: PipelineCompilationOptions::default(),
			buffers: &[VertexBufferLayout {
				array_stride: std::mem::size_of::<Vertex2d>() as u64,
				step_mode: VertexStepMode::Vertex,
				attributes: &Vertex2d::attributes(),
			}],
		},
		primitive: PrimitiveState::default(),
		depth_stencil: Some(DepthStencilState {
			format: TextureFormat::Depth24PlusStencil8,
			/* depth_write_enabled: settings.enable_depth_testing,
			depth_compare: if settings.enable_depth_testing {
				CompareFunction::Less
			} else {
				CompareFunction::Always
			}, */
			depth_write_enabled: false,
			depth_compare: CompareFunction::Always,
			stencil: settings.wgpu_stencil_state.clone(),
			bias: DepthBiasState::default(),
		}),
		multisample: MultisampleState::default(),
		fragment: Some(FragmentState {
			module: &settings.fragment_shader,
			entry_point: Some("main"),
			compilation_options: PipelineCompilationOptions::default(),
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
		multiview: None,
		cache: None,
	})
}
