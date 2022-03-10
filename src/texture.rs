use std::{path::Path, rc::Rc};

use glam::Vec2;
use glow::{HasContext, NativeTexture};
use thiserror::Error;

use crate::{
	context::Context,
	draw_params::DrawParams,
	error::GlError,
	image_data::{ImageData, LoadImageDataError},
	mesh::Mesh,
};

#[derive(Debug, Clone)]
pub struct Texture {
	pub(crate) raw_texture: Rc<RawTexture>,
}

impl Texture {
	pub fn load(ctx: &Context, path: impl AsRef<Path>) -> Result<Self, LoadTextureError> {
		let image_data = ImageData::load(path)?;
		Ok(Self {
			raw_texture: Rc::new(
				RawTexture::new(ctx.gl.clone(), &image_data)
					.map_err(|error| LoadTextureError::GlError(error.0))?,
			),
		})
	}

	pub fn draw(&self, ctx: &Context, params: impl Into<DrawParams>) -> Result<(), GlError> {
		Mesh::rectangle(ctx, Vec2::ZERO, self.raw_texture.size)?
			.with_texture(self)
			.draw(ctx, params);
		Ok(())
	}
}

#[derive(Debug)]
pub struct RawTexture {
	gl: Rc<glow::Context>,
	pub(crate) texture: NativeTexture,
	pub(crate) size: Vec2,
}

impl RawTexture {
	pub fn new(gl: Rc<glow::Context>, image_data: &ImageData) -> Result<Self, GlError> {
		let (width, height) = image_data.0.dimensions();
		let texture = unsafe { gl.create_texture() }.map_err(GlError)?;
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
		Ok(Self {
			gl,
			texture,
			size: Vec2::new(width as f32, height as f32),
		})
	}
}

impl Drop for RawTexture {
	fn drop(&mut self) {
		unsafe {
			self.gl.delete_texture(self.texture);
		}
	}
}

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
