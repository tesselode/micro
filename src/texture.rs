use std::{error::Error, rc::Rc};

use glow::{HasContext, NativeTexture};

use crate::{context::Context, image_data::ImageData};

pub(crate) struct RawTexture {
	pub(crate) gl: Rc<glow::Context>,
	pub(crate) texture: NativeTexture,
}

impl RawTexture {
	pub fn new(ctx: &Context, image_data: &ImageData) -> Result<Self, Box<dyn Error>> {
		let gl = ctx.graphics().gl();
		let texture;
		unsafe {
			texture = gl.create_texture()?;
			gl.bind_texture(glow::TEXTURE_2D, Some(texture));
			gl.tex_parameter_i32(
				glow::TEXTURE_2D,
				glow::TEXTURE_WRAP_S,
				glow::REPEAT.try_into().unwrap(),
			);
			gl.tex_parameter_i32(
				glow::TEXTURE_2D,
				glow::TEXTURE_WRAP_T,
				glow::REPEAT.try_into().unwrap(),
			);
			gl.tex_parameter_i32(
				glow::TEXTURE_2D,
				glow::TEXTURE_MIN_FILTER,
				glow::LINEAR_MIPMAP_LINEAR.try_into().unwrap(),
			);
			gl.tex_parameter_i32(
				glow::TEXTURE_2D,
				glow::TEXTURE_MAG_FILTER,
				glow::LINEAR.try_into().unwrap(),
			);
			gl.tex_image_2d(
				glow::TEXTURE_2D,
				0,
				glow::RGBA.try_into().unwrap(),
				image_data.width.try_into().unwrap(),
				image_data.height.try_into().unwrap(),
				0,
				glow::RGBA,
				glow::UNSIGNED_BYTE,
				Some(&image_data.data),
			);
			gl.generate_mipmap(glow::TEXTURE_2D);
			gl.bind_texture(glow::TEXTURE_2D, None);
		}
		Ok(Self { gl, texture })
	}

	pub(crate) fn texture(&self) -> NativeTexture {
		self.texture
	}
}

impl Drop for RawTexture {
	fn drop(&mut self) {
		unsafe {
			self.gl.delete_texture(self.texture);
		}
	}
}

#[derive(Clone)]
pub struct Texture {
	pub(crate) raw: Rc<RawTexture>,
}

impl Texture {
	pub fn new(ctx: &Context, image_data: &ImageData) -> Result<Self, Box<dyn Error>> {
		Ok(Self {
			raw: Rc::new(RawTexture::new(ctx, image_data)?),
		})
	}
}
