use std::error::Error;

use glam::Vec2;
use micro::{
	graphics::{mesh::Mesh, DrawParams},
	Context, ContextSettings, State,
};
use palette::LinSrgba;

struct MainState;

impl MainState {
	fn new(ctx: &mut Context) -> Result<Self, Box<dyn Error>> {
		Ok(Self)
	}
}

impl State<Box<dyn Error>> for MainState {
	fn draw(&mut self, ctx: &mut Context) -> Result<(), Box<dyn Error>> {
		Mesh::simple_polyline(
			ctx,
			2.0,
			[
				Vec2::new(100.0, 100.0),
				Vec2::new(300.0, 100.0),
				Vec2::new(100.0, 300.0),
			],
			LinSrgba::new(1.0, 1.0, 1.0, 1.0),
		)?
		.draw(ctx, DrawParams::new());
		Ok(())
	}
}

fn main() -> Result<(), Box<dyn Error>> {
	micro::run(ContextSettings::default(), MainState::new)
}
