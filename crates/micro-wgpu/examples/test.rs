use std::error::Error;

use glam::{Vec2, vec2};
use micro_wgpu::{
	App, Context, ContextSettings,
	graphics::{Vertex2d, mesh::Mesh},
};
use palette::LinSrgba;

const VERTICES: &[Vertex2d] = &[
	Vertex2d {
		position: vec2(-0.0868241, 0.49240386),
		texture_coords: Vec2::ZERO,
		color: LinSrgba::new(0.5, 0.0, 0.5, 1.0),
	},
	Vertex2d {
		position: vec2(-0.49513406, 0.06958647),
		texture_coords: Vec2::ZERO,
		color: LinSrgba::new(0.5, 0.0, 0.5, 1.0),
	},
	Vertex2d {
		position: vec2(-0.21918549, -0.44939706),
		texture_coords: Vec2::ZERO,
		color: LinSrgba::new(0.5, 0.0, 0.5, 1.0),
	},
	Vertex2d {
		position: vec2(0.35966998, -0.3473291),
		texture_coords: Vec2::ZERO,
		color: LinSrgba::new(0.5, 0.0, 0.5, 1.0),
	},
	Vertex2d {
		position: vec2(0.44147372, 0.2347359),
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

	fn draw(&mut self, ctx: &mut Context) -> Result<(), Self::Error> {
		self.mesh.draw(ctx);
		Ok(())
	}
}
