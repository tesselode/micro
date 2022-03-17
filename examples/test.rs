use std::error::Error;

use micro::{
	color::Rgba,
	context::Context,
	font::{Font, FontSettings},
	Game, State,
};

struct MainState {
	font: Font,
}

impl MainState {
	fn new(ctx: &mut Context) -> Result<Self, Box<dyn Error>> {
		let font = Font::from_file(ctx, "examples/Roboto-Regular.ttf", FontSettings::default())?;
		Ok(Self { font })
	}
}

impl State<Box<dyn Error>> for MainState {
	fn draw(&mut self, ctx: &mut Context) -> Result<(), Box<dyn Error>> {
		ctx.clear(Rgba::BLACK);
		Ok(())
	}
}

fn main() -> Result<(), Box<dyn Error>> {
	Game::init()?.run(MainState::new)?;
	Ok(())
}
