use std::{error::Error, time::Duration};

use glam::vec2;
use micro::{
	color::ColorConstants,
	graphics::{
		mesh::Mesh,
		text::{Font, FontSettings, LayoutSettings},
		texture::{Texture, TextureSettings},
	},
	ui::{Align, AxisSizing, Rectangle, Stack, StackSettings, Ui},
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
		Ui.render(
			ctx,
			ctx.window_size().as_vec2(),
			Stack::horizontal(StackSettings {
				gap: 10.0,
				cross_align: 0.5,
				cross_sizing: AxisSizing::Expand,
			})
			.with_children(
				[(50.0, 50.0), (100.0, 50.0), (50.0, 100.0)]
					.iter()
					.map(|size| {
						Rectangle::new()
							.with_max_size(*size)
							.with_stroke(2.0, LinSrgb::WHITE)
					}),
			),
		)?;
		Ok(())
	}
}
