use std::rc::Rc;

use glam::{Mat4, UVec2, Vec3};
use sdl2::video::Window;
use wgpu::{
	util::{BufferInitDescriptor, DeviceExt},
	BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor,
	BindGroupLayoutEntry, BindingType, BufferBindingType, BufferUsages, CommandEncoderDescriptor,
	Device, DeviceDescriptor, IndexFormat, Instance, InstanceDescriptor, LoadOp, Operations,
	PipelineLayout, PipelineLayoutDescriptor, Queue, RenderPassColorAttachment,
	RenderPassDepthStencilAttachment, RenderPassDescriptor, RequestAdapterOptions,
	SamplerBindingType, ShaderStages, Surface, SurfaceConfiguration, SurfaceError,
	TextureSampleType, TextureUsages, TextureView, TextureViewDescriptor, TextureViewDimension,
};

use crate::{
	graphics::{
		canvas::Canvas,
		color::Rgba,
		draw_params::DrawParamsUniform,
		graphics_pipeline::{GraphicsPipeline, GraphicsPipelineInner, GraphicsPipelineSettings},
		image_data::ImageData,
		mesh::{Mesh, MeshTexture},
		shader::{DefaultShader, Shader},
		texture::Texture,
		util::create_depth_stencil_texture_view,
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
	draw_instruction_sets: Vec<DrawInstructionSet>,
	pub(crate) default_texture: Texture,
	depth_stencil_texture_view: TextureView,
}

impl GraphicsContext {
	pub fn new(window: &Window) -> Result<Self, InitGraphicsError> {
		pollster::block_on(Self::new_inner(window))
	}

	pub fn resize(&mut self, size: UVec2) {
		self.config.width = size.x;
		self.config.height = size.y;
		self.surface.configure(&self.device, &self.config);
		self.depth_stencil_texture_view = create_depth_stencil_texture_view(size, &self.device);
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
		for DrawInstructionSet {
			kind,
			clear_color,
			clear_stencil_value,
			instructions,
		} in self.draw_instruction_sets.drain(..)
		{
			let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
				label: Some("Render Pass"),
				color_attachments: &[Some(RenderPassColorAttachment {
					view: match &kind {
						DrawInstructionSetKind::Surface => &view,
						DrawInstructionSetKind::Canvas(canvas) => &canvas.0.view,
					},
					resolve_target: None,
					ops: Operations {
						load: match clear_color {
							Some(color) => LoadOp::Clear(color.to_wgpu_color()),
							None => LoadOp::Load,
						},
						store: true,
					},
				})],
				depth_stencil_attachment: Some(RenderPassDepthStencilAttachment {
					view: match &kind {
						DrawInstructionSetKind::Surface => &self.depth_stencil_texture_view,
						DrawInstructionSetKind::Canvas(canvas) => {
							&canvas.0.depth_stencil_texture_view
						}
					},
					depth_ops: None,
					stencil_ops: Some(Operations {
						load: match clear_stencil_value {
							Some(clear_stencil_value) => LoadOp::Clear(clear_stencil_value),
							None => LoadOp::Load,
						},
						store: true,
					}),
				}),
			});
			for DrawInstruction {
				mesh,
				texture,
				range,
				draw_params_bind_group,
				graphics_pipeline,
				stencil_reference,
			} in &instructions
			{
				let texture_bind_group = match texture {
					MeshTexture::Texture(texture) => &texture.0.bind_group,
					MeshTexture::Canvas(canvas) => &canvas.0.bind_group,
				};
				render_pass.set_pipeline(&graphics_pipeline.render_pipeline);
				render_pass.set_bind_group(0, texture_bind_group, &[]);
				render_pass.set_bind_group(1, draw_params_bind_group, &[]);
				render_pass.set_bind_group(2, &graphics_pipeline.shader_params_bind_group, &[]);
				render_pass.set_vertex_buffer(0, mesh.0.vertex_buffer.slice(..));
				render_pass.set_index_buffer(mesh.0.index_buffer.slice(..), IndexFormat::Uint32);
				render_pass.set_stencil_reference(*stencil_reference);
				render_pass.draw_indexed(range.offset..(range.offset + range.count), 0, 0..1);
			}
		}
		self.queue.submit(std::iter::once(encoder.finish()));
		output.present();
		self.draw_instruction_sets.push(DrawInstructionSet {
			kind: DrawInstructionSetKind::Surface,
			clear_color: Some(Rgba::BLACK),
			clear_stencil_value: Some(0),
			instructions: vec![],
		});
		Ok(())
	}

	pub(crate) fn set_render_target_to_surface(&mut self) {
		self.draw_instruction_sets.push(DrawInstructionSet {
			kind: DrawInstructionSetKind::Surface,
			clear_color: None,
			clear_stencil_value: None,
			instructions: vec![],
		});
	}

	pub(crate) fn set_render_target_to_canvas(
		&mut self,
		canvas: Canvas,
		clear_color: Option<Rgba>,
		clear_stencil_value: Option<u32>,
	) {
		self.draw_instruction_sets.push(DrawInstructionSet {
			kind: DrawInstructionSetKind::Canvas(canvas),
			clear_color,
			clear_stencil_value,
			instructions: vec![],
		});
	}

	pub(crate) fn push_instruction<S: Shader>(
		&mut self,
		mesh: Mesh,
		texture: MeshTexture,
		range: OffsetAndCount,
		draw_params: DrawParams<S>,
	) {
		let draw_params_uniform = draw_params.as_uniform();
		let graphics_pipeline = draw_params
			.graphics_pipeline
			.map(|pipeline| pipeline.inner)
			.unwrap_or(self.default_graphics_pipeline.clone());
		self.push_instruction_inner(
			mesh,
			texture,
			range,
			draw_params_uniform,
			graphics_pipeline,
			draw_params.stencil_reference,
		);
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
			GraphicsPipelineSettings::<DefaultShader> {
				shader_params: 0,
				..Default::default()
			},
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
		let depth_stencil_texture_view = create_depth_stencil_texture_view(size.into(), &device);
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
			draw_instruction_sets: vec![DrawInstructionSet {
				kind: DrawInstructionSetKind::Surface,
				clear_color: Some(Rgba::BLACK),
				clear_stencil_value: Some(0),
				instructions: vec![],
			}],
			default_texture,
			depth_stencil_texture_view,
		})
	}

	fn push_instruction_inner(
		&mut self,
		mesh: Mesh,
		texture: MeshTexture,
		range: OffsetAndCount,
		mut draw_params_uniform: DrawParamsUniform,
		graphics_pipeline: Rc<GraphicsPipelineInner>,
		stencil_reference: u32,
	) {
		let set = self.draw_instruction_sets.last_mut().unwrap();
		let coordinate_system_transform = coordinate_system_transform(match &set.kind {
			DrawInstructionSetKind::Surface => UVec2::new(self.config.width, self.config.height),
			DrawInstructionSetKind::Canvas(canvas) => canvas.size(),
		});
		draw_params_uniform.transform = coordinate_system_transform * draw_params_uniform.transform;
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
		set.instructions.push(DrawInstruction {
			mesh,
			texture,
			range,
			draw_params_bind_group,
			graphics_pipeline,
			stencil_reference,
		});
	}
}

struct DrawInstruction {
	mesh: Mesh,
	texture: MeshTexture,
	range: OffsetAndCount,
	draw_params_bind_group: BindGroup,
	graphics_pipeline: Rc<GraphicsPipelineInner>,
	stencil_reference: u32,
}

struct DrawInstructionSet {
	kind: DrawInstructionSetKind,
	clear_color: Option<Rgba>,
	clear_stencil_value: Option<u32>,
	instructions: Vec<DrawInstruction>,
}

enum DrawInstructionSetKind {
	Surface,
	Canvas(Canvas),
}

fn coordinate_system_transform(size: UVec2) -> Mat4 {
	Mat4::from_translation(Vec3::new(-1.0, 1.0, 0.0))
		* Mat4::from_scale(Vec3::new(2.0 / size.x as f32, -2.0 / size.y as f32, 1.0))
}
