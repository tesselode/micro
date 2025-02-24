use glam::UVec2;
use sdl2::video::Window;
use wgpu::{
	BindGroup, BindGroupDescriptor, BindGroupLayoutDescriptor, ColorTargetState, ColorWrites,
	CompositeAlphaMode, Device, DeviceDescriptor, FragmentState, Instance, LoadOp,
	MultisampleState, Operations, PipelineCompilationOptions, PipelineLayoutDescriptor,
	PowerPreference, PresentMode, PrimitiveState, PrimitiveTopology, Queue,
	RenderPassColorAttachment, RenderPassDescriptor, RenderPipeline, RenderPipelineDescriptor,
	RequestAdapterOptions, StoreOp, Surface, SurfaceConfiguration, SurfaceTargetUnsafe,
	TextureFormat, TextureUsages, TextureViewDescriptor, VertexState, include_wgsl,
};

pub(crate) struct GraphicsContext<'window> {
	device: Device,
	queue: Queue,
	render_pipeline: RenderPipeline,
	bind_group: BindGroup,
	config: SurfaceConfiguration,
	surface: Surface<'window>,
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
		let shader = device.create_shader_module(include_wgsl!("shader.wgsl"));
		let bind_group_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
			label: None,
			entries: &[],
		});
		let bind_group = device.create_bind_group(&BindGroupDescriptor {
			label: None,
			layout: &bind_group_layout,
			entries: &[],
		});
		let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
			label: None,
			bind_group_layouts: &[&bind_group_layout],
			push_constant_ranges: &[],
		});
		let render_pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
			label: None,
			layout: Some(&pipeline_layout),
			vertex: VertexState {
				module: &shader,
				entry_point: Some("vs_main"),
				compilation_options: PipelineCompilationOptions::default(),
				buffers: &[],
			},
			primitive: PrimitiveState {
				topology: PrimitiveTopology::TriangleList,
				..Default::default()
			},
			depth_stencil: None,
			multisample: MultisampleState::default(),
			fragment: Some(FragmentState {
				module: &shader,
				entry_point: Some("fs_main"),
				compilation_options: PipelineCompilationOptions::default(),
				targets: &[Some(ColorTargetState {
					format: TextureFormat::Rgba8UnormSrgb,
					blend: None,
					write_mask: ColorWrites::ALL,
				})],
			}),
			multiview: None,
			cache: None,
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
		Self {
			device,
			queue,
			render_pipeline,
			bind_group,
			config,
			surface,
		}
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
			render_pass.set_pipeline(&self.render_pipeline);
			render_pass.set_bind_group(0, &self.bind_group, &[]);
			render_pass.draw(0..3, 0..1);
		}

		self.queue.submit([encoder.finish()]);
		frame.present();
	}
}
