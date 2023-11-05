use std::error::Error;

use glam::{UVec2, Vec2};
use micro::{
	graphics::{
		mesh::{Mesh, ShapeStyle},
		ColorConstants, DrawParams, Scaler,
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
	scaler: Scaler,
	mesh: Mesh,
}

impl MainState {
	fn new(ctx: &mut Context) -> Result<Self, Box<dyn Error>> {
		Ok(Self {
			scaler: Scaler::new(ctx, UVec2::splat(100), false),
			mesh: Mesh::circle(
				ctx,
				ShapeStyle::Fill,
				Circle {
					center: Vec2::splat(50.0),
					radius: 16.0,
				},
				LinSrgba::WHITE,
			)?,
		})
	}
}

impl State<Box<dyn Error>> for MainState {
	fn draw(&mut self, ctx: &mut Context) -> Result<(), Box<dyn Error>> {
		ctx.clear(LinSrgba::BLACK);
		self.scaler.draw(ctx, |ctx| {
			self.mesh.draw(ctx, DrawParams::new());
		});
		Ok(())
	}
}
