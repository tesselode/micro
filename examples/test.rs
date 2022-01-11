use std::error::Error;

use micro::Context;
use sdl2::event::Event;

struct Game {
	alt_color: bool,
}

impl Game {
	fn new() -> Self {
		Self { alt_color: false }
	}
}

impl micro::Game<()> for Game {
	fn event(&mut self, _ctx: &mut Context, event: Event) -> Result<(), ()> {
		if let Event::KeyDown { .. } = event {
			self.alt_color = true;
		}
		Ok(())
	}

	fn draw(&mut self, ctx: &mut Context) -> Result<(), ()> {
		if self.alt_color {
			ctx.graphics().clear(0.3, 0.4, 0.5, 1.0);
		} else {
			ctx.graphics().clear(0.1, 0.2, 0.3, 1.0);
		}
		Ok(())
	}
}

fn main() -> Result<(), Box<dyn Error>> {
	let mut ctx = Context::new()?;
	ctx.run(Game::new()).unwrap();
	Ok(())
}
