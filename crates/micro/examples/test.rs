use std::{error::Error, time::Duration};

use glam::vec2;
use micro::{
	color::ColorConstants,
	graphics::{
		mesh::Mesh,
		text::{Font, FontSettings, LayoutSettings, Text},
		texture::{Texture, TextureSettings},
	},
	ui::{
		Align, AxisSizing, Ellipse, Mask, MatchSize, Padding, Polygon, Polyline, Rectangle, Sizing,
		Stack, StackSettings, TextSettings, Transform, Widget,
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
		let text = Text::new(
			ctx,
			&self.font,
			"Hello, world!\nNice to meet you!",
			LayoutSettings::default(),
		);
		text.draw(ctx);
		let bounds = text.bounds().unwrap();
		let lowest_baseline = text.lowest_baseline().unwrap();
		Mesh::simple_polyline(
			ctx,
			1.0,
			[
				(bounds.left(), lowest_baseline),
				(bounds.right(), lowest_baseline),
			],
		)?
		.color(LinSrgb::RED)
		.draw(ctx);
		Ok(())
	}
}
