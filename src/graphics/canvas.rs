use std::rc::Rc;

use glow::{HasContext, NativeFramebuffer};
use vek::Vec2;

use crate::{context::RenderTarget, error::GlError, math::Rect, Context};

use super::{
	texture::{Texture, TextureSettings},
	DrawParams,
};

#[derive(Debug)]
pub struct Canvas {
	gl: Rc<glow::Context>,
	framebuffer: NativeFramebuffer,
	texture: Texture,
}

impl Canvas {
	pub fn new(ctx: &Context, size: Vec2<u32>, settings: CanvasSettings) -> Result<Self, GlError> {
		let gl = ctx.gl.clone();
		let framebuffer = unsafe { gl.create_framebuffer() }.map_err(GlError)?;
		unsafe {
			gl.bind_framebuffer(glow::FRAMEBUFFER, Some(framebuffer));
		}
		let mut texture = Texture::empty(ctx, size, settings.texture_settings)?;
		texture.attach_to_framebuffer();
		unsafe {
			gl.bind_framebuffer(glow::FRAMEBUFFER, None);
		}
		Ok(Self {
			gl,
			framebuffer,
			texture,
		})
	}

	pub fn size(&self) -> Vec2<u32> {
		self.texture.size()
	}

	pub fn relative_rect(&self, absolute_rect: Rect) -> Rect {
		let size = self.size().as_::<f32>();
		Rect {
			top_left: absolute_rect.top_left / size,
			bottom_right: absolute_rect.bottom_right / size,
		}
	}

	pub fn render_to<T>(&self, ctx: &mut Context, f: impl FnOnce(&mut Context) -> T) -> T {
		if let RenderTarget::Canvas { .. } = ctx.render_target {
			unimplemented!("cannot nest render_to calls");
		}
		let size = self.size();
		unsafe {
			ctx.gl
				.bind_framebuffer(glow::FRAMEBUFFER, Some(self.framebuffer));
		}
		ctx.set_render_target(RenderTarget::Canvas { size });
		let returned_value = f(ctx);
		unsafe {
			ctx.gl.bind_framebuffer(glow::FRAMEBUFFER, None);
		}
		ctx.set_render_target(RenderTarget::Window);
		returned_value
	}

	pub fn draw<'a>(
		&self,
		ctx: &Context,
		params: impl Into<DrawParams<'a>>,
	) -> Result<(), GlError> {
		self.texture.draw(ctx, params)
	}

	pub fn draw_region<'a>(
		&self,
		ctx: &Context,
		region: Rect,
		params: impl Into<DrawParams<'a>>,
	) -> Result<(), GlError> {
		self.texture.draw_region(ctx, region, params)
	}
}

impl Drop for Canvas {
	fn drop(&mut self) {
		unsafe {
			self.gl.delete_framebuffer(self.framebuffer);
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct CanvasSettings {
	pub texture_settings: TextureSettings,
}
