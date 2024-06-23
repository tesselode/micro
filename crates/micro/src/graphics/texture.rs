use std::{
	path::Path,
	rc::Rc,
	sync::{
		atomic::{AtomicU64, Ordering},
		Weak,
	},
};

use glam::{IVec2, Mat4, UVec2, Vec2};
use glow::{HasContext, NativeTexture, PixelUnpackData};
use image::{ImageBuffer, ImageError};
use palette::LinSrgba;
use thiserror::Error;

use crate::{context::Context, graphics::mesh::Mesh, math::Rect};

use super::{
	resource::{GraphicsResource, GraphicsResourceId, GraphicsResources},
	shader::Shader,
	sprite_batch::{SpriteBatch, SpriteParams},
	standard_draw_param_methods, BlendMode, ColorConstants, NineSlice,
};

#[derive(Debug, Clone)]
pub struct Texture {
	pub(crate) id: TextureId,
	_weak: Weak<()>,
	size: UVec2,
	pub region: Rect,
	pub shader: Option<Shader>,
	pub transform: Mat4,
	pub color: LinSrgba,
	pub blend_mode: BlendMode,
}

impl Texture {
	pub fn empty(ctx: &mut Context, size: UVec2, settings: TextureSettings) -> Self {
		Self::new(
			ctx.graphics.gl.clone(),
			&mut ctx.graphics.textures,
			size,
			None,
			settings,
			false,
		)
	}

	pub fn from_image(
		ctx: &mut Context,
		image: &ImageBuffer<image::Rgba<u8>, Vec<u8>>,
		settings: TextureSettings,
	) -> Self {
		Self::new(
			ctx.graphics.gl.clone(),
			&mut ctx.graphics.textures,
			UVec2::new(image.width(), image.height()),
			Some(image.as_raw()),
			settings,
			false,
		)
	}

	pub fn from_file(
		ctx: &mut Context,
		path: impl AsRef<Path>,
		settings: TextureSettings,
	) -> Result<Self, LoadTextureError> {
		let image = image::io::Reader::open(path)?.decode()?.to_rgba8();
		Ok(Self::from_image(ctx, &image, settings))
	}

	pub fn region(&self, region: Rect) -> Self {
		let mut new = self.clone();
		new.region = region;
		new
	}

	standard_draw_param_methods!();

	pub fn size(&self) -> UVec2 {
		self.size
	}

	pub fn relative_rect(&self, absolute_rect: Rect) -> Rect {
		let size = self.size.as_vec2();
		Rect::from_corners(
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
		let texture = &ctx.graphics.textures.get(self.id);
		unsafe {
			gl.bind_texture(glow::TEXTURE_2D, Some(texture.texture));
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

	pub fn draw(&self, ctx: &mut Context) {
		Mesh::rectangle_with_texture_region(
			ctx,
			Rect::new(Vec2::ZERO, self.region.size),
			self.relative_rect(self.region),
		)
		.texture(self)
		.shader(&self.shader)
		.transformed(self.transform)
		.color(self.color)
		.blend_mode(self.blend_mode)
		.draw(ctx);
	}

	pub fn draw_nine_slice(&self, ctx: &mut Context, nine_slice: NineSlice, display_rect: Rect) {
		let mut sprite_batch = SpriteBatch::new(ctx, self, 9);
		sprite_batch
			.add_nine_slice(ctx, nine_slice, display_rect, SpriteParams::default())
			.unwrap();
		sprite_batch
			.shader(&self.shader)
			.transformed(self.transform)
			.color(self.color)
			.blend_mode(self.blend_mode)
			.draw(ctx);
	}

	pub(crate) fn new(
		gl: Rc<glow::Context>,
		raw_textures: &mut GraphicsResources<RawTexture>,
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
		let (id, weak) = raw_textures.insert(RawTexture {
			gl: gl.clone(),
			texture,
		});
		Self {
			id,
			_weak: weak,
			size,
			region: Rect::new(Vec2::ZERO, size.as_vec2()),
			shader: None,
			transform: Mat4::IDENTITY,
			color: LinSrgba::WHITE,
			blend_mode: BlendMode::default(),
		}
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

#[derive(Debug)]
pub(crate) struct RawTexture {
	gl: Rc<glow::Context>,
	pub texture: NativeTexture,
}

impl Drop for RawTexture {
	fn drop(&mut self) {
		unsafe {
			self.gl.delete_texture(self.texture);
		}
	}
}

impl GraphicsResource for RawTexture {
	type Id = TextureId;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct TextureId(pub u64);

static NEXT_TEXTURE_ID: AtomicU64 = AtomicU64::new(0);

impl GraphicsResourceId for TextureId {
	fn next() -> Self {
		TextureId(NEXT_TEXTURE_ID.fetch_add(1, Ordering::SeqCst))
	}
}
