use std::error::Error;

use glam::Vec2;
use micro::{context::Context, mesh::Mesh};
use sdl2::event::Event;

struct Game {
	draw_mesh: bool,
	mesh: Mesh,
}

impl Game {
	fn new(ctx: &Context) -> Result<Self, Box<dyn Error>> {
		// -0.5, -0.5, 0.0, 0.5, -0.5, 0.0, 0.0, 0.5, 0.0
		Ok(Self {
			draw_mesh: true,
			mesh: Mesh::new(
				ctx,
				&[
					Vec2::new(-0.5, -0.5),
					Vec2::new(0.5, -0.5),
					Vec2::new(0.0, 0.5),
				],
			)?,
		})
	}
}

impl micro::Game<()> for Game {
	fn event(&mut self, _ctx: &mut Context, event: Event) -> Result<(), ()> {
		if let Event::KeyDown { .. } = event {
			self.draw_mesh = !self.draw_mesh;
		}
		Ok(())
	}

	fn draw(&mut self, ctx: &mut Context) -> Result<(), ()> {
		ctx.graphics().clear(0.1, 0.2, 0.3, 1.0);
		if self.draw_mesh {
			self.mesh.draw(ctx);
		}
		Ok(())
	}
}

fn main() -> Result<(), Box<dyn Error>> {
	let mut ctx = Context::new()?;
	let game = Game::new(&ctx)?;
	ctx.run(game).unwrap();
	Ok(())
}
