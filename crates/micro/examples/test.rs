use std::error::Error;

use glam::{UVec2, Vec2};
use micro::{
	graphics::text::{Font, FontSettings, LayoutSettings, Text},
	window::WindowMode,
	Context, ContextSettings, State,
};

struct MainState {
	font: Font,
	text: Text,
}

impl MainState {
	fn new(ctx: &mut Context) -> Result<Self, Box<dyn Error>> {
		let font = Font::from_file(
			ctx,
			"crates/micro/examples/Roboto-Regular.ttf",
			FontSettings {
				scale: 50.0,
				..Default::default()
			},
		)?;
		let text = Text::new(ctx, &font, "Hello, world!", LayoutSettings::default());
		Ok(Self { font, text })
	}
}

impl State<Box<dyn Error>> for MainState {
	fn draw(&mut self, ctx: &mut Context) -> Result<(), Box<dyn Error>> {
		self.text.draw(ctx, Vec2::new(50.0, 50.0));
		Ok(())
	}
}

fn main() -> Result<(), Box<dyn Error>> {
	micro::run(
		ContextSettings {
			window_mode: WindowMode::Windowed {
				size: UVec2::new(1280, 720),
			},
			..Default::default()
		},
		MainState::new,
	)
}
