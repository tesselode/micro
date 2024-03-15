use std::{path::Path, rc::Rc, sync::mpsc::Sender};

use glam::{IVec2, Mat4, UVec2, Vec2, Vec3};
use glow::{HasContext, NativeTexture, PixelUnpackData};
use image::{ImageBuffer, ImageError};
use palette::LinSrgba;
use thiserror::Error;

use crate::{context::Context, graphics::mesh::Mesh, math::Rect};

use super::{
	shader::Shader,
	sprite_batch::{SpriteBatch, SpriteParams},
	unused_resource::UnusedGraphicsResource,
	BlendMode, ColorConstants, NineSlice,
};

#[derive(Debug, Clone)]
pub struct Texture {
	pub(crate) inner: Rc<TextureInner>,
	pub region: Rect,
	pub shader: Option<Shader>,
	pub transform: Mat4,
	pub color: LinSrgba,
	pub blend_mode: BlendMode,
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

	pub fn region(&self, region: Rect) -> Self {
		let mut new = self.clone();
		new.region = region;
		new
	}

	pub fn shader<'a>(&self, shader: impl Into<Option<&'a Shader>>) -> Self {
		let mut new = self.clone();
		new.shader = shader.into().cloned();
		new
	}

	pub fn transformed(&self, transform: impl Into<Mat4>) -> Self {
		let mut new = self.clone();
		new.transform = transform.into() * self.transform;
		new
	}

	pub fn translated_2d(&self, translation: impl Into<Vec2>) -> Self {
		self.transformed(Mat4::from_translation(translation.into().extend(0.0)))
	}

	pub fn translated_3d(&self, translation: impl Into<Vec3>) -> Self {
		self.transformed(Mat4::from_translation(translation.into()))
	}

	pub fn scaled_2d(&self, scale: impl Into<Vec2>) -> Self {
		self.transformed(Mat4::from_scale(scale.into().extend(1.0)))
	}

	pub fn scaled_3d(&self, scale: impl Into<Vec3>) -> Self {
		self.transformed(Mat4::from_scale(scale.into()))
	}

	pub fn rotated_x(&self, rotation: f32) -> Self {
		self.transformed(Mat4::from_rotation_x(rotation))
	}

	pub fn rotated_y(&self, rotation: f32) -> Self {
		self.transformed(Mat4::from_rotation_y(rotation))
	}

	pub fn rotated_z(&self, rotation: f32) -> Self {
		self.transformed(Mat4::from_rotation_z(rotation))
	}

	pub fn color(&self, color: impl Into<LinSrgba>) -> Self {
		let mut new = self.clone();
		new.color = color.into();
		new
	}

	pub fn blend_mode(&self, blend_mode: BlendMode) -> Self {
		let mut new = self.clone();
		new.blend_mode = blend_mode;
		new
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

	pub fn draw(&self) {
		Mesh::rectangle_with_texture_region(
			Rect::new(Vec2::ZERO, self.region.size),
			self.relative_rect(self.region),
		)
		.texture(self)
		.shader(&self.shader)
		.transformed(self.transform)
		.color(self.color)
		.blend_mode(self.blend_mode)
		.draw();
	}

	pub fn draw_nine_slice(&self, nine_slice: NineSlice, display_rect: Rect) {
		let mut sprite_batch = SpriteBatch::new(self, 9);
		sprite_batch
			.add_nine_slice(nine_slice, display_rect, SpriteParams::default())
			.unwrap();
		sprite_batch
			.shader(&self.shader)
			.transformed(self.transform)
			.color(self.color)
			.blend_mode(self.blend_mode)
			.draw();
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
			region: Rect::new(Vec2::ZERO, size.as_vec2()),
			shader: None,
			transform: Mat4::IDENTITY,
			color: LinSrgba::WHITE,
			blend_mode: BlendMode::default(),
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
