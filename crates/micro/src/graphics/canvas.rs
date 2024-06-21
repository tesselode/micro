use std::{
	rc::Rc,
	sync::{
		atomic::{AtomicU64, Ordering},
		Weak,
	},
};

use glam::{Mat4, UVec2, Vec2};
use glow::{HasContext, NativeFramebuffer, NativeRenderbuffer, NativeTexture, PixelPackData};
use palette::LinSrgba;

use crate::{context::graphics::RenderTarget, math::Rect, Context};

use super::{
	resource::{GraphicsResource, GraphicsResourceId},
	shader::Shader,
	standard_draw_param_methods,
	texture::{Texture, TextureSettings},
	BlendMode, ColorConstants,
};

#[derive(Debug, Clone)]
pub struct Canvas {
	id: CanvasId,
	_weak: Weak<()>,
	texture: Texture,

	// draw params
	pub region: Rect,
	pub shader: Option<Shader>,
	pub transform: Mat4,
	pub color: LinSrgba,
	pub blend_mode: BlendMode,
}

impl Canvas {
	pub fn new(size: UVec2, settings: CanvasSettings) -> Self {
		let texture = Texture::new(size, None, settings.texture_settings, settings.hdr);
		Context::with_mut(|ctx| {
			let gl = &ctx.graphics.gl;
			let framebuffer = unsafe {
				gl.create_framebuffer()
					.expect("error creating canvas framebuffer")
			};
			unsafe {
				gl.bind_framebuffer(glow::FRAMEBUFFER, Some(framebuffer));
				let raw_texture = ctx.graphics.textures.get(texture.id);
				gl.bind_texture(glow::TEXTURE_2D, Some(raw_texture.texture));
				gl.framebuffer_texture_2d(
					glow::FRAMEBUFFER,
					glow::COLOR_ATTACHMENT0,
					glow::TEXTURE_2D,
					Some(raw_texture.texture),
					0,
				);
			}
			let multisample_framebuffer = match settings.msaa {
				Msaa::None => None,
				_ => Some(MultisampleFramebuffer::new(
					gl.clone(),
					size,
					settings.msaa.num_samples(),
					settings.hdr,
				)),
			};
			let depth_stencil_renderbuffer = unsafe {
				gl.create_renderbuffer()
					.expect("error creating depth/stencil renderbuffer")
			};
			unsafe {
				gl.bind_renderbuffer(glow::RENDERBUFFER, Some(depth_stencil_renderbuffer));
				if settings.msaa != Msaa::None {
					gl.renderbuffer_storage_multisample(
						glow::RENDERBUFFER,
						settings.msaa.num_samples().into(),
						glow::DEPTH24_STENCIL8,
						size.x as i32,
						size.y as i32,
					);
				} else {
					gl.renderbuffer_storage(
						glow::RENDERBUFFER,
						glow::DEPTH24_STENCIL8,
						size.x as i32,
						size.y as i32,
					);
				}
				gl.framebuffer_renderbuffer(
					glow::FRAMEBUFFER,
					glow::DEPTH_STENCIL_ATTACHMENT,
					glow::RENDERBUFFER,
					Some(depth_stencil_renderbuffer),
				);
			}
			unsafe {
				gl.bind_framebuffer(glow::FRAMEBUFFER, None);
			}
			let (id, weak) = ctx.graphics.canvases.insert(RawCanvas {
				gl: gl.clone(),
				framebuffer,
				depth_stencil_renderbuffer,
				multisample_framebuffer,
			});
			Self {
				id,
				_weak: weak,
				texture: texture.clone(),
				shader: None,
				transform: Mat4::IDENTITY,
				color: LinSrgba::WHITE,
				blend_mode: BlendMode::default(),
				region: Rect::new(Vec2::ZERO, size.as_vec2()),
			}
		})
	}

	pub fn region(&self, region: Rect) -> Self {
		let mut new = self.clone();
		new.region = region;
		new
	}

	standard_draw_param_methods!();

	pub fn size(&self) -> UVec2 {
		self.texture.size()
	}

	pub fn relative_rect(&self, absolute_rect: Rect) -> Rect {
		let size = self.size().as_vec2();
		Rect::from_corners(
			absolute_rect.top_left / size,
			absolute_rect.bottom_right() / size,
		)
	}

	pub fn render_to(&self) -> OnDrop {
		if let RenderTarget::Canvas { .. } = Context::with(|ctx| ctx.graphics.render_target) {
			unimplemented!("cannot nest render_to calls");
		}
		let size = self.size();
		Context::with_mut(|ctx| {
			let canvas = ctx.graphics.canvases.get(self.id);
			unsafe {
				ctx.graphics.gl.bind_framebuffer(
					glow::FRAMEBUFFER,
					Some(
						if let Some(MultisampleFramebuffer { framebuffer, .. }) =
							canvas.multisample_framebuffer
						{
							framebuffer
						} else {
							canvas.framebuffer
						},
					),
				);
			}
			ctx.graphics
				.set_render_target(RenderTarget::Canvas { size })
		});
		OnDrop {
			canvas: self,
			on_drop: |canvas| {
				Context::with_mut(|ctx| {
					let raw_canvas = ctx.graphics.canvases.get(canvas.id);
					if let Some(MultisampleFramebuffer { framebuffer, .. }) =
						raw_canvas.multisample_framebuffer
					{
						unsafe {
							ctx.graphics
								.gl
								.bind_framebuffer(glow::READ_FRAMEBUFFER, Some(framebuffer));
							ctx.graphics.gl.bind_framebuffer(
								glow::DRAW_FRAMEBUFFER,
								Some(raw_canvas.framebuffer),
							);
							let width = canvas.size().x as i32;
							let height = canvas.size().y as i32;
							ctx.graphics.gl.blit_framebuffer(
								0,
								0,
								width,
								height,
								0,
								0,
								width,
								height,
								glow::COLOR_BUFFER_BIT,
								glow::NEAREST,
							);
						}
					}
					unsafe { ctx.graphics.gl.bind_framebuffer(glow::FRAMEBUFFER, None) }
					ctx.graphics.set_render_target(RenderTarget::Window);
				});
			},
		}
	}

	pub fn draw(&self) {
		self.texture
			.region(self.region)
			.shader(&self.shader)
			.transformed(self.transform)
			.color(self.color)
			.blend_mode(self.blend_mode)
			.draw();
	}

	pub fn read(&self, buffer: &mut [u8]) {
		if buffer.len() < (self.size().x * self.size().y * 4) as usize {
			panic!("buffer not big enough");
		}
		Context::with(|ctx| {
			let gl = &ctx.graphics.gl;
			let canvas = ctx.graphics.canvases.get(self.id);
			unsafe {
				gl.bind_framebuffer(glow::READ_FRAMEBUFFER, Some(canvas.framebuffer));
				gl.read_buffer(glow::COLOR_ATTACHMENT0);
				gl.read_pixels(
					0,
					0,
					self.size().x as i32,
					self.size().y as i32,
					glow::RGBA,
					glow::UNSIGNED_BYTE,
					PixelPackData::Slice(buffer),
				);
			}
		});
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct CanvasSettings {
	pub texture_settings: TextureSettings,
	pub msaa: Msaa,
	pub hdr: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serializing", derive(serde::Serialize, serde::Deserialize))]
pub enum Msaa {
	#[default]
	None,
	X2,
	X4,
	X8,
	X16,
}

impl Msaa {
	fn num_samples(&self) -> u8 {
		match self {
			Msaa::None => 0,
			Msaa::X2 => 2,
			Msaa::X4 => 4,
			Msaa::X8 => 8,
			Msaa::X16 => 16,
		}
	}
}

#[derive(Debug)]
struct MultisampleFramebuffer {
	framebuffer: NativeFramebuffer,
	texture: NativeTexture,
}

impl MultisampleFramebuffer {
	fn new(gl: Rc<glow::Context>, size: UVec2, num_samples: u8, float: bool) -> Self {
		let framebuffer = unsafe {
			gl.create_framebuffer()
				.expect("error creating multisample framebuffer")
		};
		unsafe {
			gl.bind_framebuffer(glow::FRAMEBUFFER, Some(framebuffer));
		}
		let texture = unsafe {
			gl.create_texture()
				.expect("error creating multisample texture")
		};
		unsafe {
			gl.bind_texture(glow::TEXTURE_2D_MULTISAMPLE, Some(texture));
			gl.tex_image_2d_multisample(
				glow::TEXTURE_2D_MULTISAMPLE,
				num_samples.into(),
				if float {
					glow::RGBA16F
				} else {
					glow::SRGB8_ALPHA8
				} as i32,
				size.x as i32,
				size.y as i32,
				true,
			);
			gl.framebuffer_texture_2d(
				glow::FRAMEBUFFER,
				glow::COLOR_ATTACHMENT0,
				glow::TEXTURE_2D_MULTISAMPLE,
				Some(texture),
				0,
			);
		};
		// intentionally not unbinding this framebuffer because the
		// depth/stencil buffer will need it bound
		Self {
			framebuffer,
			texture,
		}
	}
}

#[must_use]
pub struct OnDrop<'a> {
	pub(crate) canvas: &'a Canvas,
	pub(crate) on_drop: fn(&Canvas),
}

impl<'a> Drop for OnDrop<'a> {
	fn drop(&mut self) {
		(self.on_drop)(self.canvas);
	}
}

#[derive(Debug)]
pub(crate) struct RawCanvas {
	gl: Rc<glow::Context>,
	framebuffer: NativeFramebuffer,
	depth_stencil_renderbuffer: NativeRenderbuffer,
	multisample_framebuffer: Option<MultisampleFramebuffer>,
}

impl Drop for RawCanvas {
	fn drop(&mut self) {
		unsafe {
			self.gl.delete_framebuffer(self.framebuffer);
			self.gl.delete_renderbuffer(self.depth_stencil_renderbuffer);
			if let Some(MultisampleFramebuffer {
				framebuffer,
				texture,
			}) = self.multisample_framebuffer
			{
				self.gl.delete_texture(texture);
				self.gl.delete_framebuffer(framebuffer);
			}
		}
	}
}

impl GraphicsResource for RawCanvas {
	type Id = CanvasId;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct CanvasId(pub u64);

static NEXT_CANVAS_ID: AtomicU64 = AtomicU64::new(0);

impl GraphicsResourceId for CanvasId {
	fn next() -> Self {
		CanvasId(NEXT_CANVAS_ID.fetch_add(1, Ordering::SeqCst))
	}
}
