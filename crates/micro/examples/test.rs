use std::{error::Error, time::Duration};

use glam::{Vec2, vec2};
use micro::{
	App, Context, ContextSettings,
	color::ColorConstants,
	graphics::mesh::{MeshBuilder, ShapeStyle},
	input::MouseButton,
	math::{Circle, Ray, VecConstants},
};
use palette::{LinSrgb, LinSrgba};

fn main() -> Result<(), Box<dyn Error>> {
	micro::run(ContextSettings::default(), RaycastTest::new)
}

struct RaycastTest {
	ray: Ray,
	circle: Circle,
}

impl RaycastTest {
	fn new(_ctx: &mut Context) -> Result<Self, Box<dyn Error>> {
		Ok(Self {
			ray: Ray {
				origin: vec2(50.0, 300.0),
				direction: Vec2::RIGHT,
			},
			circle: Circle {
				center: vec2(400.0, 300.0),
				radius: 50.0,
			},
		})
	}
}

impl App for RaycastTest {
	type Error = Box<dyn Error>;

	fn update(&mut self, ctx: &mut Context, _delta_time: Duration) -> Result<(), Self::Error> {
		if ctx.is_mouse_button_down(MouseButton::Left) {
			self.ray.origin = ctx.mouse_position().as_vec2();
		} else {
			self.ray.direction =
				(ctx.mouse_position().as_vec2() - self.ray.origin).normalize_or_zero();
		}
		Ok(())
	}

	fn draw(&mut self, ctx: &mut Context) -> Result<(), Self::Error> {
		ctx.clear(LinSrgb::BLACK);
		let mut mesh_builder = MeshBuilder::new()
			.with_simple_polyline(
				2.0,
				[
					self.ray.origin,
					self.ray.origin + self.ray.direction * 10000.0,
				],
				LinSrgba::WHITE,
			)?
			.with_circle(ShapeStyle::Stroke(2.0), self.circle, LinSrgba::WHITE)?;
		for point in self.ray.circle_intersection_points(self.circle) {
			mesh_builder.add_circle(
				ShapeStyle::Fill,
				Circle {
					center: point,
					radius: 5.0,
				},
				LinSrgba::RED,
			)?;
		}
		mesh_builder.build(ctx).draw(ctx);
		Ok(())
	}
}
