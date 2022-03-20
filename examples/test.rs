use std::error::Error;

use micro::{
	graphics::{
		color::Rgba,
		text::{Font, FontSettings, Text},
		DrawParams,
	},
	Context, ContextSettings, State,
};

struct MainState {
	text: Text,
}

impl MainState {
	fn new(ctx: &mut Context) -> Result<Self, Box<dyn Error>> {
		let font = Font::from_file(
			ctx,
			"examples/Roboto-Regular.ttf",
			FontSettings {
				scale: 128.0,
				..Default::default()
			},
		)?;
		let text = Text::new(ctx, &font, "Hello world!")?;
		Ok(Self { text })
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
	Context::new(ContextSettings {
		window_width: 1280,
		window_height: 720,
		vsync: false,
		..Default::default()
	})?
	.run(MainState::new)?;
	Ok(())
}
