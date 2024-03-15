use std::error::Error;

use micro::{
	clear,
	graphics::{
		mesh::{Mesh, ShapeStyle},
		text::{Font, FontSettings},
		texture::{Texture, TextureSettings},
		Canvas, CanvasSettings, ColorConstants,
	},
	math::Circle,
	push_translation_2d, push_translation_y, window_size, ContextSettings, State,
};
use palette::LinSrgba;

fn main() {
	micro::run(ContextSettings::default(), MainState::new);
}

struct MainState {
	texture: Texture,
	font: Font,
	canvas: Canvas,
}

impl MainState {
	pub fn new() -> Result<Self, Box<dyn Error>> {
		Ok(Self {
			texture: Texture::from_file(
				"resources/spritesheet_default.png",
				TextureSettings::default(),
			)?,
			font: Font::from_file("resources/Abaddon Bold.ttf", FontSettings::default())?,
			canvas: Canvas::new(window_size(), CanvasSettings::default()),
		})
	}
}

impl State<Box<dyn Error>> for MainState {
	fn draw(&mut self) -> Result<(), Box<dyn Error>> {
		clear(LinSrgba::BLACK);

		push_translation_2d!((100.0, 50.0), {
			Mesh::circle(ShapeStyle::Fill, Circle::centered_around_zero(40.0))?.draw();
		});

		Ok(())
	}
}
