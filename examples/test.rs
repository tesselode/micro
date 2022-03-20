use std::error::Error;

use glam::Vec2;
use micro::{
	graphics::{
		color::Rgba,
		mesh::{Mesh, MeshBuilder},
		DrawParams,
	},
	Context, ContextSettings, State,
};

struct MainState {
	mesh: Mesh,
}

impl MainState {
	fn new(ctx: &mut Context) -> Result<Self, Box<dyn Error>> {
		let mesh = MeshBuilder::new()
			.polyline(
				&[
					Vec2::new(50.0, 50.0),
					Vec2::new(100.0, 50.0),
					Vec2::new(50.0, 100.0),
				],
				4.0,
			)?
			.build(ctx)?;
		Ok(Self { mesh })
	}
}

impl State<Box<dyn Error>> for MainState {
	fn draw(&mut self, ctx: &mut Context) -> Result<(), Box<dyn Error>> {
		ctx.clear(Rgba::BLACK);
		self.mesh.draw(ctx, DrawParams::new());
		Ok(())
	}
}

fn main() -> Result<(), Box<dyn Error>> {
	Context::new(ContextSettings {
		window_width: 1280,
		window_height: 720,
		vsync: false,
		..Default::default()
	})?
	.run(MainState::new)?;
	Ok(())
}
