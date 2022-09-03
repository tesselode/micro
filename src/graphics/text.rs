mod font;

pub use font::*;

use fontdue::layout::{CoordinateSystem, Layout, TextStyle};
use glam::Vec2;

use crate::{context::Context, math::Rect, IntoOffsetAndCount};

use super::{
	draw_params::DrawParams,
	sprite_batch::{SpriteBatch, SpriteParams},
};

pub struct Text {
	pub(crate) sprite_batch: SpriteBatch,
	pub(crate) bounds: Option<Rect>,
}

impl Text {
	pub fn new(ctx: &Context, font: &Font, text: &str) -> Self {
		let mut layout = Layout::new(CoordinateSystem::PositiveYDown);
		layout.append(
			&[&font.font],
			&TextStyle {
				text,
				px: font.scale,
				font_index: 0,
				user_data: (),
			},
		);
		let glyphs = layout.glyphs();
		let mut sprite_batch = SpriteBatch::new(ctx, &font.texture, glyphs.len());
		let mut bounds: Option<Rect> = None;
		for glyph in glyphs {
			if !glyph.char_data.rasterize() {
				continue;
			}
			let display_rect = Rect::from_top_left_and_size(
				Vec2::new(glyph.x, glyph.y),
				Vec2::new(glyph.width as f32, glyph.height as f32),
			);
			if let Some(bounds) = &mut bounds {
				*bounds = bounds.union(display_rect);
			} else {
				bounds = Some(display_rect);
			}
			let texture_rect = *font
				.glyph_rects
				.get(&glyph.parent)
				.unwrap_or_else(|| panic!("No glyph rect for the character {}", glyph.parent));
			sprite_batch
				.add_region(
					texture_rect,
					SpriteParams::new().position(Vec2::new(glyph.x, glyph.y)),
				)
				.expect("Not enough capacity in the sprite batch");
		}
		Self {
			sprite_batch,
			bounds,
		}
	}

	pub fn num_glyphs(&self) -> usize {
		self.sprite_batch.len()
	}

	pub fn bounds(&self) -> Option<Rect> {
		self.bounds
	}

	pub fn draw<'a>(&self, ctx: &mut Context, params: impl Into<DrawParams<'a>>) {
		self.sprite_batch.draw(ctx, params);
	}

	pub fn draw_range<'a>(
		&self,
		ctx: &mut Context,
		range: impl IntoOffsetAndCount,
		params: impl Into<DrawParams<'a>>,
	) {
		self.sprite_batch.draw_range(ctx, range, params);
	}
}
