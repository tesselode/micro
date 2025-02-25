use glam::UVec2;
use sdl2::video::Window;
use wgpu::{
	BindGroup, BindGroupDescriptor, BindGroupLayoutDescriptor, Buffer, CompositeAlphaMode, Device,
	DeviceDescriptor, IndexFormat, Instance, LoadOp, Operations, PowerPreference, PresentMode,
	Queue, RenderPassColorAttachment, RenderPassDescriptor, RenderPipeline, RequestAdapterOptions,
	StoreOp, Surface, SurfaceConfiguration, SurfaceTargetUnsafe, TextureUsages,
	TextureViewDescriptor,
};

use crate::graphics::{Vertex2d, graphics_pipeline::GraphicsPipeline};

pub(crate) struct GraphicsContext<'window> {
	pub(crate) device: Device,
	queue: Queue,
	bind_group: BindGroup,
	config: SurfaceConfiguration,
	surface: Surface<'window>,
	default_render_pipeline: RenderPipeline,
	draw_commands: Vec<DrawCommand>,
}

impl GraphicsContext<'_> {
	pub(crate) fn new(window: &Window) -> Self {
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
		let (device, queue) =
			pollster::block_on(adapter.request_device(&DeviceDescriptor::default(), None))
				.expect("error getting graphics device");
		let bind_group_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
			label: None,
			entries: &[],
		});
		let bind_group = device.create_bind_group(&BindGroupDescriptor {
			label: None,
			layout: &bind_group_layout,
			entries: &[],
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
			present_mode: PresentMode::AutoVsync,
			desired_maximum_frame_latency: 1,
			alpha_mode: CompositeAlphaMode::Auto,
			view_formats: vec![],
		};
		surface.configure(&device, &config);
		let default_render_pipeline =
			GraphicsPipeline::<Vertex2d>::new_from_device(&device).render_pipeline;
		Self {
			device,
			queue,
			bind_group,
			config,
			surface,
			default_render_pipeline,
			draw_commands: vec![],
		}
	}

	pub(crate) fn queue_draw_command(&mut self, draw_command: DrawCommand) {
		self.draw_commands.push(draw_command);
	}

	pub(crate) fn resize(&mut self, size: UVec2) {
		self.config.width = size.x;
		self.config.height = size.y;
		self.surface.configure(&self.device, &self.config);
	}

	pub(crate) fn present(&mut self) {
		let frame = self
			.surface
			.get_current_texture()
			.expect("error getting surface texture");
		let output = frame.texture.create_view(&TextureViewDescriptor::default());
		let mut encoder = self.device.create_command_encoder(&Default::default());

		{
			let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
				label: None,
				color_attachments: &[Some(RenderPassColorAttachment {
					view: &output,
					resolve_target: None,
					ops: Operations {
						load: LoadOp::Clear(wgpu::Color::GREEN),
						store: StoreOp::Store,
					},
				})],
				depth_stencil_attachment: None,
				timestamp_writes: None,
				occlusion_query_set: None,
			});
			for DrawCommand {
				vertex_buffer,
				index_buffer,
				num_indices,
				render_pipeline,
			} in self.draw_commands.drain(..)
			{
				render_pass.set_pipeline(
					render_pipeline
						.as_ref()
						.unwrap_or(&self.default_render_pipeline),
				);
				render_pass.set_bind_group(0, &self.bind_group, &[]);
				render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
				render_pass.set_index_buffer(index_buffer.slice(..), IndexFormat::Uint32);
				render_pass.draw_indexed(0..num_indices, 0, 0..1);
			}
		}

		self.queue.submit([encoder.finish()]);
		frame.present();
	}
}

pub(crate) struct DrawCommand {
	pub(crate) vertex_buffer: Buffer,
	pub(crate) index_buffer: Buffer,
	pub(crate) num_indices: u32,
	pub(crate) render_pipeline: Option<RenderPipeline>,
}
