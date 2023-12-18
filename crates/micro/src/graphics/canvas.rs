use std::{
	ops::{Deref, DerefMut},
	rc::Rc,
	sync::mpsc::Sender,
};

use glam::UVec2;
use glow::{HasContext, NativeFramebuffer, NativeRenderbuffer, NativeTexture, PixelPackData};

use crate::{context::graphics::RenderTarget, math::Rect, Context};

use super::{
	texture::{Texture, TextureSettings},
	unused_resource::UnusedGraphicsResource,
	DrawParams,
};

#[derive(Debug)]
pub struct Canvas {
	framebuffer: NativeFramebuffer,
	texture: Texture,
	depth_stencil_renderbuffer: NativeRenderbuffer,
	multisample_framebuffer: Option<MultisampleFramebuffer>,
	unused_resource_sender: Sender<UnusedGraphicsResource>,
}

impl Canvas {
	pub fn new(ctx: &Context, size: UVec2, settings: CanvasSettings) -> Self {
		let gl = &ctx.graphics.gl;
		let framebuffer = unsafe {
			gl.create_framebuffer()
				.expect("error creating canvas framebuffer")
		};
		unsafe {
			gl.bind_framebuffer(glow::FRAMEBUFFER, Some(framebuffer));
		}
		let mut texture = Texture::empty(ctx, size, settings.texture_settings);
		texture.attach_to_framebuffer(ctx);
		let multisample_framebuffer = match settings.msaa {
			Msaa::None => None,
			_ => Some(MultisampleFramebuffer::new(
				gl.clone(),
				size,
				settings.msaa.num_samples(),
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
		Self {
			framebuffer,
			texture,
			depth_stencil_renderbuffer,
			multisample_framebuffer,
			unused_resource_sender: ctx.graphics.unused_resource_sender.clone(),
		}
	}

	pub fn size(&self) -> UVec2 {
		self.texture.size()
	}

	pub fn relative_rect(&self, absolute_rect: Rect) -> Rect {
		let size = self.size().as_vec2();
		Rect::from_top_left_and_bottom_right(
			absolute_rect.top_left / size,
			absolute_rect.bottom_right() / size,
		)
	}

	pub fn render_to<'a>(&'a self, ctx: &'a mut Context) -> OnDrop {
		if let RenderTarget::Canvas { .. } = ctx.graphics.render_target {
			unimplemented!("cannot nest render_to calls");
		}
		let size = self.size();
		unsafe {
			ctx.graphics.gl.bind_framebuffer(
				glow::FRAMEBUFFER,
				Some(
					if let Some(MultisampleFramebuffer { framebuffer, .. }) =
						self.multisample_framebuffer
					{
						framebuffer
					} else {
						self.framebuffer
					},
				),
			);
		}
		ctx.graphics
			.set_render_target(RenderTarget::Canvas { size });
		OnDrop {
			ctx,
			canvas: self,
			on_drop: |ctx, canvas| {
				if let Some(MultisampleFramebuffer { framebuffer, .. }) =
					canvas.multisample_framebuffer
				{
					unsafe {
						ctx.graphics
							.gl
							.bind_framebuffer(glow::READ_FRAMEBUFFER, Some(framebuffer));
						ctx.graphics
							.gl
							.bind_framebuffer(glow::DRAW_FRAMEBUFFER, Some(canvas.framebuffer));
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
				unsafe {
					ctx.graphics.gl.bind_framebuffer(glow::FRAMEBUFFER, None);
				}
				ctx.graphics.set_render_target(RenderTarget::Window);
			},
		}
	}

	pub fn draw<'a>(&self, ctx: &Context, params: impl Into<DrawParams<'a>>) {
		self.texture.draw(ctx, params)
	}

	pub fn draw_region<'a>(&self, ctx: &Context, region: Rect, params: impl Into<DrawParams<'a>>) {
		self.texture.draw_region(ctx, region, params)
	}

	pub fn read(&self, ctx: &Context, buffer: &mut [u8]) {
		if buffer.len() < (self.size().x * self.size().y * 4) as usize {
			panic!("buffer not big enough");
		}
		let gl = &ctx.graphics.gl;
		unsafe {
			gl.bind_framebuffer(glow::READ_FRAMEBUFFER, Some(self.framebuffer));
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
	}
}

impl Drop for Canvas {
	fn drop(&mut self) {
		self.unused_resource_sender
			.send(UnusedGraphicsResource::Framebuffer(self.framebuffer))
			.ok();
		self.unused_resource_sender
			.send(UnusedGraphicsResource::Renderbuffer(
				self.depth_stencil_renderbuffer,
			))
			.ok();
		if let Some(MultisampleFramebuffer {
			framebuffer,
			texture,
		}) = self.multisample_framebuffer
		{
			self.unused_resource_sender
				.send(UnusedGraphicsResource::Texture(texture))
				.ok();
			self.unused_resource_sender
				.send(UnusedGraphicsResource::Framebuffer(framebuffer))
				.ok();
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct CanvasSettings {
	pub texture_settings: TextureSettings,
	pub msaa: Msaa,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Msaa {
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

impl Default for Msaa {
	fn default() -> Self {
		Self::None
	}
}

#[must_use]
pub struct OnDrop<'a> {
	pub(crate) ctx: &'a mut Context,
	pub(crate) canvas: &'a Canvas,
	pub(crate) on_drop: fn(&mut Context, &Canvas),
}

impl<'a> Deref for OnDrop<'a> {
	type Target = Context;

	fn deref(&self) -> &Self::Target {
		self.ctx
	}
}

impl<'a> DerefMut for OnDrop<'a> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.ctx
	}
}

impl<'a> Drop for OnDrop<'a> {
	fn drop(&mut self) {
		(self.on_drop)(self.ctx, self.canvas);
	}
}

#[derive(Debug)]
struct MultisampleFramebuffer {
	framebuffer: NativeFramebuffer,
	texture: NativeTexture,
}

impl MultisampleFramebuffer {
	fn new(gl: Rc<glow::Context>, size: UVec2, num_samples: u8) -> Self {
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
				glow::SRGB8_ALPHA8 as i32,
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
