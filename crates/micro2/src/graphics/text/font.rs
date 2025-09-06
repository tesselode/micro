use std::{collections::HashMap, path::Path, sync::Arc};

use crunch::{PackedItem, PackedItems, pack_into_po2};
use derive_more::derive::{Display, Error, From};
use glam::{UVec2, Vec2};
use image::ImageBuffer;

use crate::{
	Context,
	graphics::texture::{Texture, TextureSettings},
	math::Rect,
};

const GLYPH_PADDING: usize = 2;

#[derive(Clone)]
pub struct Font {
	pub(crate) inner: Arc<FontInner>,
}

impl Font {
	pub fn from_file(
		ctx: &Context,
		path: impl AsRef<Path>,
		settings: FontSettings,
	) -> Result<Self, LoadFontError> {
		let _span = tracy_client::span!();
		Self::from_bytes(ctx, &std::fs::read(path)?, settings)
	}

	pub fn from_bytes(
		ctx: &Context,
		data: &[u8],
		settings: FontSettings,
	) -> Result<Self, LoadFontError> {
		let _span = tracy_client::span!();
		let scale = settings.scale;
		let texture_settings = settings.texture_settings;
		let font = fontdue::Font::from_bytes(
			data,
			fontdue::FontSettings {
				scale,
				..Default::default()
			},
		)
		.map_err(LoadFontError::FontError)?;
		let glyph_image_data = rasterize_chars(&font, settings);
		let (width, height, glyph_rects) = pack_glyphs(&glyph_image_data);
		let texture = create_texture(
			ctx,
			UVec2::new(width as u32, height as u32),
			&glyph_image_data,
			&glyph_rects,
			texture_settings,
		);
		Ok(Self {
			inner: Arc::new(FontInner {
				font,
				scale,
				texture,
				glyph_rects,
			}),
		})
	}

	pub fn has_glyph(&self, glyph: char) -> bool {
		self.inner.glyph_rects.contains_key(&glyph)
	}
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serializing", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serializing", serde(default))]
pub struct FontSettings {
	pub scale: f32,
	pub chars: String,
	pub texture_settings: TextureSettings,
}

impl Default for FontSettings {
	fn default() -> Self {
		Self {
			scale: 16.0,
			chars: (32u8..127u8).map(|code| code as char).collect(),
			texture_settings: TextureSettings::default(),
		}
	}
}

#[derive(Debug, Error, Display, From)]
pub enum LoadFontError {
	IoError(std::io::Error),
	FontError(#[error(not(source))] &'static str),
}

pub(crate) struct FontInner {
	pub(crate) font: fontdue::Font,
	pub(crate) scale: f32,
	pub(crate) texture: Texture,
	pub(crate) glyph_rects: HashMap<char, Rect>,
}

fn rasterize_chars(
	font: &fontdue::Font,
	settings: FontSettings,
) -> HashMap<char, ImageBuffer<image::Rgba<u8>, Vec<u8>>> {
	settings
		.chars
		.chars()
		.map(|char| {
			let (metrics, bitmap) = font.rasterize(char, settings.scale);
			let mut pixels = Vec::with_capacity(bitmap.len() * 4);
			for alpha in &bitmap {
				pixels.extend_from_slice(&[255, 255, 255, *alpha]);
			}
			(
				char,
				ImageBuffer::from_vec(metrics.width as u32, metrics.height as u32, pixels)
					.expect("buffer too small"),
			)
		})
		.collect()
}

fn pack_glyphs(
	glyph_image_data: &HashMap<char, ImageBuffer<image::Rgba<u8>, Vec<u8>>>,
) -> (usize, usize, HashMap<char, Rect>) {
	let PackedItems {
		w: width,
		h: height,
		items,
	} = pack_into_po2(
		usize::MAX,
		glyph_image_data.iter().map(|(char, image_data)| {
			let base_width: usize = image_data.width() as usize;
			let base_height: usize = image_data.height() as usize;
			crunch::Item {
				data: char,
				w: base_width + GLYPH_PADDING * 2,
				h: base_height + GLYPH_PADDING * 2,
				rot: crunch::Rotation::None,
			}
		}),
	)
	.expect("Could not pack glyphs");
	(
		width,
		height,
		items
			.into_iter()
			.map(|PackedItem { data: char, rect }| {
				(
					*char,
					Rect::new(
						Vec2::new(
							(rect.x + GLYPH_PADDING) as f32,
							(rect.y + GLYPH_PADDING) as f32,
						),
						Vec2::new(
							(rect.w - GLYPH_PADDING * 2) as f32,
							(rect.h - GLYPH_PADDING * 2) as f32,
						),
					),
				)
			})
			.collect(),
	)
}

fn create_texture(
	ctx: &Context,
	size: UVec2,
	glyph_image_data: &HashMap<char, ImageBuffer<image::Rgba<u8>, Vec<u8>>>,
	glyph_rects: &HashMap<char, Rect>,
	texture_settings: TextureSettings,
) -> Texture {
	let texture = Texture::empty(ctx, size, texture_settings);
	for (char, rect) in glyph_rects {
		texture.replace(
			ctx,
			rect.top_left.as_uvec2(),
			glyph_image_data.get(char).expect("No image data for glyph"),
		);
	}
	texture
}
