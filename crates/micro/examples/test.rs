use std::{error::Error, time::Duration};

use glam::vec2;
use micro::{
	color::ColorConstants,
	graphics::mesh::{Mesh, ShapeStyle},
	math::Circle,
	tween::{Easing, TweenSequence},
	App, Context, ContextSettings,
};
use palette::LinSrgb;

fn main() {
	micro::run(ContextSettings::default(), Game::new);
}

struct Game {
	tween_sequence: TweenSequence<f32, Duration, Event>,
}

impl Game {
	fn new(_ctx: &mut Context) -> Result<Self, Box<dyn Error>> {
		Ok(Self {
			tween_sequence: TweenSequence::new(100.0)
				.tween(Duration::from_secs(1), 700.0, Easing::Linear)
				.emit(Event::Ping)
				.tween(Duration::from_secs(1), 100.0, Easing::Linear)
				.emit(Event::Pong)
				.looping(),
		})
	}
}

impl App<Box<dyn Error>> for Game {
	fn update(&mut self, ctx: &mut Context, delta_time: Duration) -> Result<(), Box<dyn Error>> {
		self.tween_sequence.update(delta_time);
		while let Some(event) = self.tween_sequence.pop_event() {
			dbg!(event);
		}
		Ok(())
	}

	fn draw(&mut self, ctx: &mut Context) -> Result<(), Box<dyn Error>> {
		ctx.clear(LinSrgb::BLACK);
		Mesh::circle(
			ctx,
			ShapeStyle::Fill,
			Circle {
				center: vec2(self.tween_sequence.current(), 300.0),
				radius: 20.0,
			},
		)?
		.draw(ctx);
		Ok(())
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Event {
	Ping,
	Pong,
}
