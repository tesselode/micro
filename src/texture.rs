use std::{path::Path, rc::Rc};

use glow::{HasContext, NativeTexture};
use thiserror::Error;

use crate::{
	context::Context,
	image_data::{ImageData, LoadImageDataError},
};

#[derive(Debug, Error)]
pub enum LoadTextureError {
	#[error("{0}")]
	LoadImageDataError(#[from] LoadImageDataError),
	#[error("{0}")]
	GlError(String),
}

impl From<String> for LoadTextureError {
	fn from(v: String) -> Self {
		Self::GlError(v)
	}
}

pub struct RawTexture {
	gl: Rc<glow::Context>,
	pub(crate) texture: NativeTexture,
}

impl RawTexture {
	pub fn new(gl: Rc<glow::Context>, image_data: &ImageData) -> Result<Self, String> {
		let texture = unsafe { gl.create_texture()? };
		unsafe {
			gl.bind_texture(glow::TEXTURE_2D, Some(texture));
			gl.tex_image_2d(
				glow::TEXTURE_2D,
				0,
				glow::RGBA as i32,
				image_data.0.width().try_into().expect("Image is too wide"),
				image_data.0.height().try_into().expect("Image is too tall"),
				0,
				glow::RGBA,
				glow::UNSIGNED_BYTE,
				Some(image_data.0.as_raw()),
			);
			gl.generate_mipmap(glow::TEXTURE_2D);
		}
		Ok(Self { gl, texture })
	}
}

impl Drop for RawTexture {
	fn drop(&mut self) {
		unsafe {
			self.gl.delete_texture(self.texture);
		}
	}
}

pub struct Texture {
	pub(crate) raw_texture: Rc<RawTexture>,
}

impl Texture {
	pub fn load(ctx: &Context, path: impl AsRef<Path>) -> Result<Self, LoadTextureError> {
		let image_data = ImageData::load(path)?;
		Ok(Self {
			raw_texture: Rc::new(RawTexture::new(ctx.gl.clone(), &image_data)?),
		})
	}
}
