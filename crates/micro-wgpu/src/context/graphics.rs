use bytemuck::{Pod, Zeroable};
use glam::{Mat4, UVec2, Vec3, uvec2};
use palette::{LinSrgb, LinSrgba};
use sdl2::video::Window;
use wgpu::{
	BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor,
	BindGroupLayoutEntry, BindingResource, BindingType, Buffer, BufferBindingType, BufferUsages,
	CompositeAlphaMode, Device, DeviceDescriptor, Features, IndexFormat, Instance, LoadOp,
	Operations, PowerPreference, PresentMode, Queue, RenderPassColorAttachment,
	RenderPassDescriptor, RenderPipeline, RequestAdapterOptions, SamplerBindingType, ShaderStages,
	StoreOp, Surface, SurfaceConfiguration, SurfaceTargetUnsafe, TextureFormat, TextureSampleType,
	TextureUsages, TextureViewDescriptor, TextureViewDimension,
	util::{BufferInitDescriptor, DeviceExt},
};

use crate::{
	color::{ColorConstants, lin_srgb_to_wgpu_color, lin_srgba_to_wgpu_color},
	graphics::{
		Vertex2d,
		canvas::{Canvas, CanvasKind, RenderToCanvasSettings},
		graphics_pipeline::{GraphicsPipeline, GraphicsPipelineSettings},
		texture::{InternalTextureSettings, Texture, TextureSettings},
	},
};

pub(crate) struct GraphicsContext<'window> {
	pub(crate) device: Device,
	pub(crate) queue: Queue,
	pub(crate) supported_sample_counts: Vec<u32>,
	config: SurfaceConfiguration,
	surface: Surface<'window>,
	pub(crate) mesh_bind_group_layout: BindGroupLayout,
	pub(crate) render_pipeline_stack: Vec<RenderPipeline>,
	default_texture: Texture,
	pub(crate) clear_color: LinSrgb,
	pub(crate) transform_stack: Vec<Mat4>,
	main_surface_draw_commands: Vec<DrawCommand>,
	current_canvas_render_pass: Option<CanvasRenderPass>,
	finished_canvas_render_passes: Vec<CanvasRenderPass>,
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
		let supported_sample_counts = adapter
			.get_texture_format_features(TextureFormat::Rgba8UnormSrgb)
			.flags
			.supported_sample_counts();
		let (device, queue) = pollster::block_on(adapter.request_device(
			&DeviceDescriptor {
				required_features: Features::TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES,
				..Default::default()
			},
			None,
		))
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
		let default_render_pipeline = GraphicsPipeline::<Vertex2d>::new_internal(
			&device,
			&mesh_bind_group_layout,
			GraphicsPipelineSettings::default(),
		)
		.render_pipeline;
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
			mesh_bind_group_layout,
			render_pipeline_stack: vec![default_render_pipeline],
			default_texture,
			clear_color: LinSrgb::BLACK,
			transform_stack: vec![],
			main_surface_draw_commands: vec![],
			current_canvas_render_pass: None,
			finished_canvas_render_passes: vec![],
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

	pub(crate) fn global_transform(&self) -> Mat4 {
		let draw_target_size =
			if let Some(CanvasRenderPass { canvas, .. }) = &self.current_canvas_render_pass {
				canvas.size()
			} else {
				uvec2(self.config.width, self.config.height)
			};
		let coordinate_system_transform = Mat4::from_translation(Vec3::new(-1.0, 1.0, 0.0))
			* Mat4::from_scale(Vec3::new(
				2.0 / draw_target_size.x as f32,
				-2.0 / draw_target_size.y as f32,
				1.0,
			));
		self.transform_stack
			.iter()
			.fold(coordinate_system_transform, |previous, transform| {
				previous * *transform
			})
	}

	pub(crate) fn queue_draw_command(&mut self, mut settings: QueueDrawCommandSettings) {
		settings.draw_params.transform = self.global_transform() * settings.draw_params.transform;
		let command = DrawCommand {
			vertex_buffer: settings.vertex_buffer,
			index_buffer: settings.index_buffer,
			num_indices: settings.num_indices,
			render_pipeline: self.render_pipeline_stack.last().unwrap().clone(),
			texture: settings.texture,
			draw_params: settings.draw_params,
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
				label: None,
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
				depth_stencil_attachment: None,
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
}

pub(crate) struct QueueDrawCommandSettings {
	pub(crate) vertex_buffer: Buffer,
	pub(crate) index_buffer: Buffer,
	pub(crate) num_indices: u32,
	pub(crate) texture: Option<Texture>,
	pub(crate) draw_params: DrawParams,
}

#[derive(Debug, Clone, Copy, PartialEq, Pod, Zeroable)]
#[repr(C)]
pub(crate) struct DrawParams {
	pub transform: Mat4,
	pub color: LinSrgba,
}

struct DrawCommand {
	vertex_buffer: Buffer,
	index_buffer: Buffer,
	num_indices: u32,
	render_pipeline: RenderPipeline,
	texture: Option<Texture>,
	draw_params: DrawParams,
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
		num_indices,
		render_pipeline,
		texture,
		draw_params,
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
		render_pass.set_pipeline(&render_pipeline);
		render_pass.set_bind_group(0, &mesh_bind_group, &[]);
		render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
		render_pass.set_index_buffer(index_buffer.slice(..), IndexFormat::Uint32);
		render_pass.draw_indexed(0..num_indices, 0, 0..1);
	}
}
