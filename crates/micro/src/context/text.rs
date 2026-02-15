use std::{collections::HashMap, path::Path};

use cosmic_text::{CacheKey, FontSystem, SwashCache, fontdb::Database};
use etagere::{Allocation, BucketedAtlasAllocator, size2};
use glam::{IVec2, UVec2, ivec2};
use image::RgbaImage;
use tracing::warn;
use wgpu::{Device, FilterMode, Queue, TextureFormat};

use crate::{
	context::graphics::GraphicsContext,
	graphics::texture::{InternalTextureSettings, Texture, TextureSettings},
	math::IRect,
};

const STARTING_ATLAS_SIZE: u32 = 256;
const MAX_ATLAS_SIZE: u32 = 8192;
const PADDING: i32 = 1;

pub(crate) struct TextContext {
	pub(crate) font_system: FontSystem,
	swash_cache: SwashCache,
	atlas_allocator: BucketedAtlasAllocator,
	glyphs: HashMap<CacheKey, GlyphAllocation>,
	pub(crate) texture: Texture,
}

impl TextContext {
	pub fn new(graphics: &GraphicsContext) -> Self {
		Self {
			font_system: FontSystem::new_with_locale_and_db("en-US".to_string(), Database::new()),
			swash_cache: SwashCache::new(),
			atlas_allocator: BucketedAtlasAllocator::new(size2(
				STARTING_ATLAS_SIZE as i32,
				STARTING_ATLAS_SIZE as i32,
			)),
			glyphs: HashMap::new(),
			texture: Texture::new(
				&graphics.device,
				&graphics.queue,
				UVec2::splat(STARTING_ATLAS_SIZE),
				1,
				None,
				TextureSettings {
					minifying_filter: FilterMode::Linear,
					magnifying_filter: FilterMode::Linear,
					..Default::default()
				},
				InternalTextureSettings {
					format: TextureFormat::Rgba8UnormSrgb,
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

	pub fn glyph_rect(
		&mut self,
		device: &Device,
		queue: &Queue,
		cache_key: CacheKey,
	) -> Option<GlyphInfo> {
		self.glyphs
			.get(&cache_key)
			.map(|allocation| GlyphInfo {
				texture_rect: etagere_rectangle_to_irect(allocation.allocation.rectangle),
				offset: allocation.offset,
			})
			.or_else(|| self.insert_glyph_rect(device, queue, cache_key))
	}

	fn insert_glyph_rect(
		&mut self,
		device: &Device,
		queue: &Queue,
		cache_key: CacheKey,
	) -> Option<GlyphInfo> {
		let mut min_x: Option<i32> = None;
		let mut min_y: Option<i32> = None;
		let mut max_x: Option<i32> = None;
		let mut max_y: Option<i32> = None;
		self.swash_cache.with_pixels(
			&mut self.font_system,
			cache_key,
			cosmic_text::Color::rgb(0xff, 0xff, 0xff),
			|x, y, color| {
				if color.a() == 0 {
					return;
				}
				min_x = Some(min_x.map(|min_x| min_x.min(x)).unwrap_or(x));
				min_y = Some(min_y.map(|min_y| min_y.min(y)).unwrap_or(y));
				max_x = Some(max_x.map(|max_x| max_x.max(x)).unwrap_or(x));
				max_y = Some(max_y.map(|max_y| max_y.max(y)).unwrap_or(y));
			},
		);
		let (Some(min_x), Some(min_y), Some(max_x), Some(max_y)) = (min_x, min_y, max_x, max_y)
		else {
			return None;
		};
		let width = (max_x - min_x + 1) as u32;
		let height = (max_y - min_y + 1) as u32;
		let mut image = RgbaImage::new(width, height);
		self.swash_cache.with_pixels(
			&mut self.font_system,
			cache_key,
			cosmic_text::Color::rgb(0xff, 0xff, 0xff),
			|x, y, color| {
				if color.a() == 0 {
					return;
				}
				image.put_pixel(
					(x - min_x) as u32,
					(y - min_y) as u32,
					color.as_rgba().into(),
				);
			},
		);
		let TryAllocateResult {
			allocation,
			new_size,
		} = try_allocate(
			&mut self.atlas_allocator,
			width as i32 + PADDING * 2,
			height as i32 + PADDING * 2,
		);
		let allocation = allocation?;
		if let Some(new_size) = new_size {
			self.texture = self
				.texture
				.resized_inner(device, queue, UVec2::splat(new_size));
		}
		let rectangle =
			etagere_rectangle_to_irect(allocation.rectangle).padded((-PADDING, -PADDING));
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

fn try_allocate(
	allocator: &mut BucketedAtlasAllocator,
	width: i32,
	height: i32,
) -> TryAllocateResult {
	let mut new_size = None;
	loop {
		if let Some(allocation) = allocator.allocate(size2(width, height)) {
			return TryAllocateResult {
				allocation: Some(allocation),
				new_size,
			};
		}
		if allocator.size().width as u32 == MAX_ATLAS_SIZE {
			warn!("no more space in text atlas");
			return TryAllocateResult {
				allocation: None,
				new_size: None,
			};
		}
		allocator.grow(allocator.size() * 2);
		new_size = Some(allocator.size().width as u32);
	}
}

struct TryAllocateResult {
	allocation: Option<Allocation>,
	new_size: Option<u32>,
}
