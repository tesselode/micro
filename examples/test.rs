use std::error::Error;

use micro::{
	color::Rgba, context::Context, draw_params::DrawParams, texture::Texture, Game, State,
};

struct MainState {
	texture: Texture,
}

impl MainState {
	fn new(ctx: &mut Context) -> Result<Self, Box<dyn Error>> {
		Ok(Self {
			texture: Texture::load(ctx, "examples/wall.png")?,
		})
	}
}

impl State<Box<dyn Error>> for MainState {
	fn draw(&mut self, ctx: &mut Context) -> Result<(), Box<dyn Error>> {
		ctx.clear(Rgba::new(0.1, 0.2, 0.3, 1.0));
		self.texture.draw(ctx, DrawParams::new())?;
		Ok(())
	}
}

fn main() -> Result<(), Box<dyn Error>> {
	Game::init()?.run(MainState::new)?;
	Ok(())
}
