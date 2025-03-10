use std::{cell::RefCell, rc::Rc};

use glam::{IVec2, Mat4, UVec2, Vec3};
use glow::HasContext;
use sdl2::{
	VideoSubsystem,
	video::{GLContext, Window},
};
use tracy_client::{GpuContextType, SpanLocation};

use crate::graphics::{
	RawCanvas, RawVertexAttributeBuffer,
	gpu_span::{GpuSpan, GpuSpanInner},
	mesh::RawMesh,
	resource::GraphicsResources,
	shader::{DEFAULT_FRAGMENT_SHADER, DEFAULT_VERTEX_SHADER, RawShader, Shader},
	texture::{RawTexture, Texture, TextureSettings},
};

pub(crate) struct GraphicsContext {
	pub(crate) gl: Rc<glow::Context>,
	pub(crate) meshes: GraphicsResources<RawMesh>,
	pub(crate) textures: GraphicsResources<RawTexture>,
	pub(crate) shaders: GraphicsResources<RawShader>,
	pub(crate) canvases: GraphicsResources<RawCanvas>,
	pub(crate) vertex_attribute_buffers: GraphicsResources<RawVertexAttributeBuffer>,
	pub(crate) default_texture: Texture,
	pub(crate) default_shader: Shader,
	pub(crate) transform_stack: Vec<Mat4>,
	pub(crate) render_target: RenderTarget,
	pub(crate) tracy_gpu_ctx: tracy_client::GpuContext,
	gpu_spans: Vec<Rc<RefCell<GpuSpanInner>>>,
	viewport_size: IVec2,
	_sdl_gl_ctx: GLContext,
}

impl GraphicsContext {
	pub(crate) fn new(video: &VideoSubsystem, window: &Window) -> Self {
		let _sdl_gl_ctx = window
			.gl_create_context()
			.expect("error creating OpenGL context");
		let (window_width, window_height) = window.drawable_size();
		let viewport_size = UVec2::new(window_width, window_height).as_ivec2();
		let gl = Rc::new(unsafe {
			glow::Context::from_loader_function(|name| video.gl_get_proc_address(name) as *const _)
		});
		unsafe {
			gl.enable(glow::BLEND);
			gl.enable(glow::FRAMEBUFFER_SRGB);
			gl.enable(glow::CLIP_DISTANCE0);
			gl.blend_func(glow::SRC_ALPHA, glow::ONE_MINUS_SRC_ALPHA);
			gl.viewport(0, 0, viewport_size.x, viewport_size.y);
		}
		let meshes = GraphicsResources::new();
		let mut textures = GraphicsResources::new();
		let mut shaders = GraphicsResources::new();
		let canvases = GraphicsResources::new();
		let vertex_attribute_buffers = GraphicsResources::new();
		let default_texture = Texture::new(
			gl.clone(),
			&mut textures,
			UVec2::new(1, 1),
			Some(&[255, 255, 255, 255]),
			TextureSettings::default(),
			false,
		);
		let default_shader = Shader::new(
			gl.clone(),
			&mut shaders,
			DEFAULT_VERTEX_SHADER,
			DEFAULT_FRAGMENT_SHADER,
		)
		.expect("error compiling default shader");
		let gpu_timestamp = unsafe { gl.get_parameter_i64(glow::TIMESTAMP) };
		Self {
			gl,
			meshes,
			textures,
			shaders,
			canvases,
			default_texture,
			default_shader,
			vertex_attribute_buffers,
			transform_stack: vec![],
			render_target: RenderTarget::Window,
			tracy_gpu_ctx: tracy_client::Client::running()
				.unwrap()
				.new_gpu_context(
					Some("GPU context"),
					GpuContextType::OpenGL,
					gpu_timestamp,
					1.0,
				)
				.unwrap(),
			gpu_spans: vec![],
			viewport_size,
			_sdl_gl_ctx,
		}
	}

	pub(crate) fn set_render_target(&mut self, render_target: RenderTarget) {
		self.render_target = render_target;
		let viewport_size: IVec2 = match render_target {
			RenderTarget::Window => self.viewport_size,
			RenderTarget::Canvas { size } => size.as_ivec2(),
		};
		unsafe {
			self.gl.viewport(0, 0, viewport_size.x, viewport_size.y);
		}
	}

	pub(crate) fn resize(&mut self, size: UVec2) {
		self.viewport_size = size.as_ivec2();
		unsafe {
			self.gl
				.viewport(0, 0, self.viewport_size.x, self.viewport_size.y);
		}
	}

	pub(crate) fn delete_unused_resources(&mut self) {
		self.meshes.delete_unused();
		self.textures.delete_unused();
		self.shaders.delete_unused();
		self.canvases.delete_unused();
		self.vertex_attribute_buffers.delete_unused();
	}

	pub(crate) fn global_transform(&self) -> Mat4 {
		let coordinate_system_transform = match self.render_target {
			RenderTarget::Window => {
				Mat4::from_translation(Vec3::new(-1.0, 1.0, 0.0))
					* Mat4::from_scale(Vec3::new(
						2.0 / self.viewport_size.x as f32,
						-2.0 / self.viewport_size.y as f32,
						1.0,
					))
			}
			RenderTarget::Canvas { size } => {
				Mat4::from_translation(Vec3::new(-1.0, -1.0, 0.0))
					* Mat4::from_scale(Vec3::new(2.0 / size.x as f32, 2.0 / size.y as f32, 1.0))
			}
		};
		self.transform_stack
			.iter()
			.fold(coordinate_system_transform, |previous, transform| {
				previous * *transform
			})
	}

	pub(crate) fn create_gpu_span(&mut self, span_location: &'static SpanLocation) -> GpuSpan {
		let start_query = unsafe { self.gl.create_query() }.unwrap();
		unsafe {
			self.gl.query_counter(start_query, glow::TIMESTAMP);
		}
		let end_query = unsafe { self.gl.create_query() }.unwrap();
		let inner = Rc::new(RefCell::new(GpuSpanInner {
			tracy_gpu_span: self.tracy_gpu_ctx.span(span_location).unwrap(),
			start_query,
			end_query,
		}));
		self.gpu_spans.push(inner.clone());
		GpuSpan {
			gl: self.gl.clone(),
			inner,
		}
	}

	pub(crate) fn record_queries(&mut self) {
		self.gpu_spans.retain_mut(|span| {
			let recorded = span.borrow_mut().try_record(self.gl.clone());
			!recorded
		});
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum RenderTarget {
	Window,
	Canvas { size: UVec2 },
}
