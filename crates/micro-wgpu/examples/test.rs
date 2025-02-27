use std::error::Error;

use micro_wgpu::{
	App, Context, ContextSettings, Event,
	color::ColorConstants,
	graphics::{
		mesh::{Mesh, builder::ShapeStyle},
		texture::{Texture, TextureSettings},
	},
	input::Scancode,
	math::{Circle, Rect},
};
use palette::LinSrgb;

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
}

impl Test {
	fn new(ctx: &mut Context) -> Result<Self, Box<dyn Error>> {
		Ok(Self {
			texture: Texture::from_file(
				ctx,
				"resources/spritesheet_default.png",
				TextureSettings::default(),
			)?,
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
		self.texture
			.region(Rect::new((10.0, 10.0), (50.0, 50.0)))
			.color(LinSrgb::RED)
			.translated_2d(ctx.mouse_position().as_vec2())
			.draw(ctx);
		Ok(())
	}
}
