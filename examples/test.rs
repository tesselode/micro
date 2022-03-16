use std::error::Error;

use fontdue::{Font, FontSettings};
use glam::{Vec2, Vec3};
use micro::{
	color::Rgba,
	context::Context,
	draw_params::DrawParams,
	image_data::ImageData,
	mesh::{Mesh, Vertex},
	rect::Rect,
	texture::Texture,
	Game, State,
};

struct MainState {
	mesh: Mesh,
	change_timer: usize,
}

impl MainState {
	fn new(ctx: &mut Context) -> Result<Self, Box<dyn Error>> {
		Ok(Self {
			mesh: Mesh::rectangle(
				ctx,
				Rect::new(Vec2::new(100.0, 100.0), Vec2::new(100.0, 100.0)),
			)?,
			change_timer: 100,
		})
	}
}

impl State<Box<dyn Error>> for MainState {
	fn draw(&mut self, ctx: &mut Context) -> Result<(), Box<dyn Error>> {
		if self.change_timer > 0 {
			self.change_timer -= 1;
			if self.change_timer == 0 {
				self.mesh.set_vertex(
					2,
					Vertex {
						position: Vec3::ZERO,
						texture_coords: Vec2::ZERO,
					},
				);
			}
		}
		ctx.clear(Rgba::new(0.1, 0.2, 0.3, 1.0));
		self.mesh.draw(ctx, DrawParams::new());
		Ok(())
	}
}

fn main() -> Result<(), Box<dyn Error>> {
	Game::init()?.run(MainState::new)?;
	Ok(())
}
