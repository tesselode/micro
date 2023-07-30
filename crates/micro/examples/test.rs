use std::error::Error;

use micro::{
	graphics::{
		mesh::{Mesh, ShapeStyle},
		text::{Font, FontSettings, LayoutSettings, Text},
		DrawParams,
	},
	Context, ContextSettings, State,
};
use palette::LinSrgba;

struct MainState {
	text: Text,
	mesh: Mesh,
}

impl MainState {
	fn new(ctx: &mut Context) -> Result<Self, Box<dyn Error>> {
		let font = Font::from_file(
			ctx,
			"crates/micro/examples/Roboto-asdfRegular.ttf",
			FontSettings::default(),
		)?;
		let text = Text::new(
			ctx,
			&font,
			"The quick brown fox jumps over the lazy dog.",
			LayoutSettings::default(),
		)?;
		let rect = text.character_bounds(5..10).unwrap();
		let mesh = Mesh::styled_rectangle(
			ctx,
			ShapeStyle::Stroke(2.0),
			rect,
			LinSrgba::new(1.0, 0.0, 0.0, 1.0),
		)?;
		Ok(Self { text, mesh })
	}
}

impl State<Box<dyn Error>> for MainState {
	fn draw(&mut self, ctx: &mut Context) -> Result<(), Box<dyn Error>> {
		self.text.draw(ctx, DrawParams::new());
		self.mesh.draw(ctx, DrawParams::new());
		Ok(())
	}
}

fn main() {
	micro::run(ContextSettings::default(), MainState::new)
}
