use std::rc::Rc;

use glow::{HasContext, NativeFramebuffer, NativeRenderbuffer};

use crate::{
	context::Context,
	texture::{RawTexture, Texture},
};

pub struct Canvas {
	gl: Rc<glow::Context>,
	framebuffer: NativeFramebuffer,
	texture: Texture,
	renderbuffer: NativeRenderbuffer,
}

impl Canvas {
	pub fn new(ctx: &Context, width: i32, height: i32) -> Result<Self, String> {
		let gl = ctx.graphics().gl();
		let framebuffer;
		let texture;
		let renderbuffer;
		unsafe {
			framebuffer = gl.create_framebuffer()?;
			gl.bind_framebuffer(glow::FRAMEBUFFER, Some(framebuffer));
			texture = gl.create_texture()?;
			gl.bind_texture(glow::TEXTURE_2D, Some(texture));
			gl.tex_image_2d(
				glow::TEXTURE_2D,
				0,
				glow::RGBA.try_into().unwrap(),
				width,
				height,
				0,
				glow::RGBA,
				glow::UNSIGNED_BYTE,
				None,
			);
			gl.tex_parameter_i32(
				glow::TEXTURE_2D,
				glow::TEXTURE_MIN_FILTER,
				glow::LINEAR.try_into().unwrap(),
			);
			gl.tex_parameter_i32(
				glow::TEXTURE_2D,
				glow::TEXTURE_MAG_FILTER,
				glow::LINEAR.try_into().unwrap(),
			);
			gl.framebuffer_texture_2d(
				glow::FRAMEBUFFER,
				glow::COLOR_ATTACHMENT0,
				glow::TEXTURE_2D,
				Some(texture),
				0,
			);
			renderbuffer = gl.create_renderbuffer()?;
			gl.bind_renderbuffer(glow::RENDERBUFFER, Some(renderbuffer));
			gl.renderbuffer_storage(glow::RENDERBUFFER, glow::DEPTH24_STENCIL8, width, height);
			gl.bind_renderbuffer(glow::RENDERBUFFER, None);
			gl.framebuffer_renderbuffer(
				glow::FRAMEBUFFER,
				glow::DEPTH_STENCIL_ATTACHMENT,
				glow::RENDERBUFFER,
				Some(renderbuffer),
			);
			gl.bind_framebuffer(glow::FRAMEBUFFER, None);
		}
		Ok(Self {
			gl: gl.clone(),
			framebuffer,
			texture: Texture::from_raw(RawTexture {
				gl,
				native_texture: texture,
			}),
			renderbuffer,
		})
	}

	pub fn texture(&self) -> Texture {
		self.texture.clone()
	}

	pub fn draw_on(&self, f: impl FnOnce()) {
		unsafe {
			self.gl
				.bind_framebuffer(glow::FRAMEBUFFER, Some(self.framebuffer));
		}
		f();
		unsafe {
			self.gl.bind_framebuffer(glow::FRAMEBUFFER, None);
		}
	}
}
