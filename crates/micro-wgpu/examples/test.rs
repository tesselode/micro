use std::error::Error;

use glam::{Vec2, vec2};
use micro_wgpu::{
	App, Context, ContextSettings, Event,
	color::ColorConstants,
	graphics::{Vertex2d, mesh::Mesh},
	input::Scancode,
};
use palette::{LinSrgb, LinSrgba};

const VERTICES: &[Vertex2d] = &[
	Vertex2d {
		position: vec2(-86.8241, 92.40386),
		texture_coords: Vec2::ZERO,
		color: LinSrgba::new(0.5, 0.0, 0.5, 1.0),
	},
	Vertex2d {
		position: vec2(-95.13406, 69.58647),
		texture_coords: Vec2::ZERO,
		color: LinSrgba::new(0.5, 0.0, 0.5, 1.0),
	},
	Vertex2d {
		position: vec2(-19.18549, -49.39706),
		texture_coords: Vec2::ZERO,
		color: LinSrgba::new(0.5, 0.0, 0.5, 1.0),
	},
	Vertex2d {
		position: vec2(59.66998, -47.3291),
		texture_coords: Vec2::ZERO,
		color: LinSrgba::new(0.5, 0.0, 0.5, 1.0),
	},
	Vertex2d {
		position: vec2(41.47372, 34.7359),
		texture_coords: Vec2::ZERO,
		color: LinSrgba::new(0.5, 0.0, 0.5, 1.0),
	},
];

const INDICES: &[u32] = &[0, 1, 4, 1, 2, 4, 2, 3, 4];

fn main() -> Result<(), Box<dyn Error>> {
	micro_wgpu::run(
		ContextSettings {
			resizable: true,
			..Default::default()
		},
		Test::new,
	)
}

struct Test {
	mesh: Mesh,
}

impl Test {
	fn new(ctx: &mut Context) -> Result<Self, Box<dyn Error>> {
		Ok(Self {
			mesh: Mesh::new(ctx, VERTICES, INDICES),
		})
	}
}

impl App for Test {
	type Error = Box<dyn Error>;

	fn event(&mut self, ctx: &mut Context, event: Event) -> Result<(), Self::Error> {
		if let Event::KeyPressed {
			key: Scancode::Return,
			..
		} = event
		{
			ctx.set_clear_color(LinSrgb::BLUE);
		}
		Ok(())
	}

	fn draw(&mut self, ctx: &mut Context) -> Result<(), Self::Error> {
		let ctx = &mut ctx.push_translation_2d(ctx.mouse_position().as_vec2());
		self.mesh.draw(ctx);
		Ok(())
	}
}
