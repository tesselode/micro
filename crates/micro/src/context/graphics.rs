use std::rc::Rc;

use glam::{Mat4, UVec2, Vec2, Vec3};
use sdl2::video::Window;
use wgpu::{
	util::{BufferInitDescriptor, DeviceExt},
	BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor,
	BindGroupLayoutEntry, BindingType, BufferBindingType, BufferUsages, CommandEncoderDescriptor,
	Device, DeviceDescriptor, IndexFormat, Instance, InstanceDescriptor, LoadOp, Operations,
	PipelineLayout, PipelineLayoutDescriptor, Queue, RenderPassColorAttachment,
	RenderPassDescriptor, RequestAdapterOptions, SamplerBindingType, ShaderStages, Surface,
	SurfaceConfiguration, SurfaceError, TextureSampleType, TextureUsages, TextureViewDescriptor,
	TextureViewDimension,
};

use crate::{
	graphics::{
		draw_params::DrawParamsUniform,
		graphics_pipeline::{GraphicsPipeline, GraphicsPipelineInner, GraphicsPipelineSettings},
		image_data::ImageData,
		mesh::Mesh,
		shader::{DefaultShader, Shader},
		texture::Texture,
		DrawParams,
	},
	InitGraphicsError, OffsetAndCount,
};

pub struct GraphicsContext {
	surface: Surface,
	pub(crate) device: Device,
	pub(crate) queue: Queue,
	pub(crate) config: SurfaceConfiguration,
	pub(crate) texture_bind_group_layout: BindGroupLayout,
	pub(crate) draw_params_bind_group_layout: BindGroupLayout,
	pub(crate) shader_params_bind_group_layout: BindGroupLayout,
	pub(crate) render_pipeline_layout: PipelineLayout,
	default_graphics_pipeline: Rc<GraphicsPipelineInner>,
	draw_instructions: Vec<DrawInstruction>,
	pub(crate) default_texture: Texture,
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
			for DrawInstruction {
				mesh,
				texture,
				range,
				draw_params_bind_group,
				graphics_pipeline,
			} in &self.draw_instructions
			{
				render_pass.set_pipeline(&graphics_pipeline.render_pipeline);
				render_pass.set_bind_group(0, &texture.0.bind_group, &[]);
				render_pass.set_bind_group(1, draw_params_bind_group, &[]);
				render_pass.set_bind_group(2, &graphics_pipeline.shader_params_bind_group, &[]);
				render_pass.set_vertex_buffer(0, mesh.0.vertex_buffer.slice(..));
				render_pass.set_index_buffer(mesh.0.index_buffer.slice(..), IndexFormat::Uint32);
				render_pass.draw_indexed(range.offset..(range.offset + range.count), 0, 0..1);
			}
		}
		self.queue.submit(std::iter::once(encoder.finish()));
		output.present();
		self.draw_instructions.clear();
		Ok(())
	}

	pub(crate) fn push_instruction<S: Shader>(
		&mut self,
		mesh: Mesh,
		texture: Texture,
		range: OffsetAndCount,
		draw_params: DrawParams<S>,
	) {
		let draw_params_uniform = draw_params.as_uniform();
		let graphics_pipeline = draw_params
			.graphics_pipeline
			.map(|pipeline| pipeline.inner)
			.unwrap_or(self.default_graphics_pipeline.clone());
		self.push_instruction_inner(mesh, texture, range, draw_params_uniform, graphics_pipeline);
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
		let draw_params_bind_group_layout =
			device.create_bind_group_layout(&BindGroupLayoutDescriptor {
				label: Some("Draw Params Bind Group Layout"),
				entries: &[BindGroupLayoutEntry {
					binding: 0,
					visibility: ShaderStages::VERTEX,
					ty: BindingType::Buffer {
						ty: BufferBindingType::Uniform,
						has_dynamic_offset: false,
						min_binding_size: None,
					},
					count: None,
				}],
			});
		let shader_params_bind_group_layout =
			device.create_bind_group_layout(&BindGroupLayoutDescriptor {
				label: Some("Shader Params Bind Group Layout"),
				entries: &[BindGroupLayoutEntry {
					binding: 0,
					visibility: ShaderStages::VERTEX | ShaderStages::FRAGMENT,
					ty: BindingType::Buffer {
						ty: BufferBindingType::Uniform,
						has_dynamic_offset: false,
						min_binding_size: None,
					},
					count: None,
				}],
			});
		let render_pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
			label: Some("Render Pipeline Layout"),
			bind_group_layouts: &[
				&texture_bind_group_layout,
				&draw_params_bind_group_layout,
				&shader_params_bind_group_layout,
			],
			push_constant_ranges: &[],
		});
		let default_graphics_pipeline = GraphicsPipeline::new_internal(
			GraphicsPipelineSettings::<DefaultShader> { shader_params: 0 },
			&device,
			&render_pipeline_layout,
			&shader_params_bind_group_layout,
			&config,
		)
		.inner;
		let default_texture = Texture::new_internal(
			Some(&ImageData {
				size: UVec2::ONE,
				pixels: vec![255, 255, 255, 255],
			}),
			UVec2::ONE,
			&device,
			&queue,
			&texture_bind_group_layout,
		);
		Ok(Self {
			surface,
			device,
			queue,
			config,
			texture_bind_group_layout,
			draw_params_bind_group_layout,
			shader_params_bind_group_layout,
			render_pipeline_layout,
			default_graphics_pipeline,
			draw_instructions: vec![],
			default_texture,
		})
	}

	fn push_instruction_inner(
		&mut self,
		mesh: Mesh,
		texture: Texture,
		range: OffsetAndCount,
		mut draw_params_uniform: DrawParamsUniform,
		graphics_pipeline: Rc<GraphicsPipelineInner>,
	) {
		draw_params_uniform.transform = self.global_transform() * draw_params_uniform.transform;
		let draw_params_buffer = self.device.create_buffer_init(&BufferInitDescriptor {
			label: Some("Draw Params Buffer"),
			contents: bytemuck::cast_slice(&[draw_params_uniform]),
			usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
		});
		let draw_params_bind_group = self.device.create_bind_group(&BindGroupDescriptor {
			label: Some("Draw Params Bind Group"),
			entries: &[BindGroupEntry {
				binding: 0,
				resource: draw_params_buffer.as_entire_binding(),
			}],
			layout: &self.draw_params_bind_group_layout,
		});
		self.draw_instructions.push(DrawInstruction {
			mesh,
			texture,
			range,
			draw_params_bind_group,
			graphics_pipeline,
		});
	}

	fn global_transform(&self) -> Mat4 {
		let screen_size = Vec2::new(self.config.width as f32, self.config.height as f32);
		Mat4::from_translation(Vec3::new(-1.0, 1.0, 0.0))
			* Mat4::from_scale(Vec3::new(2.0 / screen_size.x, -2.0 / screen_size.y, 1.0))
	}
}

struct DrawInstruction {
	mesh: Mesh,
	texture: Texture,
	range: OffsetAndCount,
	draw_params_bind_group: BindGroup,
	graphics_pipeline: Rc<GraphicsPipelineInner>,
}
