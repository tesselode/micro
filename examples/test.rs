use std::error::Error;

use glam::{Mat4, Vec3};
use micro::{
	blend_mode::{BlendAlphaMode, BlendMode},
	color::Rgba,
	context::Context,
	draw_params::DrawParams,
	texture::Texture,
	Game, State,
};

struct MainState {
	texture: Texture,
}

impl MainState {
	fn new(ctx: &Context) -> Self {
		Self {
			texture: Texture::load(ctx, "examples/wall.png").unwrap(),
		}
	}
}

impl State<Box<dyn Error>> for MainState {
	fn draw(&mut self, ctx: &mut Context) -> Result<(), Box<dyn Error>> {
		ctx.clear(Rgba::new(0.1, 0.2, 0.3, 1.0));
		self.texture
			.draw(ctx, BlendMode::Add(BlendAlphaMode::AlphaMultiply))?;
		self.texture.draw(
			ctx,
			DrawParams::new()
				.transform(Mat4::from_translation(Vec3::new(50.0, 50.0, 0.0)))
				.color(Rgba::WHITE.with_alpha(0.75)),
		)?;
		Ok(())
	}
}

fn main() -> Result<(), Box<dyn Error>> {
	Game::init()?.run(|ctx| Ok(MainState::new(ctx)))?;
	Ok(())
}
