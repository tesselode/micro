use glam::{UVec2, Vec2};
use sdl2::video::Window;
use wgpu::{
	util::{BufferInitDescriptor, DeviceExt},
	BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingType, BlendState, Buffer, BufferUsages,
	ColorTargetState, ColorWrites, CommandEncoderDescriptor, Device, DeviceDescriptor,
	FragmentState, IndexFormat, Instance, InstanceDescriptor, LoadOp, MultisampleState, Operations,
	PipelineLayoutDescriptor, PrimitiveState, Queue, RenderPassColorAttachment,
	RenderPassDescriptor, RenderPipeline, RenderPipelineDescriptor, RequestAdapterOptions,
	SamplerBindingType, ShaderStages, Surface, SurfaceConfiguration, SurfaceError,
	TextureSampleType, TextureUsages, TextureViewDescriptor, TextureViewDimension, VertexState,
};

use crate::{
	graphics::{color::Rgba, image_data::ImageData, texture::Texture, Vertex},
	InitGraphicsError,
};

const VERTICES: &[Vertex] = &[
	Vertex {
		position: Vec2::new(-0.0868241, 0.49240386),
		texture_coords: Vec2::new(0.4131759, 0.99240386),
		color: Rgba::WHITE,
	},
	Vertex {
		position: Vec2::new(-0.49513406, 0.06958647),
		texture_coords: Vec2::new(0.0048659444, 0.56958647),
		color: Rgba::WHITE,
	},
	Vertex {
		position: Vec2::new(-0.21918549, -0.44939706),
		texture_coords: Vec2::new(0.28081453, 0.05060294),
		color: Rgba::WHITE,
	},
	Vertex {
		position: Vec2::new(0.35966998, -0.3473291),
		texture_coords: Vec2::new(0.85967, 0.1526709),
		color: Rgba::WHITE,
	},
	Vertex {
		position: Vec2::new(0.44147372, 0.2347359),
		texture_coords: Vec2::new(0.9414737, 0.7347359),
		color: Rgba::WHITE,
	},
];

const INDICES: &[u16] = &[0, 1, 4, 1, 2, 4, 2, 3, 4];

pub struct GraphicsContext {
	surface: Surface,
	device: Device,
	queue: Queue,
	config: SurfaceConfiguration,
	render_pipeline: RenderPipeline,
	vertex_buffer: Buffer,
	index_buffer: Buffer,
	texture: Texture,
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
			render_pass.set_bind_group(0, &self.texture.0.bind_group, &[]);
			render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
			render_pass.set_index_buffer(self.index_buffer.slice(..), IndexFormat::Uint16);
			render_pass.draw_indexed(0..INDICES.len() as u32, 0, 0..1);
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
			usage: TextureUsages::RENDER_ATTACHMENT,
			format: surface_format,
			width: size.0,
			height: size.1,
			present_mode: surface_capabilities.present_modes[0],
			alpha_mode: surface_capabilities.alpha_modes[0],
			view_formats: vec![],
		};
		surface.configure(&device, &config);
		let shader = device.create_shader_module(wgpu::include_wgsl!("../graphics/shader.wgsl"));
		let texture_bind_group_layout =
			device.create_bind_group_layout(&BindGroupLayoutDescriptor {
				entries: &[
					BindGroupLayoutEntry {
						binding: 0,
						visibility: ShaderStages::FRAGMENT,
						ty: BindingType::Texture {
							multisampled: false,
							view_dimension: TextureViewDimension::D2,
							sample_type: TextureSampleType::Float { filterable: true },
						},
						count: None,
					},
					BindGroupLayoutEntry {
						binding: 1,
						visibility: ShaderStages::FRAGMENT,
						ty: BindingType::Sampler(SamplerBindingType::Filtering),
						count: None,
					},
				],
				label: Some("texture_bind_group_layout"),
			});
		let render_pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
			label: Some("Render Pipeline Layout"),
			bind_group_layouts: &[&texture_bind_group_layout],
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
		let index_buffer = device.create_buffer_init(&BufferInitDescriptor {
			label: Some("Index Buffer"),
			contents: bytemuck::cast_slice(INDICES),
			usage: BufferUsages::INDEX,
		});
		let image_data = ImageData::load("crates/micro/examples/tree.png").unwrap();
		let texture = Texture::from_image_data_internal(
			&image_data,
			&device,
			&queue,
			&texture_bind_group_layout,
		);
		Ok(Self {
			surface,
			device,
			queue,
			config,
			render_pipeline,
			vertex_buffer,
			index_buffer,
			texture,
		})
	}
}
