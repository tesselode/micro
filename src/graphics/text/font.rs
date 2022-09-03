use std::{collections::HashMap, path::Path};

use crunch::pack_into_po2;
use glam::{UVec2, Vec2};
use thiserror::Error;

use crate::{
	context::Context,
	graphics::{
		image_data::ImageData,
		texture::{Texture, TextureSettings},
	},
	math::Rect,
};

const GLYPH_PADDING: usize = 2;

pub struct Font {
	pub(crate) font: fontdue::Font,
	pub(crate) scale: f32,
	pub(crate) texture: Texture,
	pub(crate) glyph_rects: HashMap<char, Rect>,
}

impl Font {
	pub fn from_file(
		ctx: &Context,
		path: impl AsRef<Path>,
		settings: FontSettings,
	) -> Result<Self, LoadFontError> {
		Self::from_bytes(ctx, &std::fs::read(path)?, settings)
	}

	pub fn from_bytes(
		ctx: &Context,
		data: &[u8],
		settings: FontSettings,
	) -> Result<Self, LoadFontError> {
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
			UVec2::new(
				width.try_into().expect("Packed glyphs are too wide"),
				height.try_into().expect("Packed glyphs are too tall"),
			),
			&glyph_image_data,
			&glyph_rects,
			texture_settings,
		);
		Ok(Self {
			font,
			scale,
			texture,
			glyph_rects,
		})
	}
}

#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug, Error)]
pub enum LoadFontError {
	#[error("{0}")]
	IoError(#[from] std::io::Error),
	#[error("{0}")]
	FontError(&'static str),
}

fn rasterize_chars(font: &fontdue::Font, settings: FontSettings) -> HashMap<char, ImageData> {
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
				ImageData {
					size: UVec2::new(
						metrics.width.try_into().expect("Glyph too wide"),
						metrics.height.try_into().expect("Glyph too tall"),
					),
					pixels,
				},
			)
		})
		.collect()
}

fn pack_glyphs(glyph_image_data: &HashMap<char, ImageData>) -> (usize, usize, HashMap<char, Rect>) {
	let (width, height, packed_items) = pack_into_po2(
		usize::MAX,
		glyph_image_data.iter().map(|(char, image_data)| {
			let base_width: usize = image_data.size.x.try_into().unwrap();
			let base_height: usize = image_data.size.y.try_into().unwrap();
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
		packed_items
			.into_iter()
			.map(|(rect, char)| {
				(
					*char,
					Rect::from_top_left_and_size(
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
	glyph_image_data: &HashMap<char, ImageData>,
	glyph_rects: &HashMap<char, Rect>,
	texture_settings: TextureSettings,
) -> Texture {
	let texture = Texture::empty(ctx, size, texture_settings);
	for (char, rect) in glyph_rects {
		texture.replace(
			rect.top_left.x as i32,
			rect.top_left.y as i32,
			glyph_image_data.get(char).expect("No image data for glyph"),
		);
	}
	texture
}
