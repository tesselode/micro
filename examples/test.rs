use std::error::Error;

use glam::Vec2;
use micro::{context::Context, mesh::Mesh};
use sdl2::{event::Event, keyboard::Keycode};

struct Game {
	mesh: Option<Mesh>,
}

impl Game {
	fn new(ctx: &Context) -> Result<Self, Box<dyn Error>> {
		// -0.5, -0.5, 0.0, 0.5, -0.5, 0.0, 0.0, 0.5, 0.0
		Ok(Self {
			mesh: Some(Mesh::new(
				ctx,
				&[
					Vec2::new(0.5, 0.5),
					Vec2::new(0.5, -0.5),
					Vec2::new(-0.5, -0.5),
					Vec2::new(-0.5, 0.5),
				],
				&[0, 1, 3, 1, 2, 3],
			)?),
		})
	}
}

impl micro::Game<()> for Game {
	fn event(&mut self, _ctx: &mut Context, event: Event) -> Result<(), ()> {
		if let Event::KeyDown {
			keycode: Some(Keycode::Space),
			..
		} = event
		{
			self.mesh = None;
		}
		Ok(())
	}

	fn draw(&mut self, ctx: &mut Context) -> Result<(), ()> {
		ctx.graphics().clear(0.1, 0.2, 0.3, 1.0);
		if let Some(mesh) = &self.mesh {
			mesh.draw();
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
