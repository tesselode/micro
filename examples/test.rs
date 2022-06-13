use std::time::Duration;

use micro::{
	graphics::{
		color::Rgba,
		mesh::{Mesh, ShapeStyle},
	},
	tween::{tween_sequence::TweenSequence, Easing},
	Context, ContextSettings, State,
};
use vek::Vec2;

struct MainState {
	mesh: Mesh,
	tween_sequence: TweenSequence<f32>,
}

impl MainState {
	fn new(ctx: &mut Context) -> Self {
		Self {
			mesh: Mesh::circle(ctx, ShapeStyle::Fill, Vec2::zero(), 64.0),
			tween_sequence: TweenSequence::new(0.0)
				.wait(Duration::from_secs_f32(1.0))
				.tween(Duration::from_secs_f32(1.0), 1.0, Easing::InOutSine)
				.tween(Duration::from_secs_f32(0.5), 0.0, Easing::InPowi(2)),
		}
	}
}

impl State for MainState {
	fn update(&mut self, _ctx: &mut Context, delta_time: Duration) {
		self.tween_sequence.update(delta_time);
	}

	fn draw(&mut self, ctx: &mut Context) {
		ctx.clear(Rgba::BLACK);
		self.mesh
			.draw(ctx, Vec2::new(800.0 * self.tween_sequence.current(), 300.0));
	}
}

fn main() {
	micro::run(ContextSettings::default(), MainState::new)
}
