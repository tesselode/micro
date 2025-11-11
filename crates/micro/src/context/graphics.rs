mod cached_resources;
mod default_resources;
mod layouts;

pub(crate) use default_resources::*;
pub(crate) use layouts::*;

use std::{any::TypeId, collections::HashMap};

use bytemuck::{Pod, Zeroable};
use glam::{Mat4, UVec2, Vec3, uvec2};
use palette::{LinSrgb, LinSrgba};
use sdl3::video::Window;
use wgpu::{
	BindGroup, BindGroupDescriptor, BindGroupEntry, BindingResource, Buffer, BufferUsages,
	CompositeAlphaMode, Device, DeviceDescriptor, IndexFormat, Instance, LoadOp, Operations,
	PowerPreference, PresentMode, Queue, RenderPassColorAttachment,
	RenderPassDepthStencilAttachment, RenderPassDescriptor, RenderPipeline, RequestAdapterOptions,
	StoreOp, Surface, SurfaceConfiguration, SurfaceTargetUnsafe, TextureFormat, TextureUsages,
	TextureViewDescriptor,
	util::{BufferInitDescriptor, DeviceExt},
};

use crate::{
	ContextSettings,
	color::{ColorConstants, lin_srgb_to_wgpu_color, lin_srgba_to_wgpu_color},
	context::{
		Push,
		graphics::cached_resources::{CachedResources, RenderPipelineSettings},
	},
	graphics::{
		BlendMode, Canvas, RenderToCanvasSettings, Shader, StencilState, StorageBuffer, Vertex,
		canvas::CanvasKind,
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
	pub(crate) layouts: Layouts,
	pub(crate) default_resources: DefaultResources,
	pub(crate) clear_color: LinSrgb,
	graphics_state_stack: Vec<GraphicsState>,
	cached_resources: CachedResources,
	main_surface_draw_commands: Vec<DrawCommand>,
	current_canvas_render_pass: Option<CanvasRenderPass>,
	finished_canvas_render_passes: Vec<CanvasRenderPass>,
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
		let supported_sample_counts = adapter
			.get_texture_format_features(TextureFormat::Rgba8UnormSrgb)
			.flags
			.supported_sample_counts();
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
			desired_maximum_frame_latency: settings.desired_maximum_frame_latency,
			alpha_mode: CompositeAlphaMode::Auto,
			view_formats: vec![],
		};
		surface.configure(&device, &config);
		let layouts = Layouts::new(&device);
		let default_resources = DefaultResources::new(&device, &queue, &layouts);
		let main_surface_depth_stencil_texture = Texture::new(
			&device,
			&queue,
			uvec2(width, height),
			1,
			None,
			&TextureSettings::default(),
			InternalTextureSettings {
				format: TextureFormat::Depth24PlusStencil8,
				sample_count: 1,
			},
		);
		let mut ctx = Self {
			device,
			queue,
			supported_sample_counts,
			config,
			surface,
			main_surface_depth_stencil_texture,
			layouts,
			default_resources,
			clear_color: LinSrgb::BLACK,
			graphics_state_stack: vec![],
			cached_resources: CachedResources::new(),
			main_surface_draw_commands: vec![],
			current_canvas_render_pass: None,
			finished_canvas_render_passes: vec![],
		};
		ctx.graphics_state_stack.push(ctx.default_graphics_state());
		ctx
	}

	pub(crate) fn resize(&mut self, size: UVec2) {
		self.config.width = size.x;
		self.config.height = size.y;
		self.surface.configure(&self.device, &self.config);
		self.main_surface_depth_stencil_texture = Texture::new(
			&self.device,
			&self.queue,
			size,
			1,
			None,
			&TextureSettings::default(),
			InternalTextureSettings {
				format: TextureFormat::Depth24PlusStencil8,
				sample_count: 1,
			},
		);
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

	pub(crate) fn queue_draw_command<V: Vertex>(&mut self, settings: QueueDrawCommandSettings) {
		let vertex_type = TypeId::of::<V>();
		self.cached_resources.cache_vertex_info::<V>();
		let graphics_state = self
			.graphics_state_stack
			.last()
			.expect("no graphics state on stack");
		let sample_count =
			if let Some(CanvasRenderPass { canvas, .. }) = &self.current_canvas_render_pass {
				canvas.sample_count()
			} else {
				1
			};
		let texture_format =
			if let Some(CanvasRenderPass { canvas, .. }) = &self.current_canvas_render_pass {
				canvas.format()
			} else {
				self.config.format
			};
		let texture_view_dimension = settings.texture.view_dimension;
		let draw_command = DrawCommand {
			vertex_buffer: settings.vertex_buffer,
			index_buffer: settings.index_buffer,
			range: settings.range,
			instances: settings.instances,
			texture: settings.texture,
			draw_params: DrawParams {
				global_transform: self.coordinate_system_transform()
					* graphics_state.transform
					* settings.transform,
				local_transform: settings.transform,
				color: settings.color,
			},
			scissor_rect: graphics_state.scissor_rect,
			shader_params_bind_group: graphics_state
				.shader
				.params_bind_group
				.as_ref()
				.unwrap_or(&self.default_resources.default_shader_params_bind_group)
				.clone(),
			storage_buffers: graphics_state.shader.storage_buffers.clone(),
			shader_textures: graphics_state.shader.textures.clone(),
			stencil_reference: graphics_state.stencil_state.reference,
			render_pipeline_settings: RenderPipelineSettings {
				vertex_type,
				shader_name: graphics_state.shader.name.clone(),
				shader_source: graphics_state.shader.source.clone(),
				blend_mode: settings.blend_mode,
				enable_color_writes: graphics_state.stencil_state.enable_color_writes,
				enable_depth_testing: graphics_state.enable_depth_testing,
				wgpu_stencil_state: graphics_state.stencil_state.as_wgpu_stencil_state(),
				sample_count,
				texture_format,
				texture_view_dimension,
				num_storage_buffers: graphics_state.shader.storage_buffers.len(),
				num_shader_textures: graphics_state.shader.textures.len(),
			},
		};
		if let Some(CanvasRenderPass { draw_commands, .. }) = &mut self.current_canvas_render_pass {
			draw_commands.push(draw_command);
		} else {
			self.main_surface_draw_commands.push(draw_command);
		}
	}

	pub(crate) fn push_graphics_state(&mut self, new: Push) {
		self.graphics_state_stack.push(
			self.graphics_state_stack
				.last()
				.cloned()
				.unwrap_or_else(|| self.default_graphics_state())
				.push(&new),
		);
	}

	pub(crate) fn pop_graphics_state(&mut self) {
		self.graphics_state_stack.pop();
	}

	pub(crate) fn present_mode(&self) -> PresentMode {
		self.config.present_mode
	}

	pub(crate) fn desired_maximum_frame_latency(&self) -> u32 {
		self.config.desired_maximum_frame_latency
	}

	pub(crate) fn surface_format(&self) -> TextureFormat {
		self.config.format
	}

	pub(crate) fn current_render_target_size(&self) -> UVec2 {
		if let Some(CanvasRenderPass { canvas, .. }) = &self.current_canvas_render_pass {
			canvas.size()
		} else {
			uvec2(self.config.width, self.config.height)
		}
	}

	pub(crate) fn set_present_mode(&mut self, present_mode: PresentMode) {
		self.config.present_mode = present_mode;
		self.surface.configure(&self.device, &self.config);
	}

	pub(crate) fn set_desired_maximum_frame_latency(&mut self, frames: u32) {
		self.config.desired_maximum_frame_latency = frames;
		self.surface.configure(&self.device, &self.config);
	}

	pub(crate) fn present(&mut self) {
		self.create_shaders();
		self.create_render_pipelines();

		let mut encoder = self.device.create_command_encoder(&Default::default());

		// run canvas render passes
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
					depth_slice: None,
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
				&self.device,
				&mut self.layouts,
				&self.cached_resources.render_pipelines,
				&mut draw_commands,
				render_pass,
				URect::new(UVec2::ZERO, canvas.size()),
			);
		}

		// run main surface render pass
		let frame = self
			.surface
			.get_current_texture()
			.expect("error getting surface texture");
		let output = frame.texture.create_view(&TextureViewDescriptor::default());
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
			run_draw_commands(
				&self.device,
				&mut self.layouts,
				&self.cached_resources.render_pipelines,
				&mut self.main_surface_draw_commands,
				render_pass,
				URect::new(UVec2::ZERO, uvec2(self.config.width, self.config.height)),
			);
		}

		self.queue.submit([encoder.finish()]);
		frame.present();

		self.graphics_state_stack.clear();
		self.graphics_state_stack
			.push(self.default_graphics_state());
	}

	fn default_graphics_state(&self) -> GraphicsState {
		GraphicsState {
			transform: Mat4::IDENTITY,
			shader: self.default_resources.default_shader.clone(),
			stencil_state: StencilState::default(),
			enable_depth_testing: false,
			scissor_rect: None,
		}
	}

	fn coordinate_system_transform(&self) -> Mat4 {
		let current_render_target_size = self.current_render_target_size();
		Mat4::from_translation(Vec3::new(-1.0, 1.0, 0.0))
			* Mat4::from_scale(Vec3::new(
				2.0 / current_render_target_size.x as f32,
				-2.0 / current_render_target_size.y as f32,
				1.0,
			))
	}

	fn create_shaders(&mut self) {
		let _span = tracy_client::span!();
		self.cached_resources
			.create_shaders(&self.device, &self.main_surface_draw_commands);
		for CanvasRenderPass { draw_commands, .. } in &self.finished_canvas_render_passes {
			self.cached_resources
				.create_shaders(&self.device, draw_commands);
		}
	}

	fn create_render_pipelines(&mut self) {
		let _span = tracy_client::span!();
		self.cached_resources.create_render_pipelines(
			&self.device,
			&mut self.layouts,
			&self.main_surface_draw_commands,
		);
		for CanvasRenderPass { draw_commands, .. } in &self.finished_canvas_render_passes {
			self.cached_resources.create_render_pipelines(
				&self.device,
				&mut self.layouts,
				draw_commands,
			);
		}
	}
}

pub(crate) struct QueueDrawCommandSettings {
	pub(crate) vertex_buffer: Buffer,
	pub(crate) index_buffer: Buffer,
	pub(crate) range: (u32, u32),
	pub(crate) instances: (u32, u32),
	pub(crate) texture: Texture,
	pub(crate) transform: Mat4,
	pub(crate) color: LinSrgba,
	pub(crate) blend_mode: BlendMode,
}

#[derive(Debug, Clone, PartialEq)]
struct DrawCommand {
	vertex_buffer: Buffer,
	index_buffer: Buffer,
	range: (u32, u32),
	instances: (u32, u32),
	texture: Texture,
	draw_params: DrawParams,
	scissor_rect: Option<URect>,
	shader_params_bind_group: BindGroup,
	storage_buffers: Vec<StorageBuffer>,
	shader_textures: Vec<Texture>,
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

struct CanvasRenderPass {
	canvas: Canvas,
	settings: RenderToCanvasSettings,
	draw_commands: Vec<DrawCommand>,
}

#[derive(Debug, Clone, PartialEq)]
struct GraphicsState {
	transform: Mat4,
	shader: Shader,
	stencil_state: StencilState,
	enable_depth_testing: bool,
	scissor_rect: Option<URect>,
}

impl GraphicsState {
	fn push(&self, push: &Push) -> Self {
		Self {
			transform: if let Some(transform) = push.transform {
				self.transform * transform
			} else {
				self.transform
			},
			shader: push.shader.as_ref().unwrap_or(&self.shader).clone(),
			stencil_state: push.stencil_state.unwrap_or(self.stencil_state),
			enable_depth_testing: push
				.enable_depth_testing
				.unwrap_or(self.enable_depth_testing),
			scissor_rect: push.scissor_rect.unwrap_or(self.scissor_rect),
		}
	}
}

fn run_draw_commands(
	device: &Device,
	layouts: &mut Layouts,
	render_pipelines: &HashMap<RenderPipelineSettings, RenderPipeline>,
	draw_commands: &mut Vec<DrawCommand>,
	mut render_pass: wgpu::RenderPass<'_>,
	default_scissor_rect: URect,
) {
	for DrawCommand {
		vertex_buffer,
		index_buffer,
		range,
		instances,
		texture,
		draw_params,
		scissor_rect,
		shader_params_bind_group,
		storage_buffers,
		shader_textures,
		stencil_reference,
		render_pipeline_settings,
	} in draw_commands.drain(..)
	{
		let pipeline = &render_pipelines[&render_pipeline_settings];
		render_pass.set_pipeline(pipeline);
		render_pass.set_bind_group(
			0,
			&device.create_bind_group(&BindGroupDescriptor {
				label: Some("Mesh Bind Group"),
				layout: &layouts.mesh_bind_group_layout(texture.view_dimension, device),
				entries: &[
					BindGroupEntry {
						binding: 0,
						resource: device
							.create_buffer_init(&BufferInitDescriptor {
								label: Some("Draw Params Buffer"),
								contents: bytemuck::cast_slice(&[draw_params]),
								usage: BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
							})
							.as_entire_binding(),
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
			}),
			&[],
		);
		render_pass.set_bind_group(1, &shader_params_bind_group, &[]);
		render_pass.set_bind_group(
			2,
			&device.create_bind_group(&BindGroupDescriptor {
				label: Some("Storage Buffers Bind Group"),
				layout: &pipeline.get_bind_group_layout(2),
				entries: &storage_buffers
					.iter()
					.enumerate()
					.map(|(i, buffer)| BindGroupEntry {
						binding: i as u32,
						resource: buffer.0.as_entire_binding(),
					})
					.collect::<Vec<_>>(),
			}),
			&[],
		);
		render_pass.set_bind_group(
			3,
			&create_shader_textures_bind_group(device, pipeline, &shader_textures),
			&[],
		);
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
		render_pass.draw_indexed(range.0..range.1, 0, instances.0..instances.1);
	}
}

fn create_shader_textures_bind_group(
	device: &Device,
	pipeline: &RenderPipeline,
	textures: &[Texture],
) -> BindGroup {
	let mut entries = vec![];
	for (i, texture) in textures.iter().enumerate() {
		entries.push(BindGroupEntry {
			binding: (i * 2) as u32,
			resource: BindingResource::TextureView(&texture.view),
		});
		entries.push(BindGroupEntry {
			binding: (i * 2 + 1) as u32,
			resource: BindingResource::Sampler(&texture.sampler),
		});
	}
	device.create_bind_group(&BindGroupDescriptor {
		label: Some("Shader Textures Bind Group"),
		layout: &pipeline.get_bind_group_layout(3),
		entries: &entries,
	})
}
