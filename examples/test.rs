use std::error::Error;

use glam::Vec2;
use micro::{
	graphics::{
		color::Rgba,
		mesh::{Mesh, Vertex},
		texture::{Texture, TextureSettings, TextureWrapping},
		DrawParams,
	},
	Context, Game, State,
};

struct MainState {
	texture: Texture,
	mesh: Mesh,
}

impl MainState {
	fn new(ctx: &mut Context) -> Result<Self, Box<dyn Error>> {
		let texture = Texture::load(
			ctx,
			"examples/wall.png",
			TextureSettings {
				wrapping: TextureWrapping::ClampToBorder(Rgba::BLUE),
			},
		)?;
		let mesh = Mesh::new(
			ctx,
			&[
				Vertex {
					position: Vec2::new(200.0, 200.0),
					texture_coords: Vec2::new(2.0, 2.0),
				},
				Vertex {
					position: Vec2::new(200.0, 100.0),
					texture_coords: Vec2::new(2.0, 0.0),
				},
				Vertex {
					position: Vec2::new(100.0, 100.0),
					texture_coords: Vec2::new(0.0, 0.0),
				},
				Vertex {
					position: Vec2::new(100.0, 200.0),
					texture_coords: Vec2::new(0.0, 2.0),
				},
			],
			&[0, 1, 3, 1, 2, 3],
		)?;
		Ok(Self { texture, mesh })
	}
}

impl State<Box<dyn Error>> for MainState {
	fn draw(&mut self, ctx: &mut Context) -> Result<(), Box<dyn Error>> {
		ctx.clear(Rgba::BLACK);
		self.mesh
			.draw_textured(ctx, &self.texture, DrawParams::new());
		Ok(())
	}
}

fn main() -> Result<(), Box<dyn Error>> {
	Game::init()?.run(MainState::new)?;
	Ok(())
}
