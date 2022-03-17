use std::rc::Rc;

use fontdue::layout::{CoordinateSystem, Layout, TextStyle};
use glam::Vec2;

use crate::{
	context::Context,
	draw_params::DrawParams,
	error::GlError,
	rect::Rect,
	sprite_batch::{Sprite, SpriteBatch},
	texture::Texture,
};

use super::Font;

pub struct Text {
	pub(crate) texture: Rc<Texture>,
	pub(crate) sprite_batch: SpriteBatch,
}

impl Text {
	pub fn new(ctx: &mut Context, font: &Font, text: &str) -> Result<Self, GlError> {
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
		let mut sprite_batch = SpriteBatch::new(ctx, glyphs.len())?;
		for glyph in glyphs {
			sprite_batch
				.add(Sprite {
					display_rect: Rect {
						top_left: Vec2::new(glyph.x, glyph.y),
						size: Vec2::new(glyph.width as f32, glyph.height as f32),
					},
					texture_rect: *font
						.glyph_rects
						.get(&glyph.parent)
						.expect("No glyph rect for this character"),
				})
				.expect("Not enough capacity in the sprite batch");
		}
		Ok(Self {
			texture: font.texture.clone(),
			sprite_batch,
		})
	}

	pub fn draw<'a>(&self, ctx: &mut Context, params: impl Into<DrawParams<'a>>) {
		self.sprite_batch.draw(ctx, &self.texture, params);
	}
}
