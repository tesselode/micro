use std::error::Error;

use micro::{
	blend_mode::{BlendAlphaMode, BlendMode},
	color::Rgba,
	context::Context,
	draw_params::DrawParams,
	font::{text::Text, Font, FontSettings},
	Game, State,
};

struct MainState {
	font: Font,
	text: Text,
}

impl MainState {
	fn new(ctx: &mut Context) -> Result<Self, Box<dyn Error>> {
		let font = Font::from_file(
			ctx,
			"examples/Roboto-Regular.ttf",
			FontSettings {
				scale: 64.0,
				..Default::default()
			},
		)?;
		let text = Text::new(ctx, &font, "hello world!")?;
		Ok(Self { font, text })
	}
}

impl State<Box<dyn Error>> for MainState {
	fn draw(&mut self, ctx: &mut Context) -> Result<(), Box<dyn Error>> {
		ctx.clear(Rgba::BLACK);
		self.text.draw(ctx, DrawParams::new());
		Ok(())
	}
}

fn main() -> Result<(), Box<dyn Error>> {
	Game::init()?.run(MainState::new)?;
	Ok(())
}
