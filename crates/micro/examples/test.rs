use std::{error::Error, time::Duration};

use glam::vec2;
use micro::{
	color::ColorConstants,
	graphics::{
		mesh::Mesh,
		text::{Font, FontSettings, LayoutSettings},
		texture::{Texture, TextureSettings},
	},
	ui::{
		Align, AxisSizing, Ellipse, Mask, MatchSize, Padding, Polygon, Polyline, Rectangle, Sizing,
		Stack, StackSettings, Text, TextSettings, TextSizeReporting, TextSizing, Transform, Widget,
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
		Stack::vertical(StackSettings {
			gap: 0.0,
			cross_align: 0.0,
			cross_sizing: AxisSizing::Shrink,
		})
		.with_child(Text::new(
			&self.font,
			"How are you?",
			TextSettings {
				sizing: TextSizing::Min {
					size_reporting: TextSizeReporting {
						include_lowest_line_descenders: false,
					},
				},
				..Default::default()
			},
		))
		.with_child(
			Rectangle::new()
				.with_vertical_sizing(AxisSizing::Max(2.0))
				.with_fill(LinSrgb::RED),
		)
		.render(ctx, ctx.window_size().as_vec2())?;
		Ok(())
	}
}
