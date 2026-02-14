use std::{collections::HashMap, path::Path};

use cosmic_text::{CacheKey, FontSystem, SwashCache, fontdb::Database};
use etagere::{Allocation, AtlasAllocator, size2};
use glam::{IVec2, ivec2, uvec2};
use image::RgbaImage;
use wgpu::{FilterMode, Queue};

use crate::{
	context::graphics::GraphicsContext,
	graphics::texture::{InternalTextureSettings, Texture, TextureSettings},
	math::{IRect, Rect},
};

pub(crate) struct TextContext {
	pub(crate) font_system: FontSystem,
	swash_cache: SwashCache,
	atlas_allocator: AtlasAllocator,
	glyphs: HashMap<CacheKey, GlyphAllocation>,
	pub(crate) texture: Texture,
}

impl TextContext {
	pub fn new(graphics: &GraphicsContext) -> Self {
		Self {
			font_system: FontSystem::new_with_locale_and_db("en-US".to_string(), Database::new()),
			swash_cache: SwashCache::new(),
			atlas_allocator: AtlasAllocator::new(size2(8192, 8192)),
			glyphs: HashMap::new(),
			texture: Texture::new(
				&graphics.device,
				&graphics.queue,
				uvec2(8192, 8192),
				1,
				None,
				TextureSettings {
					minifying_filter: FilterMode::Linear,
					magnifying_filter: FilterMode::Linear,
					..Default::default()
				},
				InternalTextureSettings {
					format: graphics.surface_format(),
					sample_count: 1,
				},
			),
		}
	}

	pub fn load_font_file(&mut self, path: impl AsRef<Path>) -> std::io::Result<()> {
		self.font_system.db_mut().load_font_file(path)
	}

	pub fn load_fonts_dir(&mut self, path: impl AsRef<Path>) {
		self.font_system.db_mut().load_fonts_dir(path);
	}

	pub fn glyph_rect(&mut self, queue: &Queue, cache_key: CacheKey) -> Option<GlyphInfo> {
		self.glyphs
			.get(&cache_key)
			.map(|allocation| GlyphInfo {
				texture_rect: etagere_rectangle_to_irect(allocation.allocation.rectangle),
				offset: allocation.offset,
			})
			.or_else(|| self.insert_glyph_rect(queue, cache_key))
	}

	fn insert_glyph_rect(&mut self, queue: &Queue, cache_key: CacheKey) -> Option<GlyphInfo> {
		let mut min_x = 0;
		let mut min_y = 0;
		let mut max_x = 0;
		let mut max_y = 0;
		self.swash_cache.with_pixels(
			&mut self.font_system,
			cache_key,
			cosmic_text::Color::rgb(0xff, 0xff, 0xff),
			|x, y, _| {
				min_x = min_x.min(x);
				min_y = min_y.min(y);
				max_x = max_x.max(x);
				max_y = max_y.max(y);
			},
		);
		let width = (max_x - min_x + 1) as u32;
		let height = (max_y - min_y + 1) as u32;
		let mut image = RgbaImage::new(width, height);
		self.swash_cache.with_pixels(
			&mut self.font_system,
			cache_key,
			cosmic_text::Color::rgb(0xff, 0xff, 0xff),
			|x, y, color| {
				image.put_pixel(
					(x - min_x) as u32,
					(y - min_y) as u32,
					color.as_rgba().into(),
				);
			},
		);
		let allocation = self
			.atlas_allocator
			.allocate(size2(width as i32, height as i32))?;
		let rectangle = etagere_rectangle_to_irect(allocation.rectangle);
		let offset = ivec2(min_x, min_y);
		self.texture
			.replace_inner(queue, rectangle.top_left.as_uvec2(), &image);
		self.glyphs
			.insert(cache_key, GlyphAllocation { allocation, offset });
		Some(GlyphInfo {
			texture_rect: rectangle,
			offset,
		})
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct GlyphInfo {
	pub texture_rect: IRect,
	pub offset: IVec2,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct GlyphAllocation {
	allocation: Allocation,
	offset: IVec2,
}

fn etagere_rectangle_to_irect(rectangle: etagere::Rectangle) -> IRect {
	IRect::from_corners(
		ivec2(rectangle.min.x, rectangle.min.y),
		ivec2(rectangle.max.x, rectangle.max.y),
	)
}
