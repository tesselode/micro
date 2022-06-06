use std::error::Error;

use micro::{
	graphics::{
		color::Rgba,
		mesh::{Mesh, ShapeStyle},
		text::{Font, FontSettings, Text},
		DrawParams,
	},
	Context, ContextSettings, State,
};
use vek::Vec2;

struct MainState {
	font: Font,
}

impl MainState {
	fn new(ctx: &mut Context) -> Result<Self, Box<dyn Error>> {
		Ok(Self {
			font: Font::from_file(ctx, "examples/Roboto-Regular.ttf", FontSettings::default())?,
		})
	}
}

impl State<Box<dyn Error>> for MainState {
	fn draw(&mut self, ctx: &mut Context) -> Result<(), Box<dyn Error>> {
		ctx.clear(Rgba::BLACK);
		Text::new(ctx, &self.font, "This is some\nmultiline text.").draw(ctx, DrawParams::new());
		Ok(())
	}
}

fn main() -> Result<(), Box<dyn Error>> {
	micro::run(ContextSettings::default(), MainState::new)
}
