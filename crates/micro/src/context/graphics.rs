use glam::{UVec2, Vec2};
use sdl2::video::Window;
use wgpu::{
	util::{BufferInitDescriptor, DeviceExt},
	BlendState, Buffer, BufferUsages, ColorTargetState, ColorWrites, CommandEncoderDescriptor,
	Device, DeviceDescriptor, FragmentState, Instance, InstanceDescriptor, LoadOp,
	MultisampleState, Operations, PipelineLayoutDescriptor, PrimitiveState, Queue,
	RenderPassColorAttachment, RenderPassDescriptor, RenderPipeline, RenderPipelineDescriptor,
	RequestAdapterOptions, Surface, SurfaceConfiguration, SurfaceError, TextureViewDescriptor,
	VertexState,
};

use crate::{
	graphics::{color::Rgba, Vertex},
	InitGraphicsError,
};

const VERTICES: &[Vertex] = &[
	Vertex {
		position: Vec2::new(0.0, 0.5),
		color: Rgba::RED,
	},
	Vertex {
		position: Vec2::new(-0.5, -0.5),
		color: Rgba::GREEN,
	},
	Vertex {
		position: Vec2::new(0.5, -0.5),
		color: Rgba::BLUE,
	},
];

pub struct GraphicsContext {
	surface: Surface,
	device: Device,
	queue: Queue,
	config: SurfaceConfiguration,
	render_pipeline: RenderPipeline,
	vertex_buffer: Buffer,
}

impl GraphicsContext {
	pub fn new(window: &Window) -> Result<Self, InitGraphicsError> {
		pollster::block_on(Self::new_inner(window))
	}

	pub fn resize(&mut self, size: UVec2) {
		self.config.width = size.x;
		self.config.height = size.y;
		self.surface.configure(&self.device, &self.config);
	}

	pub fn render(&mut self) -> Result<(), SurfaceError> {
		let output = self.surface.get_current_texture()?;
		let view = output
			.texture
			.create_view(&TextureViewDescriptor::default());
		let mut encoder = self
			.device
			.create_command_encoder(&CommandEncoderDescriptor {
				label: Some("Render Encoder"),
			});
		{
			let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
				label: Some("Render Pass"),
				color_attachments: &[Some(RenderPassColorAttachment {
					view: &view,
					resolve_target: None,
					ops: Operations {
						load: LoadOp::Clear(wgpu::Color {
							r: 0.1,
							g: 0.2,
							b: 0.3,
							a: 1.0,
						}),
						store: true,
					},
				})],
				depth_stencil_attachment: None,
			});
			render_pass.set_pipeline(&self.render_pipeline);
			render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
			render_pass.draw(0..VERTICES.len() as u32, 0..1);
		}
		self.queue.submit(std::iter::once(encoder.finish()));
		output.present();
		Ok(())
	}

	async fn new_inner(window: &Window) -> Result<Self, InitGraphicsError> {
		let size = window.size();
		let instance = Instance::new(InstanceDescriptor::default());
		let surface = unsafe { instance.create_surface(window)? };
		let adapter = instance
			.request_adapter(&RequestAdapterOptions {
				compatible_surface: Some(&surface),
				..Default::default()
			})
			.await
			.ok_or(InitGraphicsError::NoAdapterFound)?;
		let (device, queue) = adapter
			.request_device(
				&DeviceDescriptor::default(),
				None, // Trace path
			)
			.await?;
		let surface_capabilities = surface.get_capabilities(&adapter);
		let surface_format = surface_capabilities
			.formats
			.iter()
			.copied()
			.find(|f| f.describe().srgb)
			.unwrap_or(surface_capabilities.formats[0]);
		let config = SurfaceConfiguration {
			usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
			format: surface_format,
			width: size.0,
			height: size.1,
			present_mode: surface_capabilities.present_modes[0],
			alpha_mode: surface_capabilities.alpha_modes[0],
			view_formats: vec![],
		};
		surface.configure(&device, &config);
		let shader = device.create_shader_module(wgpu::include_wgsl!("../graphics/shader.wgsl"));
		let render_pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
			label: Some("Render Pipeline Layout"),
			bind_group_layouts: &[],
			push_constant_ranges: &[],
		});
		let render_pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
			label: Some("Render Pipeline"),
			layout: Some(&render_pipeline_layout),
			vertex: VertexState {
				module: &shader,
				entry_point: "vs_main",
				buffers: &[Vertex::buffer_layout()],
			},
			fragment: Some(FragmentState {
				module: &shader,
				entry_point: "fs_main",
				targets: &[Some(ColorTargetState {
					format: config.format,
					blend: Some(BlendState::REPLACE),
					write_mask: ColorWrites::ALL,
				})],
			}),
			primitive: PrimitiveState::default(),
			depth_stencil: None,
			multisample: MultisampleState::default(),
			multiview: None,
		});
		let vertex_buffer = device.create_buffer_init(&BufferInitDescriptor {
			label: Some("Vertex Buffer"),
			contents: bytemuck::cast_slice(VERTICES),
			usage: BufferUsages::VERTEX,
		});
		Ok(Self {
			surface,
			device,
			queue,
			config,
			render_pipeline,
			vertex_buffer,
		})
	}
}
