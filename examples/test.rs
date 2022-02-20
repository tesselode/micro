use std::error::Error;

use glam::Vec3;
use micro::{
	color::Rgba,
	context::Context,
	mesh::{Mesh, Vertex},
	Game, State,
};

struct MainState {
	mesh: Mesh,
}

impl MainState {
	fn new(ctx: &Context) -> Self {
		Self {
			mesh: Mesh::new(
				ctx,
				&[
					Vertex {
						position: Vec3::new(-0.5, -0.5, 0.0),
					},
					Vertex {
						position: Vec3::new(0.5, -0.5, 0.0),
					},
					Vertex {
						position: Vec3::new(0.0, 0.5, 0.0),
					},
				],
			)
			.unwrap(),
		}
	}
}

impl State<Box<dyn Error>> for MainState {
	fn draw(&mut self, ctx: &mut Context) -> Result<(), Box<dyn Error>> {
		ctx.clear(Rgba::new(0.1, 0.2, 0.3, 1.0));
		self.mesh.draw(ctx);
		Ok(())
	}
}

fn main() -> Result<(), Box<dyn Error>> {
	Game::init()?.run(|ctx| Ok(MainState::new(ctx)))?;
	Ok(())
}
