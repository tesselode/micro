use std::error::Error;

use glam::UVec2;
use micro::{
	graphics::{
		color_constants::ColorConstants,
		texture::{Texture, TextureSettings},
		DrawParams,
	},
	window::WindowMode,
	Context, ContextSettings, State,
};
use palette::LinSrgba;

pub struct MainState {
	texture: Texture,
}

impl MainState {
	fn new(ctx: &mut Context) -> Result<Self, Box<dyn Error>> {
		Ok(Self {
			texture: Texture::from_file(ctx, "examples/wall.png", TextureSettings::default())?,
		})
	}
}

impl State<Box<dyn Error>> for MainState {
	fn draw(&mut self, ctx: &mut Context) -> Result<(), Box<dyn Error>> {
		ctx.clear(LinSrgba::BLACK);
		self.texture.draw(ctx, DrawParams::new());
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
