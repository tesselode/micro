use std::{error::Error, time::Duration};

use glam::vec2;
use micro::{
	color::ColorConstants,
	graphics::{
		text::{Font, FontSettings},
		texture::{Texture, TextureSettings},
	},
	ui::{
		CrossSizing, Ellipse, FractionalMaxSize, Mask, MaxSize, Padding, Polygon, Polyline,
		Rectangle, Stack, StackSettings, Transform, Widget,
	},
	App, Context, ContextSettings,
};
use palette::{LinSrgb, LinSrgba};

fn main() {
	micro::run(ContextSettings::default(), MainState::new);
}

struct MainState {
	font: Font,
	texture: Texture,
}

impl MainState {
	fn new(ctx: &mut Context) -> Result<Self, Box<dyn Error>> {
		Ok(Self {
			font: Font::from_file(
				ctx,
				"resources/NotoSans-Regular.ttf",
				FontSettings::default(),
			)?,
			texture: Texture::from_file(
				ctx,
				"resources/spritesheet_default.png",
				TextureSettings::default(),
			)?,
		})
	}
}

impl App<Box<dyn Error>> for MainState {
	fn update(&mut self, _ctx: &mut Context, delta_time: Duration) -> Result<(), Box<dyn Error>> {
		Ok(())
	}

	fn draw(&mut self, ctx: &mut Context) -> Result<(), Box<dyn Error>> {
		MaxSize::new((100.0, 100.0))
			.with_child(
				Mask::new()
					.with_mask_child(Rectangle::new().with_fill(LinSrgb::WHITE))
					.with_child(
						Transform::translation((50.0, 50.0))
							.with_child(Ellipse::new().with_fill(LinSrgb::WHITE)),
					),
			)
			.render(ctx, ctx.window_size().as_vec2())?;
		Ok(())
	}
}
