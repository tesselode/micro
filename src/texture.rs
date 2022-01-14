use std::{error::Error, rc::Rc};

use glow::{HasContext, NativeTexture};

use crate::{context::Context, image_data::ImageData};

pub(crate) struct RawTexture {
	gl: Rc<glow::Context>,
	native_texture: NativeTexture,
}

impl RawTexture {
	pub(crate) fn new(
		gl: Rc<glow::Context>,
		image_data: &ImageData,
	) -> Result<Self, Box<dyn Error>> {
		let native_texture;
		unsafe {
			native_texture = gl.create_texture()?;
			gl.bind_texture(glow::TEXTURE_2D, Some(native_texture));
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
		Ok(Self { gl, native_texture })
	}
}

impl Drop for RawTexture {
	fn drop(&mut self) {
		unsafe {
			self.gl.delete_texture(self.native_texture);
		}
	}
}

#[derive(Clone)]
pub struct Texture {
	raw: Rc<RawTexture>,
}

impl Texture {
	pub fn new(ctx: &Context, image_data: &ImageData) -> Result<Self, Box<dyn Error>> {
		Ok(Self {
			raw: Rc::new(RawTexture::new(ctx.graphics().gl(), image_data)?),
		})
	}

	pub(crate) fn from_raw(raw_texture: RawTexture) -> Self {
		Self {
			raw: Rc::new(raw_texture),
		}
	}

	pub(crate) fn native_texture(&self) -> NativeTexture {
		self.raw.native_texture
	}
}
