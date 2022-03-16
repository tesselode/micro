use std::error::Error;

use glam::Vec2;
use micro::{
	color::Rgba,
	context::Context,
	draw_params::DrawParams,
	rect::Rect,
	sprite_batch::{Sprite, SpriteBatch, SpriteId},
	texture::Texture,
	Game, State,
};

struct MainState {
	texture: Texture,
	sprite_batch: SpriteBatch,
	sprite_id: SpriteId,
	remove_timer: usize,
}

impl MainState {
	fn new(ctx: &mut Context) -> Result<Self, Box<dyn Error>> {
		let texture = Texture::load(ctx, "examples/player.png")?;
		let mut sprite_batch = SpriteBatch::new(ctx, 10)?;
		sprite_batch.add(Sprite {
			display_rect: Rect::new(Vec2::new(100.0, 100.0), Vec2::new(100.0, 100.0)),
			texture_rect: texture
				.relative_rect(Rect::new(Vec2::new(0.0, 0.0), Vec2::new(72.0, 97.0))),
		})?;
		let sprite_id = sprite_batch.add(Sprite {
			display_rect: Rect::new(Vec2::new(500.0, 100.0), Vec2::new(100.0, 100.0)),
			texture_rect: texture
				.relative_rect(Rect::new(Vec2::new(73.0, 0.0), Vec2::new(72.0, 97.0))),
		})?;
		Ok(Self {
			texture,
			sprite_batch,
			sprite_id,
			remove_timer: 100,
		})
	}
}

impl State<Box<dyn Error>> for MainState {
	fn draw(&mut self, ctx: &mut Context) -> Result<(), Box<dyn Error>> {
		ctx.clear(Rgba::BLACK);
		if self.remove_timer > 0 {
			self.remove_timer -= 1;
			if self.remove_timer == 0 {
				self.sprite_batch.remove(self.sprite_id)?;
			}
		}
		self.sprite_batch
			.draw(ctx, &self.texture, DrawParams::new());
		Ok(())
	}
}

fn main() -> Result<(), Box<dyn Error>> {
	Game::init()?.run(MainState::new)?;
	Ok(())
}
