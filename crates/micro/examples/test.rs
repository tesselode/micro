use std::{error::Error, time::Duration};

use fontdue::layout::{HorizontalAlign, VerticalAlign};
use glam::vec2;
use micro::{
	color::ColorConstants,
	graphics::{
		text::{Font, FontSettings},
		texture::{Texture, TextureSettings},
	},
	ui::{
		Align, CrossSizing, FractionalMaxSize, Image, MaxSize, Padding, Rectangle, Stack,
		StackSettings, Text, TextSettings, TextShadow, TextSizing, Transform, Widget,
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
		Padding::symmetric(vec2(50.0, 100.0))
			.with_child(Rectangle::new().with_stroke(5.0, LinSrgb::WHITE))
			.with_child(
				FractionalMaxSize::new((0.25, 0.5))
					.with_child(Rectangle::new().with_stroke(5.0, LinSrgb::WHITE)),
			)
			.render(ctx, ctx.window_size().as_vec2())?;
		Ok(())
	}
}
