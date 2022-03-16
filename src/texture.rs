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
	rect::Rect,
};

#[derive(Debug)]
pub struct Texture {
	gl: Rc<glow::Context>,
	pub(crate) texture: NativeTexture,
	pub(crate) size: Vec2,
}

impl Texture {
	pub fn from_image_data(ctx: &Context, image_data: &ImageData) -> Result<Self, GlError> {
		Self::new_from_gl(ctx.gl.clone(), image_data)
	}

	pub(crate) fn new_from_gl(
		gl: Rc<glow::Context>,
		image_data: &ImageData,
	) -> Result<Self, GlError> {
		let texture = unsafe { gl.create_texture() }.map_err(GlError)?;
		unsafe {
			gl.bind_texture(glow::TEXTURE_2D, Some(texture));
			gl.tex_image_2d(
				glow::TEXTURE_2D,
				0,
				glow::RGBA as i32,
				image_data.width.try_into().expect("Image is too wide"),
				image_data.height.try_into().expect("Image is too tall"),
				0,
				glow::RGBA,
				glow::UNSIGNED_BYTE,
				Some(&image_data.pixels),
			);
			gl.generate_mipmap(glow::TEXTURE_2D);
		}
		Ok(Self {
			gl,
			texture,
			size: Vec2::new(image_data.width as f32, image_data.height as f32),
		})
	}

	pub fn load(ctx: &Context, path: impl AsRef<Path>) -> Result<Self, LoadTextureError> {
		let image_data = ImageData::load(path)?;
		Self::from_image_data(ctx, &image_data).map_err(|error| LoadTextureError::GlError(error.0))
	}

	pub fn size(&self) -> Vec2 {
		self.size
	}

	pub fn relative_rect(&self, absolute_rect: Rect) -> Rect {
		Rect {
			top_left: absolute_rect.top_left / self.size(),
			size: absolute_rect.size / self.size(),
		}
	}

	pub fn draw<'a>(
		&self,
		ctx: &Context,
		params: impl Into<DrawParams<'a>>,
	) -> Result<(), GlError> {
		Mesh::rectangle(ctx, Rect::new(Vec2::ZERO, self.size))?.draw_textured(ctx, self, params);
		Ok(())
	}
}

impl Drop for Texture {
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
