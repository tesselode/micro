use std::{error::Error, time::Duration};

use glam::{vec2, FloatExt};
use micro::{
	color::ColorConstants,
	graphics::mesh::{Mesh, ShapeStyle},
	math::Circle,
	tween::{Easing, TweenSequence},
	App, Context, ContextSettings,
};
use palette::LinSrgb;

fn main() {
	micro::run(ContextSettings::default(), MainState::new);
}

struct MainState {
	tween_sequence: TweenSequence<f32>,
}

impl MainState {
	fn new(_ctx: &mut Context) -> Result<Self, Box<dyn Error>> {
		Ok(Self {
			tween_sequence: TweenSequence::new(0.0)
				.wait(Duration::from_millis(500))
				.tween(Duration::from_millis(500), 1.0, Easing::InOutPowi(2))
				.wait(Duration::from_millis(500))
				.looping(),
		})
	}
}

impl App<Box<dyn Error>> for MainState {
	fn update(&mut self, _ctx: &mut Context, delta_time: Duration) -> Result<(), Box<dyn Error>> {
		self.tween_sequence.update(delta_time);
		Ok(())
	}

	fn draw(&mut self, ctx: &mut Context) -> Result<(), Box<dyn Error>> {
		ctx.clear(LinSrgb::BLACK);
		Mesh::circle(
			ctx,
			ShapeStyle::Fill,
			Circle {
				center: vec2(100.0.lerp(700.0, self.tween_sequence.current()), 300.0),
				radius: 50.0,
			},
		)?
		.draw(ctx);
		Ok(())
	}
}
