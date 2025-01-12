use std::{error::Error, time::Duration};

use glam::{vec2, Vec2};
use micro::{
	color::ColorConstants,
	graphics::mesh::{MeshBuilder, ShapeStyle},
	math::{Circle, Polygon},
	App, Context, ContextSettings,
};
use palette::{LinSrgb, LinSrgba};

const POLYGON_POINTS: &[Vec2] = &[
	vec2(300.0, 300.0),
	vec2(500.0, 300.0),
	vec2(400.0, 500.0),
	vec2(100.0, 450.0),
];
const CIRCLE_RADIUS: f32 = 50.0;

fn main() {
	micro::run(ContextSettings::default(), Game::new);
}

struct Game {
	polygon: Polygon,
}

impl Game {
	fn new(_ctx: &mut Context) -> Result<Self, Box<dyn Error>> {
		Ok(Self {
			polygon: Polygon::new(POLYGON_POINTS),
		})
	}
}

impl App for Game {
	type Error = Box<dyn Error>;

	fn update(&mut self, ctx: &mut Context, _delta_time: Duration) -> Result<(), Box<dyn Error>> {
		Ok(())
	}

	fn draw(&mut self, ctx: &mut Context) -> Result<(), Box<dyn Error>> {
		ctx.clear(LinSrgb::BLACK);
		let circle = circle(ctx);
		MeshBuilder::new()
			.with_circle(
				ShapeStyle::Stroke(2.0),
				circle,
				if self.polygon.overlaps_circle(circle) {
					LinSrgba::RED
				} else {
					LinSrgba::WHITE
				},
			)?
			.with_simple_polygon(
				ShapeStyle::Stroke(2.0),
				self.polygon.points.iter().copied(),
				LinSrgba::WHITE,
			)?
			.build(ctx)
			.draw(ctx);
		Ok(())
	}
}

fn circle(ctx: &Context) -> Circle {
	Circle {
		center: ctx.mouse_position().as_vec2(),
		radius: CIRCLE_RADIUS,
	}
}
