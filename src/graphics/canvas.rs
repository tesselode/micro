use std::rc::Rc;

use glow::{HasContext, NativeFramebuffer, NativeTexture};
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
	multisample_framebuffer: Option<MultisampleFramebuffer>,
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
		let multisample_framebuffer = match settings.msaa {
			Msaa::None => None,
			_ => Some(MultisampleFramebuffer::new(
				gl.clone(),
				size,
				settings.msaa.num_samples(),
			)?),
		};
		Ok(Self {
			gl,
			framebuffer,
			texture,
			multisample_framebuffer,
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
			ctx.gl.bind_framebuffer(
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
		ctx.set_render_target(RenderTarget::Canvas { size });
		let returned_value = f(ctx);
		if let Some(MultisampleFramebuffer { framebuffer, .. }) = self.multisample_framebuffer {
			unsafe {
				ctx.gl
					.bind_framebuffer(glow::READ_FRAMEBUFFER, Some(framebuffer));
				ctx.gl
					.bind_framebuffer(glow::DRAW_FRAMEBUFFER, Some(self.framebuffer));
				let width = self.size().x.try_into().expect("Canvas is too wide");
				let height = self.size().x.try_into().expect("Canvas is too tall");
				ctx.gl.blit_framebuffer(
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

#[derive(Debug)]
struct MultisampleFramebuffer {
	framebuffer: NativeFramebuffer,
	texture: NativeTexture,
}

impl MultisampleFramebuffer {
	fn new(gl: Rc<glow::Context>, size: Vec2<u32>, num_samples: u8) -> Result<Self, GlError> {
		let framebuffer = unsafe { gl.create_framebuffer() }.map_err(GlError)?;
		unsafe {
			gl.bind_framebuffer(glow::FRAMEBUFFER, Some(framebuffer));
		}
		let texture = unsafe { gl.create_texture().map_err(GlError)? };
		unsafe {
			gl.bind_texture(glow::TEXTURE_2D_MULTISAMPLE, Some(texture));
			gl.tex_image_2d_multisample(
				glow::TEXTURE_2D_MULTISAMPLE,
				num_samples.into(),
				glow::RGBA.try_into().unwrap(),
				size.x.try_into().expect("Canvas is too wide"),
				size.y.try_into().expect("Canvas is too tall"),
				true,
			);
			gl.framebuffer_texture_2d(
				glow::FRAMEBUFFER,
				glow::COLOR_ATTACHMENT0,
				glow::TEXTURE_2D_MULTISAMPLE,
				Some(texture),
				0,
			);
			gl.bind_framebuffer(glow::FRAMEBUFFER, None);
		};
		Ok(Self {
			framebuffer,
			texture,
		})
	}
}