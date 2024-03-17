use std::error::Error;

use micro::{
	clear,
	graphics::{
		text::{Font, FontSettings, LayoutSettings, Text},
		texture::{Texture, TextureSettings},
		Canvas, CanvasSettings, ColorConstants,
	},
	render_to_canvas, window_size, ContextSettings, State,
};
use palette::LinSrgba;

fn main() {
	micro::run(ContextSettings::default(), MainState::new);
}

struct MainState {
	font: Font,
	text: Text,
	canvas: Canvas,
	texture: Texture,
}

impl MainState {
	pub fn new() -> Result<Self, Box<dyn Error>> {
		let font = Font::from_file("resources/Abaddon Bold.ttf", FontSettings::default())?;
		let text = Text::new(&font, "Hello, world!", LayoutSettings::default());
		Ok(Self {
			font,
			text,
			canvas: Canvas::new(window_size(), CanvasSettings::default()),
			texture: Texture::from_file(
				"resources/spritesheet_default.png",
				TextureSettings::default(),
			)?,
		})
	}
}

impl State<Box<dyn Error>> for MainState {
	fn draw(&mut self) -> Result<(), Box<dyn Error>> {
		clear(LinSrgba::BLACK);
		render_to_canvas!(self.canvas, {
			clear(LinSrgba::BLACK);
			self.text
				.color(LinSrgba::RED)
				.translated_x(100.0)
				.range(2..=4)
				.draw();
		});
		self.canvas.draw();
		self.texture.draw();
		Ok(())
	}
}
