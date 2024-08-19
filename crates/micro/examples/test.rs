use std::{error::Error, time::Duration};

use glam::{vec2, FloatExt};
use micro::{
	color::ColorConstants,
	graphics::mesh::{Mesh, ShapeStyle},
	math::Circle,
	tween::{Easing, TweenSequence},
	ui::{Padding, Rectangle, Widget},
	App, Context, ContextSettings,
};
use palette::LinSrgb;

fn main() {
	micro::run(ContextSettings::default(), MainState::new);
}

struct MainState {}

impl MainState {
	fn new(_ctx: &mut Context) -> Result<Self, Box<dyn Error>> {
		Ok(Self {})
	}
}

impl App<Box<dyn Error>> for MainState {
	fn update(&mut self, _ctx: &mut Context, delta_time: Duration) -> Result<(), Box<dyn Error>> {
		Ok(())
	}

	fn draw(&mut self, ctx: &mut Context) -> Result<(), Box<dyn Error>> {
		let mut widget = Padding::symmetric(vec2(50.0, 100.0)).with_child(
			Rectangle::new()
				.with_fill(LinSrgb::RED)
				.with_stroke(10.0, LinSrgb::BLUE),
		);
		widget.size(ctx.window_size().as_vec2());
		widget.draw(ctx)?;
		Ok(())
	}
}
