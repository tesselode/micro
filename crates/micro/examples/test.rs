use std::error::Error;

use glam::{UVec2, Vec2};
use micro::{
	graphics::{
		text::{LayoutSettings, Text},
		ColorConstants, DrawParams,
	},
	resource::{loader::FontLoader, Resources},
	Context, ContextSettings, State, WindowMode,
};
use palette::LinSrgba;

pub struct MainState {
	fonts: Resources<FontLoader>,
	text: Text,
}

impl MainState {
	fn new(ctx: &mut Context) -> Result<Self, Box<dyn Error>> {
		let mut fonts = Resources::new(FontLoader::default());
		fonts.load(ctx, "")?;
		let text = Text::new(
			ctx,
			&fonts["Roboto-Regular"],
			"Test text please ignore",
			LayoutSettings::default(),
		);
		Ok(Self { fonts, text })
	}
}

impl State<Box<dyn Error>> for MainState {
	fn draw(&mut self, ctx: &mut Context) -> Result<(), Box<dyn Error>> {
		ctx.clear(LinSrgba::BLACK);
		self.text.draw(ctx, DrawParams::new());
		Ok(())
	}
}

fn main() {
	micro::run(
		ContextSettings {
			window_mode: WindowMode::Windowed {
				size: UVec2::new(500, 500),
			},
			resizable: true,
			..Default::default()
		},
		MainState::new,
	);
}
