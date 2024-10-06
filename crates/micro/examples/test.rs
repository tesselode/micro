use std::{error::Error, time::Duration};

use glam::{vec2, Vec2};
use micro::{
	color::ColorConstants,
	graphics::mesh::{MeshBuilder, ShapeStyle},
	input::MouseButton,
	math::{Circle, LineSegment},
	App, Context, ContextSettings,
};
use palette::{LinSrgb, LinSrgba};

const CIRCLE_RADIUS: f32 = 50.0;

fn main() {
	micro::run(ContextSettings::default(), Game::new);
}

struct Game {
	line_segment_start: Vec2,
	circle_center: Vec2,
}

impl Game {
	fn new(_ctx: &mut Context) -> Result<Self, Box<dyn Error>> {
		Ok(Self {
			line_segment_start: Vec2::splat(50.0),
			circle_center: vec2(400.0, 300.0),
		})
	}

	fn circle(&self) -> Circle {
		Circle {
			center: self.circle_center,
			radius: CIRCLE_RADIUS,
		}
	}

	fn line_segment(&self, ctx: &Context) -> LineSegment {
		LineSegment {
			start: self.line_segment_start,
			end: ctx.mouse_position().as_vec2(),
		}
	}
}

impl App<Box<dyn Error>> for Game {
	fn update(&mut self, ctx: &mut Context, _delta_time: Duration) -> Result<(), Box<dyn Error>> {
		if ctx.is_mouse_button_down(MouseButton::Left) {
			self.line_segment_start = ctx.mouse_position().as_vec2();
		}
		if ctx.is_mouse_button_down(MouseButton::Right) {
			self.circle_center = ctx.mouse_position().as_vec2();
		}
		Ok(())
	}

	fn draw(&mut self, ctx: &mut Context) -> Result<(), Box<dyn Error>> {
		ctx.clear(LinSrgb::BLACK);
		let circle = self.circle();
		let line_segment = self.line_segment(ctx);
		MeshBuilder::new()
			.with_circle(ShapeStyle::Stroke(2.0), circle, LinSrgb::WHITE.into())?
			.with_simple_polyline(
				2.0,
				<[Vec2; 2]>::from(line_segment),
				if line_segment.intersects_circle(circle) {
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
