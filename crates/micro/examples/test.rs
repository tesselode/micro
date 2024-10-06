use std::{error::Error, time::Duration};

use glam::{vec2, Vec2};
use micro::{
	color::ColorConstants,
	graphics::mesh::{MeshBuilder, ShapeStyle},
	math::{Circle, Rect},
	App, Context, ContextSettings,
};
use palette::{LinSrgb, LinSrgba};

const CIRCLE: Circle = Circle {
	center: vec2(400.0, 300.0),
	radius: 200.0,
};
const RECT_SIZE: Vec2 = vec2(100.0, 50.0);

fn main() {
	micro::run(ContextSettings::default(), Game::new);
}

struct Game;

impl Game {
	fn new(_ctx: &mut Context) -> Result<Self, Box<dyn Error>> {
		Ok(Self)
	}

	fn rect(&self, ctx: &Context) -> Rect {
		Rect::centered_around(ctx.mouse_position().as_vec2(), RECT_SIZE)
	}
}

impl App<Box<dyn Error>> for Game {
	fn update(&mut self, ctx: &mut Context, _delta_time: Duration) -> Result<(), Box<dyn Error>> {
		Ok(())
	}

	fn draw(&mut self, ctx: &mut Context) -> Result<(), Box<dyn Error>> {
		ctx.clear(LinSrgb::BLACK);
		let rect = self.rect(ctx);
		MeshBuilder::new()
			.with_circle(ShapeStyle::Stroke(2.0), CIRCLE, LinSrgb::WHITE.into())?
			.with_rectangle(
				ShapeStyle::Stroke(2.0),
				rect,
				if rect.overlaps_circle(CIRCLE) {
					LinSrgba::RED
				} else {
					LinSrgba::WHITE
				},
			)?
			.build(ctx)
			.draw(ctx);
		Ok(())
	}
}
