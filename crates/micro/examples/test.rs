use std::{error::Error, path::Path, time::Duration};

use glam::{vec2, Vec2};
use micro::{
	color::ColorConstants,
	graphics::{
		mesh::Mesh,
		text::{Font, FontSettings, LayoutSettings},
		texture::{Texture, TextureSettings},
	},
	input::Scancode,
	ui::{
		Align, AxisSizing, Ellipse, Mask, MatchSize, Padding, Polygon, Polyline, Rectangle, Sizing,
		Stack, StackSettings, Text, TextSettings, TextSizeReporting, TextSizing, Transform, Ui,
		UiState, Widget,
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
	ui: Ui,
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
			ui: Ui::new(),
		})
	}
}

impl App<Box<dyn Error>> for MainState {
	fn update(&mut self, _ctx: &mut Context, delta_time: Duration) -> Result<(), Box<dyn Error>> {
		Ok(())
	}

	fn draw(&mut self, ctx: &mut Context) -> Result<(), Box<dyn Error>> {
		if ctx.is_key_down(Scancode::Space) {
			self.ui
				.render(ctx, ctx.window_size().as_vec2(), TestWidget)?;
		} else {
			self.ui
				.render(ctx, ctx.window_size().as_vec2(), Rectangle::new())?;
		}
		Ok(())
	}
}

#[derive(Debug)]
struct TestWidget;

impl Widget for TestWidget {
	fn name(&self) -> &'static str {
		"testWidget"
	}

	fn size(
		&mut self,
		ctx: &mut Context,
		state: &mut UiState,
		path: &Path,
		allotted_size: Vec2,
	) -> Vec2 {
		Vec2::ZERO
	}

	fn draw(&self, ctx: &mut Context, state: &mut UiState, path: &Path) -> anyhow::Result<()> {
		let TestWidgetState { num_frames } = state.get_mut(path);
		*num_frames += 1;
		println!("{}", *num_frames);
		Ok(())
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
struct TestWidgetState {
	num_frames: usize,
}
