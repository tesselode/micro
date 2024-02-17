use std::{path::Path, rc::Rc, sync::mpsc::Sender};

use glam::{IVec2, UVec2, Vec2};
use glow::{HasContext, NativeTexture, PixelUnpackData};
use image::{ImageBuffer, ImageError};
use palette::LinSrgba;
use thiserror::Error;

use crate::{
	context::Context,
	graphics::{draw_params::DrawParams, mesh::Mesh},
	math::Rect,
};

use super::{
	sprite_batch::{SpriteBatch, SpriteParams},
	unused_resource::UnusedGraphicsResource,
	NineSlice,
};

#[derive(Debug, Clone)]
pub struct Texture {
	pub(crate) inner: Rc<TextureInner>,
}

#[derive(Debug)]
pub(crate) struct TextureInner {
	pub texture: NativeTexture,
	pub size: UVec2,
	unused_resource_sender: Sender<UnusedGraphicsResource>,
}

impl Texture {
	pub fn empty(ctx: &Context, size: UVec2, settings: TextureSettings) -> Self {
		Self::new_from_gl(
			&ctx.graphics.gl,
			ctx.graphics.unused_resource_sender.clone(),
			size,
			None,
			settings,
			false,
		)
	}

	pub fn from_image(
		ctx: &Context,
		image: &ImageBuffer<image::Rgba<u8>, Vec<u8>>,
		settings: TextureSettings,
	) -> Self {
		Self::new_from_gl(
			&ctx.graphics.gl,
			ctx.graphics.unused_resource_sender.clone(),
			UVec2::new(image.width(), image.height()),
			Some(image.as_raw()),
			settings,
			false,
		)
	}

	pub fn from_file(
		ctx: &Context,
		path: impl AsRef<Path>,
		settings: TextureSettings,
	) -> Result<Self, LoadTextureError> {
		let image = image::io::Reader::open(path)?.decode()?.to_rgba8();
		Ok(Self::from_image(ctx, &image, settings))
	}

	pub fn size(&self) -> UVec2 {
		self.inner.size
	}

	pub fn relative_rect(&self, absolute_rect: Rect) -> Rect {
		let size = self.inner.size.as_vec2();
		Rect::from_top_left_and_bottom_right(
			absolute_rect.top_left / size,
			absolute_rect.bottom_right() / size,
		)
	}

	pub fn replace(
		&self,
		ctx: &Context,
		top_left: IVec2,
		image: &ImageBuffer<image::Rgba<u8>, Vec<u8>>,
	) {
		let gl = &ctx.graphics.gl;
		unsafe {
			gl.bind_texture(glow::TEXTURE_2D, Some(self.inner.texture));
			gl.tex_sub_image_2d(
				glow::TEXTURE_2D,
				0,
				top_left.x,
				top_left.y,
				image.width() as i32,
				image.height() as i32,
				glow::RGBA,
				glow::UNSIGNED_BYTE,
				PixelUnpackData::Slice(image.as_raw()),
			);
		}
	}

	pub fn draw<'a>(&self, ctx: &Context, params: impl Into<DrawParams<'a>>) {
		Mesh::rectangle(ctx, Rect::new(Vec2::ZERO, self.inner.size.as_vec2()))
			.draw_textured(ctx, self, params);
	}

	pub fn draw_region<'a>(&self, ctx: &Context, region: Rect, params: impl Into<DrawParams<'a>>) {
		Mesh::rectangle_with_texture_region(
			ctx,
			Rect::new(Vec2::ZERO, region.size),
			self.relative_rect(region),
		)
		.draw_textured(ctx, self, params);
	}

	pub fn draw_nine_slice<'a>(
		&self,
		ctx: &Context,
		nine_slice: NineSlice,
		display_rect: Rect,
		params: impl Into<DrawParams<'a>>,
	) {
		let mut sprite_batch = SpriteBatch::new(ctx, self, 9);
		sprite_batch
			.add_nine_slice(ctx, nine_slice, display_rect, SpriteParams::default())
			.unwrap();
		sprite_batch.draw(ctx, params)
	}

	pub(crate) fn new_from_gl(
		gl: &glow::Context,
		unused_resource_sender: Sender<UnusedGraphicsResource>,
		size: UVec2,
		pixels: Option<&[u8]>,
		settings: TextureSettings,
		float: bool,
	) -> Self {
		let texture = unsafe { gl.create_texture().expect("error creating texture") };
		unsafe {
			gl.bind_texture(glow::TEXTURE_2D, Some(texture));
			gl.tex_parameter_i32(
				glow::TEXTURE_2D,
				glow::TEXTURE_WRAP_S,
				settings.wrapping.as_u32() as i32,
			);
			gl.tex_parameter_i32(
				glow::TEXTURE_2D,
				glow::TEXTURE_WRAP_T,
				settings.wrapping.as_u32() as i32,
			);
			if let TextureWrapping::ClampToBorder(color) = settings.wrapping {
				let color: [f32; 4] = color.into();
				gl.tex_parameter_f32_slice(glow::TEXTURE_2D, glow::TEXTURE_BORDER_COLOR, &color);
			}
			gl.tex_parameter_i32(
				glow::TEXTURE_2D,
				glow::TEXTURE_MIN_FILTER,
				settings.minifying_filter.as_u32() as i32,
			);
			gl.tex_parameter_i32(
				glow::TEXTURE_2D,
				glow::TEXTURE_MAG_FILTER,
				settings.magnifying_filter.as_u32() as i32,
			);
			gl.tex_image_2d(
				glow::TEXTURE_2D,
				0,
				if float {
					glow::RGBA16F
				} else {
					glow::SRGB8_ALPHA8
				} as i32,
				size.x as i32,
				size.y as i32,
				0,
				glow::RGBA,
				if float {
					glow::FLOAT
				} else {
					glow::UNSIGNED_BYTE
				},
				pixels,
			);
			gl.generate_mipmap(glow::TEXTURE_2D);
		}
		Self {
			inner: Rc::new(TextureInner {
				texture,
				size,
				unused_resource_sender,
			}),
		}
	}

	pub(crate) fn attach_to_framebuffer(&mut self, ctx: &Context) {
		let gl = &ctx.graphics.gl;
		unsafe {
			gl.bind_texture(glow::TEXTURE_2D, Some(self.inner.texture));
			gl.framebuffer_texture_2d(
				glow::FRAMEBUFFER,
				glow::COLOR_ATTACHMENT0,
				glow::TEXTURE_2D,
				Some(self.inner.texture),
				0,
			);
		}
	}
}

impl Drop for TextureInner {
	fn drop(&mut self) {
		self.unused_resource_sender
			.send(UnusedGraphicsResource::Texture(self.texture))
			.ok();
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
#[cfg_attr(feature = "serializing", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serializing", serde(default))]
pub struct TextureSettings {
	pub wrapping: TextureWrapping,
	pub minifying_filter: TextureFilter,
	pub magnifying_filter: TextureFilter,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serializing", derive(serde::Serialize, serde::Deserialize))]
pub enum TextureWrapping {
	Repeat,
	MirroredRepeat,
	ClampToEdge,
	ClampToBorder(LinSrgba),
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
#[cfg_attr(feature = "serializing", derive(serde::Serialize, serde::Deserialize))]
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
	IoError(#[from] std::io::Error),
	#[error("{0}")]
	ImageError(#[from] ImageError),
}
