use std::{path::Path, rc::Rc, sync::mpsc::Sender};

use glam::{IVec2, Mat4, UVec2, Vec2};
use glow::{HasContext, NativeTexture, PixelUnpackData};
use image::{ImageBuffer, ImageError};
use palette::LinSrgba;
use thiserror::Error;

use crate::{context::Context, graphics::mesh::Mesh, math::Rect};

use super::{
	shader::Shader,
	sprite_batch::{SpriteBatch, SpriteParams},
	standard_draw_command_methods,
	unused_resource::UnusedGraphicsResource,
	BlendMode, ColorConstants, NineSlice,
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
	pub fn empty(size: UVec2, settings: TextureSettings) -> Self {
		Context::with(|ctx| {
			Self::new_from_gl(
				&ctx.graphics.gl,
				ctx.graphics.unused_resource_sender.clone(),
				size,
				None,
				settings,
				false,
			)
		})
	}

	pub fn from_image(
		image: &ImageBuffer<image::Rgba<u8>, Vec<u8>>,
		settings: TextureSettings,
	) -> Self {
		Context::with(|ctx| {
			Self::new_from_gl(
				&ctx.graphics.gl,
				ctx.graphics.unused_resource_sender.clone(),
				UVec2::new(image.width(), image.height()),
				Some(image.as_raw()),
				settings,
				false,
			)
		})
	}

	pub fn from_file(
		path: impl AsRef<Path>,
		settings: TextureSettings,
	) -> Result<Self, LoadTextureError> {
		let image = image::io::Reader::open(path)?.decode()?.to_rgba8();
		Ok(Self::from_image(&image, settings))
	}

	pub fn size(&self) -> UVec2 {
		self.inner.size
	}

	pub fn relative_rect(&self, absolute_rect: Rect) -> Rect {
		let size = self.inner.size.as_vec2();
		Rect::from_corners(
			absolute_rect.top_left / size,
			absolute_rect.bottom_right() / size,
		)
	}

	pub fn replace(&self, top_left: IVec2, image: &ImageBuffer<image::Rgba<u8>, Vec<u8>>) {
		Context::with(|ctx| {
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
		});
	}

	pub fn draw(&self) -> DrawTextureCommand {
		DrawTextureCommand {
			texture: self,
			params: DrawTextureParams {
				region: Rect::new(Vec2::ZERO, self.size().as_vec2()),
				shader: None,
				transform: Mat4::IDENTITY,
				color: LinSrgba::WHITE,
				blend_mode: BlendMode::default(),
			},
		}
	}

	pub fn draw_nine_slice(
		&self,
		nine_slice: NineSlice,
		display_rect: Rect,
	) -> DrawNineSliceCommand {
		DrawNineSliceCommand {
			texture: self,
			nine_slice,
			display_rect,
			params: DrawNineSliceParams {
				shader: None,
				transform: Mat4::IDENTITY,
				color: LinSrgba::WHITE,
				blend_mode: BlendMode::default(),
			},
		}
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

	pub(crate) fn attach_to_framebuffer(&mut self) {
		Context::with(|ctx| {
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
		});
	}

	fn draw_inner(&self, params: &DrawTextureParams) {
		Mesh::rectangle_with_texture_region(
			Rect::new(Vec2::ZERO, params.region.size),
			self.relative_rect(params.region),
		)
		.draw()
		.texture(self)
		.shader(params.shader)
		.transformed(params.transform)
		.color(params.color)
		.blend_mode(params.blend_mode);
	}

	fn draw_nine_slice_inner(
		&self,
		nine_slice: NineSlice,
		display_rect: Rect,
		params: &DrawNineSliceParams,
	) {
		let mut sprite_batch = SpriteBatch::new(self, 9);
		sprite_batch
			.add_nine_slice(nine_slice, display_rect, SpriteParams::default())
			.unwrap();
		sprite_batch
			.draw()
			.shader(params.shader)
			.transformed(params.transform)
			.color(params.color)
			.blend_mode(params.blend_mode);
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

pub struct DrawTextureParams<'a> {
	pub region: Rect,
	pub shader: Option<&'a Shader>,
	pub transform: Mat4,
	pub color: LinSrgba,
	pub blend_mode: BlendMode,
}

pub struct DrawTextureCommand<'a> {
	texture: &'a Texture,
	params: DrawTextureParams<'a>,
}

impl<'a> DrawTextureCommand<'a> {
	pub fn region(mut self, region: Rect) -> Self {
		self.params.region = region;
		self
	}

	standard_draw_command_methods!();
}

impl<'a> Drop for DrawTextureCommand<'a> {
	fn drop(&mut self) {
		self.texture.draw_inner(&self.params);
	}
}

pub struct DrawNineSliceParams<'a> {
	pub shader: Option<&'a Shader>,
	pub transform: Mat4,
	pub color: LinSrgba,
	pub blend_mode: BlendMode,
}

pub struct DrawNineSliceCommand<'a> {
	texture: &'a Texture,
	nine_slice: NineSlice,
	params: DrawNineSliceParams<'a>,
	display_rect: Rect,
}

impl<'a> DrawNineSliceCommand<'a> {
	standard_draw_command_methods!();
}

impl<'a> Drop for DrawNineSliceCommand<'a> {
	fn drop(&mut self) {
		self.texture
			.draw_nine_slice_inner(self.nine_slice, self.display_rect, &self.params);
	}
}
