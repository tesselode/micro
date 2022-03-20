use std::{path::Path, rc::Rc};

use glam::Vec2;
use glow::{HasContext, NativeTexture, PixelUnpackData};
use thiserror::Error;

use crate::{
	context::Context,
	error::GlError,
	graphics::{
		draw_params::DrawParams,
		image_data::{ImageData, LoadImageDataError},
		mesh::Mesh,
	},
	math::Rect,
};

use super::color::Rgba;

#[derive(Debug)]
pub struct Texture {
	gl: Rc<glow::Context>,
	pub(crate) texture: NativeTexture,
	pub(crate) size: (u32, u32),
}

impl Texture {
	pub fn empty(
		ctx: &Context,
		size: (u32, u32),
		settings: TextureSettings,
	) -> Result<Self, GlError> {
		Self::new_from_gl(ctx.gl.clone(), size, None, settings)
	}

	pub fn from_image_data(
		ctx: &Context,
		image_data: &ImageData,
		settings: TextureSettings,
	) -> Result<Self, GlError> {
		Self::new_from_gl(
			ctx.gl.clone(),
			(image_data.width, image_data.height),
			Some(&image_data.pixels),
			settings,
		)
	}

	pub(crate) fn new_from_gl(
		gl: Rc<glow::Context>,
		size: (u32, u32),
		pixels: Option<&[u8]>,
		settings: TextureSettings,
	) -> Result<Self, GlError> {
		let texture = unsafe { gl.create_texture() }.map_err(GlError)?;
		unsafe {
			gl.bind_texture(glow::TEXTURE_2D, Some(texture));
			gl.tex_parameter_i32(
				glow::TEXTURE_2D,
				glow::TEXTURE_WRAP_S,
				settings.wrapping.as_u32().try_into().unwrap(),
			);
			gl.tex_parameter_i32(
				glow::TEXTURE_2D,
				glow::TEXTURE_WRAP_T,
				settings.wrapping.as_u32().try_into().unwrap(),
			);
			if let TextureWrapping::ClampToBorder(color) = settings.wrapping {
				let color: [f32; 4] = color.into();
				gl.tex_parameter_f32_slice(glow::TEXTURE_2D, glow::TEXTURE_BORDER_COLOR, &color);
			}
			gl.tex_parameter_i32(
				glow::TEXTURE_2D,
				glow::TEXTURE_MIN_FILTER,
				settings.minifying_filter.as_u32().try_into().unwrap(),
			);
			gl.tex_parameter_i32(
				glow::TEXTURE_2D,
				glow::TEXTURE_MAG_FILTER,
				settings.magnifying_filter.as_u32().try_into().unwrap(),
			);
			gl.tex_image_2d(
				glow::TEXTURE_2D,
				0,
				glow::RGBA as i32,
				size.0.try_into().expect("Texture is too wide"),
				size.1.try_into().expect("Texture is too tall"),
				0,
				glow::RGBA,
				glow::UNSIGNED_BYTE,
				pixels,
			);
			gl.generate_mipmap(glow::TEXTURE_2D);
		}
		Ok(Self { gl, texture, size })
	}

	pub fn from_file(
		ctx: &Context,
		path: impl AsRef<Path>,
		settings: TextureSettings,
	) -> Result<Self, LoadTextureError> {
		let image_data = ImageData::load(path)?;
		Self::from_image_data(ctx, &image_data, settings)
			.map_err(|error| LoadTextureError::GlError(error.0))
	}

	pub fn size(&self) -> (u32, u32) {
		self.size
	}

	pub fn relative_rect(&self, absolute_rect: Rect) -> Rect {
		let size = Vec2::new(self.size.0 as f32, self.size.1 as f32);
		Rect {
			top_left: absolute_rect.top_left / size,
			bottom_right: absolute_rect.bottom_right / size,
		}
	}

	pub fn replace(&self, x: i32, y: i32, image_data: &ImageData) {
		unsafe {
			self.gl.bind_texture(glow::TEXTURE_2D, Some(self.texture));
			self.gl.tex_sub_image_2d(
				glow::TEXTURE_2D,
				0,
				x,
				y,
				image_data.width.try_into().expect("Image data too wide"),
				image_data.height.try_into().expect("Image data too tall"),
				glow::RGBA,
				glow::UNSIGNED_BYTE,
				PixelUnpackData::Slice(&image_data.pixels),
			);
		}
	}

	pub fn draw<'a>(
		&self,
		ctx: &Context,
		params: impl Into<DrawParams<'a>>,
	) -> Result<(), GlError> {
		Mesh::rectangle(
			ctx,
			Rect::new(
				Vec2::ZERO,
				Vec2::new(self.size.0 as f32, self.size.1 as f32),
			),
		)?
		.draw_textured(ctx, self, params);
		Ok(())
	}

	pub fn draw_region<'a>(
		&self,
		ctx: &Context,
		region: Rect,
		params: impl Into<DrawParams<'a>>,
	) -> Result<(), GlError> {
		Mesh::rectangle_with_texture_region(
			ctx,
			Rect::new(Vec2::ZERO, region.size()),
			self.relative_rect(region),
		)?
		.draw_textured(ctx, self, params);
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

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct TextureSettings {
	pub wrapping: TextureWrapping,
	pub minifying_filter: TextureFilter,
	pub magnifying_filter: TextureFilter,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TextureWrapping {
	Repeat,
	MirroredRepeat,
	ClampToEdge,
	ClampToBorder(Rgba),
}

impl TextureWrapping {
	fn as_u32(&self) -> u32 {
		match self {
			TextureWrapping::Repeat => glow::REPEAT,
			TextureWrapping::MirroredRepeat => glow::MIRRORED_REPEAT,
			TextureWrapping::ClampToEdge => glow::CLAMP_TO_EDGE,
			TextureWrapping::ClampToBorder(_) => glow::CLAMP_TO_BORDER,
		}
	}
}

impl Default for TextureWrapping {
	fn default() -> Self {
		Self::Repeat
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TextureFilter {
	Nearest,
	Linear,
}

impl TextureFilter {
	fn as_u32(&self) -> u32 {
		match self {
			TextureFilter::Nearest => glow::NEAREST,
			TextureFilter::Linear => glow::LINEAR,
		}
	}
}

impl Default for TextureFilter {
	fn default() -> Self {
		Self::Nearest
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
