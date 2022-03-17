use std::{collections::HashMap, path::Path, rc::Rc};

use crunch::pack_into_po2;
use glam::Vec2;
use thiserror::Error;

use crate::{
	context::Context, error::GlError, image_data::ImageData, rect::Rect, texture::Texture,
};

const GLYPH_PADDING: usize = 2;

pub struct Font {
	pub(crate) texture: Rc<Texture>,
	pub(crate) glyph_rects: HashMap<char, Rect>,
}

impl Font {
	pub fn from_file(
		ctx: &mut Context,
		path: impl AsRef<Path>,
		settings: FontSettings,
	) -> Result<Self, LoadFontError> {
		Self::from_bytes(ctx, &std::fs::read(path)?, settings)
	}

	pub fn from_bytes(
		ctx: &mut Context,
		data: &[u8],
		settings: FontSettings,
	) -> Result<Self, LoadFontError> {
		let font = fontdue::Font::from_bytes(
			data,
			fontdue::FontSettings {
				scale: settings.scale,
				..Default::default()
			},
		)
		.map_err(LoadFontError::FontError)?;
		let glyph_image_data = rasterize_chars(&font, settings);
		let (width, height, glyph_rects) = pack_glyphs(&glyph_image_data);
		let texture = create_texture(ctx, width, height, &glyph_image_data, &glyph_rects)
			.map_err(|error| LoadFontError::GlError(error.0))?;
		Ok(Self {
			texture: Rc::new(texture),
			glyph_rects,
		})
	}
}

pub struct FontSettings {
	pub scale: f32,
	pub chars: String,
}

impl Default for FontSettings {
	fn default() -> Self {
		Self {
			scale: 16.0,
			chars: (32u8..127u8).map(|code| code as char).collect(),
		}
	}
}

#[derive(Debug, Error)]
pub enum LoadFontError {
	#[error("{0}")]
	IoError(#[from] std::io::Error),
	#[error("{0}")]
	FontError(&'static str),
	#[error("{0}")]
	GlError(String),
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
					width: metrics.width.try_into().expect("Glyph too wide"),
					height: metrics.height.try_into().expect("Glyph too tall"),
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
			let base_width: usize = image_data.width.try_into().unwrap();
			let base_height: usize = image_data.height.try_into().unwrap();
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
					Rect {
						top_left: Vec2::new(rect.x as f32, rect.y as f32),
						size: Vec2::new(rect.w as f32, rect.h as f32),
					},
				)
			})
			.collect(),
	)
}

fn create_texture(
	ctx: &mut Context,
	width: usize,
	height: usize,
	glyph_image_data: &HashMap<char, ImageData>,
	glyph_rects: &HashMap<char, Rect>,
) -> Result<Texture, GlError> {
	let texture = Texture::empty(
		ctx,
		width.try_into().expect("Packed glyphs are too wide"),
		height.try_into().expect("Packed glyphs are too tall"),
	)?;
	for (char, rect) in glyph_rects {
		texture.replace(
			(rect.top_left.x + GLYPH_PADDING as f32) as i32,
			(rect.top_left.y + GLYPH_PADDING as f32) as i32,
			glyph_image_data.get(char).expect("No image data for glyph"),
		);
	}
	Ok(texture)
}
