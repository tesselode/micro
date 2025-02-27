use std::error::Error;

use micro_wgpu::{
	App, Context, ContextSettings, Event,
	color::ColorConstants,
	graphics::{
		mesh::{Mesh, builder::ShapeStyle},
		sprite_batch::{SpriteBatch, SpriteParams},
		text::{Font, FontSettings, LayoutSettings, Text},
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
	text: Text,
}

impl Test {
	fn new(ctx: &mut Context) -> Result<Self, Box<dyn Error>> {
		let font = Font::from_file(
			ctx,
			"resources/NotoSans-Regular.ttf",
			FontSettings::default(),
		)?;
		let text = Text::new(ctx, &font, "Hello, world!", LayoutSettings::default());
		Ok(Self { text })
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
		self.text
			.color(LinSrgb::RED)
			.translated_2d(ctx.mouse_position().as_vec2())
			.draw(ctx);
		Ok(())
	}
}
