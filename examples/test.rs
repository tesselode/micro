use std::error::Error;

use glam::Vec2;
use micro::{
	color::Rgba,
	context::Context,
	image_data::ImageData,
	mesh::{Mesh, Vertex},
	texture::Texture,
};
use sdl2::{event::Event, keyboard::Keycode};

struct Game {
	mesh: Option<Mesh>,
}

impl Game {
	fn new(ctx: &Context) -> Result<Self, Box<dyn Error>> {
		let texture = Texture::new(ctx, &ImageData::from_file("examples/bricks.png")?)?;
		Ok(Self {
			mesh: Some(Mesh::new(
				ctx,
				&[
					Vertex {
						position: Vec2::new(0.5, 0.5),
						color: Rgba {
							red: 1.0,
							green: 0.0,
							blue: 0.0,
							alpha: 1.0,
						},
						texture_coords: Vec2::new(1.0, 1.0),
					},
					Vertex {
						position: Vec2::new(0.5, -0.5),
						color: Rgba {
							red: 0.0,
							green: 1.0,
							blue: 0.0,
							alpha: 1.0,
						},
						texture_coords: Vec2::new(1.0, 0.0),
					},
					Vertex {
						position: Vec2::new(-0.5, -0.5),
						color: Rgba {
							red: 0.0,
							green: 0.0,
							blue: 1.0,
							alpha: 1.0,
						},
						texture_coords: Vec2::new(0.0, 0.0),
					},
					Vertex {
						position: Vec2::new(-0.5, 0.5),
						color: Rgba {
							red: 1.0,
							green: 1.0,
							blue: 0.0,
							alpha: 1.0,
						},
						texture_coords: Vec2::new(0.0, 1.0),
					},
				],
				&[0, 1, 3, 1, 2, 3],
				&texture,
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
