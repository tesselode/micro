mod font;

pub use font::*;
use vek::Vec2;

use std::rc::Rc;

use fontdue::layout::{CoordinateSystem, Layout, TextStyle};

use crate::{context::Context, graphics::texture::Texture, math::Rect};

use super::{
	draw_params::DrawParams,
	sprite_batch::{Sprite, SpriteBatch},
};

pub struct Text {
	pub(crate) texture: Rc<Texture>,
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
		let mut sprite_batch = SpriteBatch::new(ctx, glyphs.len());
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
			sprite_batch
				.add(Sprite {
					display_rect,
					texture_rect: *font
						.glyph_rects
						.get(&glyph.parent)
						.expect(&format!("No glyph rect for the character {}", glyph.parent)),
				})
				.expect("Not enough capacity in the sprite batch");
		}
		Self {
			texture: font.texture.clone(),
			sprite_batch,
			bounds,
		}
	}

	pub fn bounds(&self) -> Option<Rect> {
		self.bounds
	}

	pub fn draw<'a>(&self, ctx: &mut Context, params: impl Into<DrawParams<'a>>) {
		self.sprite_batch.draw(ctx, &self.texture, params);
	}
}
