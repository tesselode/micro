use std::{path::Path, rc::Rc};

use glow::{HasContext, NativeTexture, PixelUnpackData};
use thiserror::Error;
use vek::Vec2;

use crate::{
	context::Context,
	graphics::{
		draw_params::DrawParams,
		image_data::{ImageData, LoadImageDataError},
		mesh::Mesh,
	},
	math::Rect,
};

use super::color::Rgba;

#[derive(Debug, Clone)]
pub struct Texture {
	pub(crate) inner: Rc<TextureInner>,
}

#[derive(Debug)]
pub(crate) struct TextureInner {
	gl: Rc<glow::Context>,
	pub texture: NativeTexture,
	pub size: Vec2<u32>,
}

impl Texture {
	pub fn empty(ctx: &Context, size: Vec2<u32>, settings: TextureSettings) -> Self {
		Self::new_from_gl(ctx.gl.clone(), size, None, settings)
	}

	pub fn from_image_data(
		ctx: &Context,
		image_data: &ImageData,
		settings: TextureSettings,
	) -> Self {
		Self::new_from_gl(
			ctx.gl.clone(),
			image_data.size,
			Some(&image_data.pixels),
			settings,
		)
	}

	pub(crate) fn new_from_gl(
		gl: Rc<glow::Context>,
		size: Vec2<u32>,
		pixels: Option<&[u8]>,
		settings: TextureSettings,
	) -> Self {
		let texture = unsafe { gl.create_texture().unwrap() };
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
				size.x.try_into().expect("Texture is too wide"),
				size.y.try_into().expect("Texture is too tall"),
				0,
				glow::RGBA,
				glow::UNSIGNED_BYTE,
				pixels,
			);
			gl.generate_mipmap(glow::TEXTURE_2D);
		}
		Self {
			inner: Rc::new(TextureInner { gl, texture, size }),
		}
	}

	pub fn from_file(
		ctx: &Context,
		path: impl AsRef<Path>,
		settings: TextureSettings,
	) -> Result<Self, LoadImageDataError> {
		let image_data = ImageData::load(path)?;
		Ok(Self::from_image_data(ctx, &image_data, settings))
	}

	pub fn size(&self) -> Vec2<u32> {
		self.inner.size
	}

	pub fn relative_rect(&self, absolute_rect: Rect) -> Rect {
		let size = self.inner.size.as_::<f32>();
		Rect {
			top_left: absolute_rect.top_left / size,
			bottom_right: absolute_rect.bottom_right / size,
		}
	}

	pub fn replace(&self, x: i32, y: i32, image_data: &ImageData) {
		unsafe {
			self.inner
				.gl
				.bind_texture(glow::TEXTURE_2D, Some(self.inner.texture));
			self.inner.gl.tex_sub_image_2d(
				glow::TEXTURE_2D,
				0,
				x,
				y,
				image_data.size.x.try_into().expect("Image data too wide"),
				image_data.size.y.try_into().expect("Image data too tall"),
				glow::RGBA,
				glow::UNSIGNED_BYTE,
				PixelUnpackData::Slice(&image_data.pixels),
			);
		}
	}

	pub fn draw<'a>(&self, ctx: &Context, params: impl Into<DrawParams<'a>>) {
		Mesh::rectangle(ctx, Rect::new(Vec2::zero(), self.inner.size.as_::<f32>()))
			.draw_textured(ctx, self, params);
	}

	pub fn draw_region<'a>(&self, ctx: &Context, region: Rect, params: impl Into<DrawParams<'a>>) {
		Mesh::rectangle_with_texture_region(
			ctx,
			Rect::new(Vec2::zero(), region.size()),
			self.relative_rect(region),
		)
		.draw_textured(ctx, self, params);
	}

	pub(crate) fn attach_to_framebuffer(&mut self) {
		unsafe {
			self.inner
				.gl
				.bind_texture(glow::TEXTURE_2D, Some(self.inner.texture));
			self.inner.gl.framebuffer_texture_2d(
				glow::FRAMEBUFFER,
				glow::COLOR_ATTACHMENT0,
				glow::TEXTURE_2D,
				Some(self.inner.texture),
				0,
			);
		}
	}

	pub fn bind_to_slot(&self, slot: u32) {
		unsafe {
			self.inner.gl.active_texture(glow::TEXTURE0 + slot);
			self.inner
				.gl
				.bind_texture(glow::TEXTURE_2D, Some(self.inner.texture));
			self.inner.gl.active_texture(glow::TEXTURE0);
		}
	}
}

impl Drop for TextureInner {
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
