use std::collections::HashMap;

use glam::UVec2;
use palette::LinSrgb;
use sdl3::video::Window;
use wgpu::{
	BlendState, Buffer, ColorTargetState, ColorWrites, CompositeAlphaMode, Device,
	DeviceDescriptor, FragmentState, IndexFormat, Instance, LoadOp, MultisampleState, Operations,
	PipelineCompilationOptions, PipelineLayout, PipelineLayoutDescriptor, PowerPreference,
	PrimitiveState, Queue, RenderPassColorAttachment, RenderPassDescriptor, RenderPipeline,
	RenderPipelineDescriptor, RequestAdapterOptions, StoreOp, Surface, SurfaceConfiguration,
	SurfaceTargetUnsafe, TextureUsages, TextureViewDescriptor, VertexBufferLayout, VertexState,
	VertexStepMode,
};

use crate::{
	ContextSettings,
	color::{ColorConstants, lin_srgb_to_wgpu_color},
	graphics::{HasVertexAttributes, Shader, Vertex2d},
};

const DEFAULT_SHADER_SOURCE: &str = include_str!("shader.glsl");

pub(crate) struct GraphicsContext {
	pub(crate) device: Device,
	pub(crate) queue: Queue,
	config: SurfaceConfiguration,
	surface: Surface<'static>,
	pub(crate) default_shader: Shader,
	pub(crate) clear_color: LinSrgb,
	pipeline_layout: PipelineLayout,
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
		let default_shader = Shader::new_internal(&device, "Default Shader", DEFAULT_SHADER_SOURCE);
		let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor::default());
		Self {
			device,
			queue,
			config,
			surface,
			default_shader,
			clear_color: LinSrgb::BLACK,
			pipeline_layout,
			render_pipelines: HashMap::new(),
			main_surface_draw_commands: vec![],
		}
	}

	pub(crate) fn resize(&mut self, size: UVec2) {
		self.config.width = size.x;
		self.config.height = size.y;
		self.surface.configure(&self.device, &self.config);
	}

	pub fn queue_draw_command(&mut self, draw_command: DrawCommand) {
		self.main_surface_draw_commands.push(draw_command);
	}

	pub(crate) fn present(&mut self) {
		self.create_render_pipelines();
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
				depth_stencil_attachment: None,
				timestamp_writes: None,
				occlusion_query_set: None,
			});
			for DrawCommand {
				vertex_buffer,
				index_buffer,
				num_indices,
				render_pipeline_settings,
			} in self.main_surface_draw_commands.drain(..)
			{
				render_pass.set_pipeline(&self.render_pipelines[&render_pipeline_settings]);
				render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
				render_pass.set_index_buffer(index_buffer.slice(..), IndexFormat::Uint32);
				render_pass.draw_indexed(0..num_indices, 0, 0..1);
			}
		}
		self.queue.submit([encoder.finish()]);
		frame.present();
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct DrawCommand {
	pub(crate) vertex_buffer: Buffer,
	pub(crate) index_buffer: Buffer,
	pub(crate) num_indices: u32,
	pub(crate) render_pipeline_settings: RenderPipelineSettings,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct RenderPipelineSettings {
	pub(crate) shader: Shader,
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
			module: &settings.shader.vertex,
			entry_point: Some("main"),
			compilation_options: PipelineCompilationOptions::default(),
			buffers: &[VertexBufferLayout {
				array_stride: std::mem::size_of::<Vertex2d>() as u64,
				step_mode: VertexStepMode::Vertex,
				attributes: &Vertex2d::attributes(),
			}],
		},
		primitive: PrimitiveState::default(),
		depth_stencil: None,
		multisample: MultisampleState::default(),
		fragment: Some(FragmentState {
			module: &settings.shader.fragment,
			entry_point: Some("main"),
			compilation_options: PipelineCompilationOptions::default(),
			targets: &[Some(ColorTargetState {
				format: config.format,
				blend: Some(BlendState::REPLACE),
				write_mask: ColorWrites::ALL,
			})],
		}),
		multiview: None,
		cache: None,
	})
}
