use std::error::Error;

use glam::{Mat4, Vec2, Vec3};
use micro::{
	color::Rgba,
	context::Context,
	mesh::{Mesh, Vertex},
	texture::Texture,
	Game, State,
};

struct MainState {
	mesh: Mesh,
}

impl MainState {
	fn new(ctx: &Context) -> Self {
		let texture = Texture::load(ctx, "examples/wall.png").unwrap();
		Self {
			mesh: {
				let mut mesh = Mesh::new(
					ctx,
					&[
						Vertex {
							position: Vec3::new(0.5, 0.5, 0.0),
							texture_coords: Vec2::new(1.0, 1.0),
						},
						Vertex {
							position: Vec3::new(0.5, -0.5, 0.0),
							texture_coords: Vec2::new(1.0, -1.0),
						},
						Vertex {
							position: Vec3::new(-0.5, -0.5, 0.0),
							texture_coords: Vec2::new(-1.0, -1.0),
						},
						Vertex {
							position: Vec3::new(-0.5, 0.5, 0.0),
							texture_coords: Vec2::new(-1.0, 1.0),
						},
					],
					&[0, 1, 3, 1, 2, 3],
				)
				.unwrap();
				mesh.set_texture(Some(&texture));
				mesh
			},
		}
	}
}

impl State<Box<dyn Error>> for MainState {
	fn draw(&mut self, ctx: &mut Context) -> Result<(), Box<dyn Error>> {
		ctx.clear(Rgba::new(0.1, 0.2, 0.3, 1.0));
		self.mesh.draw(
			ctx,
			Mat4::from_translation(Vec3::new(-0.25, -0.25, 0.0))
				* Mat4::from_scale(Vec3::splat(0.1)),
		);
		Ok(())
	}
}

fn main() -> Result<(), Box<dyn Error>> {
	Game::init()?.run(|ctx| Ok(MainState::new(ctx)))?;
	Ok(())
}
