use std::error::Error;

use glam::{Mat4, Vec2, Vec3};
use micro::{
	canvas::Canvas,
	color::Rgba,
	context::Context,
	draw_params::DrawParams,
	image_data::ImageData,
	mesh::{Mesh, Vertex},
	texture::Texture,
};
use sdl2::{event::Event, keyboard::Keycode};

struct Game {
	canvas: Canvas,
	mesh: Mesh,
	color: Rgba,
	angle: f32,
}

impl Game {
	fn new(ctx: &Context) -> Result<Self, Box<dyn Error>> {
		let texture = Texture::new(ctx, &ImageData::from_file("examples/bricks.png")?)?;
		Ok(Self {
			canvas: Canvas::new(ctx, 800, 600)?,
			mesh: Mesh::new(
				ctx,
				&[
					Vertex {
						position: Vec2::new(800.0, 600.0),
						color: Rgba {
							red: 1.0,
							green: 0.0,
							blue: 0.0,
							alpha: 1.0,
						},
						texture_coords: Vec2::new(1.0, 1.0),
					},
					Vertex {
						position: Vec2::new(800.0, 0.0),
						color: Rgba {
							red: 0.0,
							green: 1.0,
							blue: 0.0,
							alpha: 1.0,
						},
						texture_coords: Vec2::new(1.0, 0.0),
					},
					Vertex {
						position: Vec2::new(0.0, 0.0),
						color: Rgba {
							red: 0.0,
							green: 0.0,
							blue: 1.0,
							alpha: 1.0,
						},
						texture_coords: Vec2::new(0.0, 0.0),
					},
					Vertex {
						position: Vec2::new(0.0, 600.0),
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
				Some(&texture),
			)?,
			color: Rgba::WHITE,
			angle: 0.0,
		})
	}
}

impl micro::Game<Box<dyn Error>> for Game {
	fn event(&mut self, _ctx: &mut Context, event: Event) -> Result<(), Box<dyn Error>> {
		if let Event::KeyDown {
			keycode: Some(Keycode::Space),
			..
		} = event
		{
			self.color = Rgba {
				red: 1.0,
				green: 0.0,
				blue: 0.0,
				alpha: 1.0,
			};
		}
		Ok(())
	}

	fn update(&mut self, _ctx: &mut Context) -> Result<(), Box<dyn Error>> {
		self.angle += 1.0 / 60.0;
		Ok(())
	}

	fn draw(&mut self, ctx: &mut Context) -> Result<(), Box<dyn Error>> {
		ctx.graphics().clear(0.0, 0.0, 0.0, 0.0);
		self.canvas.draw_on(|| {
			ctx.graphics().clear(0.1, 0.2, 0.3, 1.0);
			self.mesh.draw(
				ctx,
				DrawParams {
					color: self.color,
					transform: Mat4::from_translation(Vec3::new(400.0, 300.0, 0.0))
						* Mat4::from_rotation_z(self.angle)
						* Mat4::from_translation(Vec3::new(-400.0, -300.0, 0.0)),
				},
			);
		});
		Mesh::rectangle(
			ctx,
			Vec2::ZERO,
			Vec2::new(100.0, 100.0),
			Some(&self.canvas.texture()),
		)?
		.draw(ctx, DrawParams::new());
		Ok(())
	}
}

fn main() -> Result<(), Box<dyn Error>> {
	let mut ctx = Context::new()?;
	let game = Game::new(&ctx)?;
	ctx.run(game).unwrap();
	Ok(())
}
