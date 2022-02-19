use std::error::Error;

use micro::{color::Rgba, context::Context, Game, State};

struct MainState;

impl State<Box<dyn Error>> for MainState {
	fn draw(&mut self, ctx: &mut Context) -> Result<(), Box<dyn Error>> {
		ctx.clear(Rgba::new(0.1, 0.2, 0.3, 1.0));
		Ok(())
	}
}

fn main() -> Result<(), Box<dyn Error>> {
	Game::init()?.run(|_| Ok(MainState))?;
	Ok(())
}
