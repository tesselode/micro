use std::{error::Error, time::Duration};

use glam::vec2;
use micro::{
	color::ColorConstants,
	graphics::{
		text::{Font, FontSettings},
		texture::{Texture, TextureSettings},
	},
	ui::{
		Align, AxisSizing, Ellipse, Mask, MatchSize, Padding, Polygon, Polyline, Rectangle, Sizing,
		Stack, StackSettings, Text, TextSettings, Transform, Widget,
	},
	App, Context, ContextSettings,
};
use palette::{Darken, LinSrgb, LinSrgba};

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
		MatchSize::new()
			.with_child(Rectangle::new().with_fill(LinSrgb::WHITE.darken(0.9)))
			.with_sizing_child(
				Align::center()
					.with_horizontal_sizing(AxisSizing::Shrink)
					.with_vertical_sizing(AxisSizing::Max(100.0))
					.with_child(Padding::horizontal(10.0).with_child(Text::new(
						&self.font,
						"Hello, world!",
						TextSettings::default(),
					))),
			)
			.render(ctx, ctx.window_size().as_vec2())?;
		Ok(())
	}
}
