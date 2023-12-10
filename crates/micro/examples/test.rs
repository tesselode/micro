use std::{error::Error, time::Duration};

use glam::Vec2;
use micro::{
	graphics::{
		mesh::{Mesh, ShapeStyle},
		ColorConstants, DrawParams,
	},
	math::Circle,
	Context, ContextSettings, State,
};
use palette::LinSrgba;

fn main() {
	micro::run(
		ContextSettings {
			resizable: true,
			..Default::default()
		},
		MainState::new,
	)
}

struct MainState {
	mesh: Mesh,
}

impl MainState {
	fn new(ctx: &mut Context) -> Result<Self, Box<dyn Error>> {
		Ok(Self {
			mesh: Mesh::circle(
				ctx,
				ShapeStyle::Stroke(7.0),
				Circle {
					center: Vec2::splat(200.0),
					radius: 50.0,
				},
				LinSrgba::RED,
			)?,
		})
	}
}

impl State<Box<dyn Error>> for MainState {
	fn update(&mut self, ctx: &mut Context, delta_time: Duration) -> Result<(), Box<dyn Error>> {
		Ok(())
	}

	fn draw(&mut self, ctx: &mut Context) -> Result<(), Box<dyn Error>> {
		ctx.clear(LinSrgba::BLACK);
		self.mesh.draw(ctx, DrawParams::new());
		Ok(())
	}
}
