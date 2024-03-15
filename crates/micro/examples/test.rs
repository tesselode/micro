use std::error::Error;

use micro::{
	clear,
	graphics::{
		text::{Font, FontSettings, LayoutSettings, Text},
		ColorConstants,
	},
	ContextSettings, State,
};
use palette::LinSrgba;

fn main() {
	micro::run(ContextSettings::default(), MainState::new);
}

struct MainState {
	font: Font,
	text: Text,
}

impl MainState {
	pub fn new() -> Result<Self, Box<dyn Error>> {
		let font = Font::from_file("resources/Abaddon Bold.ttf", FontSettings::default())?;
		let text = Text::new(&font, "Hello, world!", LayoutSettings::default());
		Ok(Self { font, text })
	}
}

impl State<Box<dyn Error>> for MainState {
	fn draw(&mut self) -> Result<(), Box<dyn Error>> {
		clear(LinSrgba::BLACK);
		self.text.color(LinSrgba::RED).range(2..=4).draw();
		Ok(())
	}
}
