use bytemuck::{Pod, Zeroable};
use glam::{Mat4, UVec2, Vec3, uvec2};
use palette::{LinSrgb, LinSrgba};
use sdl2::video::Window;
use wgpu::{
	BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor,
	BindGroupLayoutEntry, BindingResource, BindingType, Buffer, BufferBindingType, BufferUsages,
	CompositeAlphaMode, Device, DeviceDescriptor, Features, IndexFormat, Instance, LoadOp,
	Operations, PowerPreference, PresentMode, Queue, RenderPassColorAttachment,
	RenderPassDepthStencilAttachment, RenderPassDescriptor, RequestAdapterOptions,
	SamplerBindingType, ShaderStages, StoreOp, Surface, SurfaceConfiguration, SurfaceTargetUnsafe,
	TextureFormat, TextureSampleType, TextureUsages, TextureViewDescriptor, TextureViewDimension,
	util::{BufferInitDescriptor, DeviceExt},
};

use crate::{
	color::{ColorConstants, lin_srgb_to_wgpu_color, lin_srgba_to_wgpu_color},
	graphics::{
		Canvas, CanvasKind, DefaultShader, GraphicsPipeline, GraphicsPipelineBuilder,
		RawGraphicsPipeline, RenderToCanvasSettings,
		texture::{InternalTextureSettings, Texture, TextureSettings},
	},
	math::URect,
};

pub(crate) struct GraphicsContext {
	pub(crate) device: Device,
	pub(crate) queue: Queue,
	pub(crate) supported_sample_counts: Vec<u32>,
	config: SurfaceConfiguration,
	surface: Surface<'static>,
	main_surface_depth_stencil_texture: Texture,
	pub(crate) mesh_bind_group_layout: BindGroupLayout,
	pub(crate) shader_params_bind_group_layout: BindGroupLayout,
	default_texture: Texture,
	pub(crate) default_graphics_pipeline: GraphicsPipeline,
	pub(crate) clear_color: LinSrgb,
	pub(crate) transform_stack: Vec<Mat4>,
	pub(crate) stencil_reference_stack: Vec<u8>,
	main_surface_draw_commands: Vec<DrawCommand>,
	current_canvas_render_pass: Option<CanvasRenderPass>,
	finished_canvas_render_passes: Vec<CanvasRenderPass>,
}

impl GraphicsContext {
	pub(crate) fn new(
		window: &Window,
		present_mode: PresentMode,
		required_features: Features,
	) -> Self {
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
		let supported_sample_counts = adapter
			.get_texture_format_features(TextureFormat::Rgba8UnormSrgb)
			.flags
			.supported_sample_counts();
		let (device, queue) = pollster::block_on(adapter.request_device(
			&DeviceDescriptor {
				required_features,
				..Default::default()
			},
			None,
		))
		.expect("error getting graphics device");
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
			present_mode,
			desired_maximum_frame_latency: 1,
			alpha_mode: CompositeAlphaMode::Auto,
			view_formats: vec![],
		};
		surface.configure(&device, &config);
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
		let default_graphics_pipeline = GraphicsPipeline::<DefaultShader>::new_internal(
			&device,
			&mesh_bind_group_layout,
			&shader_params_bind_group_layout,
			GraphicsPipelineBuilder {
				label: "Default Graphics Pipeline".into(),
				blend_mode: Default::default(),
				shader_params: Default::default(),
				enable_depth_testing: false,
				stencil_state: Default::default(),
				enable_color_writes: true,
				sample_count: 1,
				format: config.format,
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
		Self {
			device,
			queue,
			supported_sample_counts,
			config,
			surface,
			main_surface_depth_stencil_texture,
			mesh_bind_group_layout,
			shader_params_bind_group_layout,
			default_texture,
			clear_color: LinSrgb::BLACK,
			default_graphics_pipeline,
			transform_stack: vec![],
			stencil_reference_stack: vec![0],
			main_surface_draw_commands: vec![],
			current_canvas_render_pass: None,
			finished_canvas_render_passes: vec![],
		}
	}

	pub(crate) fn current_render_target_size(&self) -> UVec2 {
		if let Some(CanvasRenderPass { canvas, .. }) = &self.current_canvas_render_pass {
			canvas.size()
		} else {
			uvec2(self.config.width, self.config.height)
		}
	}

	pub(crate) fn start_canvas_render_pass(
		&mut self,
		canvas: Canvas,
		settings: RenderToCanvasSettings,
	) {
		if self.current_canvas_render_pass.is_some() {
			panic!("cannot nest render_to calls");
		}
		self.current_canvas_render_pass = Some(CanvasRenderPass {
			canvas,
			settings,
			draw_commands: vec![],
		});
	}

	pub(crate) fn finish_canvas_render_pass(&mut self) {
		let canvas_render_pass = self
			.current_canvas_render_pass
			.take()
			.expect("no current canvas render pass");
		self.finished_canvas_render_passes.push(canvas_render_pass);
	}

	pub(crate) fn queue_draw_command(
		&mut self,
		settings: QueueDrawCommandSettings,
		graphics_pipeline: RawGraphicsPipeline,
		num_instances: u32,
	) {
		let command = DrawCommand {
			vertex_buffer: settings.vertex_buffer,
			index_buffer: settings.index_buffer,
			range: settings.range,
			graphics_pipeline,
			texture: settings.texture,
			draw_params: DrawParams {
				transform: self.global_transform() * settings.local_transform,
				local_transform: settings.local_transform,
				color: settings.color,
			},
			scissor_rect: settings
				.scissor_rect
				.unwrap_or_else(|| self.default_scissor_rect()),
			stencil_reference: *self.stencil_reference_stack.last().unwrap(),
			num_instances,
		};
		if let Some(CanvasRenderPass { draw_commands, .. }) = &mut self.current_canvas_render_pass {
			draw_commands.push(command);
		} else {
			self.main_surface_draw_commands.push(command);
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

	pub(crate) fn present_mode(&self) -> PresentMode {
		self.config.present_mode
	}

	pub(crate) fn surface_format(&self) -> TextureFormat {
		self.config.format
	}

	pub(crate) fn set_present_mode(&mut self, present_mode: PresentMode) {
		self.config.present_mode = present_mode;
		self.surface.configure(&self.device, &self.config);
	}

	pub(crate) fn present(&mut self) {
		let frame = self
			.surface
			.get_current_texture()
			.expect("error getting surface texture");
		let output = frame.texture.create_view(&TextureViewDescriptor::default());
		let mut encoder = self.device.create_command_encoder(&Default::default());

		for CanvasRenderPass {
			canvas,
			settings,
			mut draw_commands,
		} in self.finished_canvas_render_passes.drain(..)
		{
			let render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
				label: Some(&settings.render_pass_label),
				color_attachments: &[Some(RenderPassColorAttachment {
					view: match &canvas.kind {
						CanvasKind::Normal { texture }
						| CanvasKind::Multisampled { texture, .. } => &texture.view,
					},
					resolve_target: match &canvas.kind {
						CanvasKind::Normal { .. } => None,
						CanvasKind::Multisampled {
							resolve_texture, ..
						} => Some(&resolve_texture.view),
					},
					ops: Operations {
						load: match settings.clear_color {
							Some(clear_color) => {
								LoadOp::Clear(lin_srgba_to_wgpu_color(clear_color))
							}
							None => LoadOp::Load,
						},
						store: StoreOp::Store,
					},
				})],
				depth_stencil_attachment: Some(RenderPassDepthStencilAttachment {
					view: &canvas.depth_stencil_texture.view,
					depth_ops: Some(Operations {
						load: match settings.clear_depth_buffer {
							true => LoadOp::Clear(1.0),
							false => LoadOp::Load,
						},
						store: StoreOp::Store,
					}),
					stencil_ops: Some(Operations {
						load: match settings.clear_stencil_value {
							true => LoadOp::Clear(0),
							false => LoadOp::Load,
						},
						store: StoreOp::Store,
					}),
				}),
				timestamp_writes: None,
				occlusion_query_set: None,
			});
			run_draw_commands(
				render_pass,
				&mut draw_commands,
				&self.device,
				&self.default_texture,
				&self.mesh_bind_group_layout,
			);
		}

		{
			let render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
				label: Some("Main Surface Render Pass"),
				color_attachments: &[Some(RenderPassColorAttachment {
					view: &output,
					resolve_target: None,
					ops: Operations {
						load: LoadOp::Clear(lin_srgb_to_wgpu_color(self.clear_color)),
						store: StoreOp::Store,
					},
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
			run_draw_commands(
				render_pass,
				&mut self.main_surface_draw_commands,
				&self.device,
				&self.default_texture,
				&self.mesh_bind_group_layout,
			);
		}

		self.queue.submit([encoder.finish()]);
		frame.present();
	}

	fn global_transform(&self) -> Mat4 {
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

	fn default_scissor_rect(&self) -> URect {
		let size = if let Some(CanvasRenderPass { canvas, .. }) = &self.current_canvas_render_pass {
			canvas.size()
		} else {
			uvec2(self.config.width, self.config.height)
		};
		URect::new(UVec2::ZERO, size)
	}
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct QueueDrawCommandSettings {
	pub vertex_buffer: Buffer,
	pub index_buffer: Buffer,
	pub range: (u32, u32),
	pub texture: Option<Texture>,
	pub local_transform: Mat4,
	pub color: LinSrgba,
	pub scissor_rect: Option<URect>,
}

#[derive(Debug, Clone, Copy, PartialEq, Pod, Zeroable)]
#[repr(C)]
struct DrawParams {
	transform: Mat4,
	local_transform: Mat4,
	color: LinSrgba,
}

struct DrawCommand {
	vertex_buffer: Buffer,
	index_buffer: Buffer,
	range: (u32, u32),
	graphics_pipeline: RawGraphicsPipeline,
	texture: Option<Texture>,
	draw_params: DrawParams,
	scissor_rect: URect,
	stencil_reference: u8,
	num_instances: u32,
}

struct CanvasRenderPass {
	canvas: Canvas,
	settings: RenderToCanvasSettings,
	draw_commands: Vec<DrawCommand>,
}

fn run_draw_commands(
	mut render_pass: wgpu::RenderPass<'_>,
	draw_commands: &mut Vec<DrawCommand>,
	device: &Device,
	default_texture: &Texture,
	mesh_bind_group_layout: &BindGroupLayout,
) {
	for DrawCommand {
		vertex_buffer,
		index_buffer,
		range,
		graphics_pipeline,
		texture,
		draw_params,
		scissor_rect,
		stencil_reference,
		num_instances,
	} in draw_commands.drain(..)
	{
		let draw_params_buffer = device.create_buffer_init(&BufferInitDescriptor {
			label: Some("Draw Params Buffer"),
			contents: bytemuck::cast_slice(&[draw_params]),
			usage: BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
		});
		let texture = texture.unwrap_or(default_texture.clone());
		let mesh_bind_group = device.create_bind_group(&BindGroupDescriptor {
			label: Some("Draw Params Bind Group"),
			layout: mesh_bind_group_layout,
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
		render_pass.set_pipeline(&graphics_pipeline.render_pipeline);
		render_pass.set_bind_group(0, &mesh_bind_group, &[]);
		render_pass.set_bind_group(1, &graphics_pipeline.shader_params_bind_group, &[]);
		render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
		render_pass.set_index_buffer(index_buffer.slice(..), IndexFormat::Uint32);
		render_pass.set_scissor_rect(
			scissor_rect.left(),
			scissor_rect.top(),
			scissor_rect.size.x,
			scissor_rect.size.y,
		);
		render_pass.set_stencil_reference(stencil_reference as u32);
		render_pass.draw_indexed(range.0..range.1, 0, 0..num_instances);
	}
}
