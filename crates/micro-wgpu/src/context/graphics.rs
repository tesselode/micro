use bytemuck::{Pod, Zeroable};
use glam::{Mat4, UVec2, vec3};
use palette::{LinSrgb, LinSrgba};
use sdl2::video::Window;
use wgpu::{
	BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor,
	BindGroupLayoutEntry, BindingResource, BindingType, Buffer, BufferBindingType, BufferUsages,
	CompositeAlphaMode, Device, DeviceDescriptor, IndexFormat, Instance, LoadOp, Operations,
	PowerPreference, PresentMode, Queue, RenderPassColorAttachment, RenderPassDescriptor,
	RenderPipeline, RequestAdapterOptions, SamplerBindingType, ShaderStages, StoreOp, Surface,
	SurfaceConfiguration, SurfaceTargetUnsafe, TextureSampleType, TextureUsages,
	TextureViewDescriptor, TextureViewDimension,
	util::{BufferInitDescriptor, DeviceExt},
};

use crate::{
	color::{ColorConstants, lin_srgb_to_wgpu_color},
	graphics::{
		Vertex2d,
		graphics_pipeline::GraphicsPipeline,
		texture::{Texture, TextureSettings},
	},
};

pub(crate) struct GraphicsContext<'window> {
	pub(crate) device: Device,
	pub(crate) queue: Queue,
	config: SurfaceConfiguration,
	surface: Surface<'window>,
	pub(crate) mesh_bind_group_layout: BindGroupLayout,
	default_render_pipeline: RenderPipeline,
	default_texture: Texture,
	pub(crate) clear_color: LinSrgb,
	pub(crate) transform_stack: Vec<Mat4>,
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
		let mesh_bind_group_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
			label: None,
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
			GraphicsPipeline::<Vertex2d>::new_internal(&device, &mesh_bind_group_layout)
				.render_pipeline;
		let default_texture = Texture::new(
			&device,
			&queue,
			UVec2::new(1, 1),
			Some(&[255, 255, 255, 255]),
			TextureSettings::default(),
		);
		Self {
			device,
			queue,
			config,
			surface,
			mesh_bind_group_layout,
			default_render_pipeline,
			default_texture,
			clear_color: LinSrgb::BLACK,
			transform_stack: vec![],
			draw_commands: vec![],
		}
	}

	pub(crate) fn global_transform(&self) -> Mat4 {
		let coordinate_system_transform = Mat4::from_translation(vec3(-1.0, 1.0, 0.0))
			* Mat4::from_scale(vec3(
				2.0 / self.config.width as f32,
				-2.0 / self.config.height as f32,
				1.0,
			));
		self.transform_stack
			.iter()
			.fold(coordinate_system_transform, |previous, transform| {
				previous * *transform
			})
	}

	pub(crate) fn queue_draw_command(&mut self, mut draw_command: DrawCommand) {
		draw_command.draw_params.transform =
			self.global_transform() * draw_command.draw_params.transform;
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
						load: LoadOp::Clear(lin_srgb_to_wgpu_color(self.clear_color)),
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
				texture,
				draw_params,
			} in self.draw_commands.drain(..)
			{
				let draw_params_buffer = self.device.create_buffer_init(&BufferInitDescriptor {
					label: Some("Draw Params Buffer"),
					contents: bytemuck::cast_slice(&[draw_params]),
					usage: BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
				});
				let texture = texture.unwrap_or(self.default_texture.clone());
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
				render_pass.set_pipeline(
					render_pipeline
						.as_ref()
						.unwrap_or(&self.default_render_pipeline),
				);
				render_pass.set_bind_group(0, &mesh_bind_group, &[]);
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
	pub(crate) texture: Option<Texture>,
	pub(crate) draw_params: DrawParams,
}

#[derive(Debug, Clone, Copy, PartialEq, Pod, Zeroable)]
#[repr(C)]
pub(crate) struct DrawParams {
	pub transform: Mat4,
	pub color: LinSrgba,
}
