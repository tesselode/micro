use std::error::Error;

use micro_wgpu::{
	App, Context, ContextSettings, Event,
	color::ColorConstants,
	graphics::{
		mesh::{Mesh, builder::ShapeStyle},
		sprite_batch::{SpriteBatch, SpriteParams},
		texture::{Texture, TextureSettings},
	},
	input::Scancode,
	math::{Circle, Rect},
};
use palette::{LinSrgb, LinSrgba};

fn main() -> Result<(), Box<dyn Error>> {
	micro_wgpu::run(
		ContextSettings {
			resizable: true,
			..Default::default()
		},
		Test::new,
	)
}

struct Test {
	texture: Texture,
	sprite_batch: SpriteBatch,
}

impl Test {
	fn new(ctx: &mut Context) -> Result<Self, Box<dyn Error>> {
		let texture = Texture::from_file(
			ctx,
			"resources/spritesheet_default.png",
			TextureSettings::default(),
		)?;
		let mut sprite_batch = SpriteBatch::new(ctx, &texture, 100);
		sprite_batch.add(
			ctx,
			SpriteParams::new()
				.color(LinSrgba::new(1.0, 1.0, 1.0, 0.5))
				.translated((100.0, 100.0)),
		)?;
		sprite_batch.add(
			ctx,
			SpriteParams::new()
				.color(LinSrgba::new(1.0, 1.0, 1.0, 0.5))
				.translated((200.0, 200.0)),
		)?;
		Ok(Self {
			texture,
			sprite_batch,
		})
	}
}

impl App for Test {
	type Error = Box<dyn Error>;

	fn event(&mut self, ctx: &mut Context, event: Event) -> Result<(), Self::Error> {
		if let Event::KeyPressed {
			key: Scancode::Return,
			..
		} = event
		{
			ctx.set_clear_color(LinSrgb::BLUE);
		}
		Ok(())
	}

	fn draw(&mut self, ctx: &mut Context) -> Result<(), Self::Error> {
		self.sprite_batch
			.translated_2d(ctx.mouse_position().as_vec2())
			.draw(ctx);
		Ok(())
	}
}
