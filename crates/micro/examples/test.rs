use std::error::Error;

use glam::{UVec2, Vec2};
use micro::{
	graphics::{color::Rgba, texture::Texture, Mesh, Vertex},
	window::WindowMode,
	Context, ContextSettings, State,
};

const INDICES: &[u16] = &[0, 1, 4, 1, 2, 4, 2, 3, 4];

struct MainState {
	mesh: Mesh,
	texture: Texture,
}

impl MainState {
	fn new(ctx: &mut Context) -> Result<Self, Box<dyn Error>> {
		let vertices: &[Vertex] = &[
			Vertex {
				position: Vec2::splat(500.0) + 100.0 * Vec2::new(-0.0868241, 0.49240386),
				texture_coords: Vec2::new(0.4131759, 0.99240386),
				color: Rgba::WHITE,
			},
			Vertex {
				position: Vec2::splat(500.0) + 100.0 * Vec2::new(-0.49513406, 0.06958647),
				texture_coords: Vec2::new(0.0048659444, 0.56958647),
				color: Rgba::WHITE,
			},
			Vertex {
				position: Vec2::splat(500.0) + 100.0 * Vec2::new(-0.21918549, -0.44939706),
				texture_coords: Vec2::new(0.28081453, 0.05060294),
				color: Rgba::WHITE,
			},
			Vertex {
				position: Vec2::splat(500.0) + 100.0 * Vec2::new(0.35966998, -0.3473291),
				texture_coords: Vec2::new(0.85967, 0.1526709),
				color: Rgba::WHITE,
			},
			Vertex {
				position: Vec2::splat(500.0) + 100.0 * Vec2::new(0.44147372, 0.2347359),
				texture_coords: Vec2::new(0.9414737, 0.7347359),
				color: Rgba::WHITE,
			},
		];

		Ok(Self {
			mesh: Mesh::new(ctx, vertices, INDICES),
			texture: Texture::from_file(ctx, "crates/micro/examples/tree.png")?,
		})
	}
}

impl State<Box<dyn Error>> for MainState {
	fn draw(&mut self, ctx: &mut Context) -> Result<(), Box<dyn Error>> {
		self.mesh
			.draw_textured(ctx, &self.texture, Vec2::new(100.0, 100.0));
		self.mesh.draw(ctx, Rgba::RED);
		Ok(())
	}
}

fn main() -> Result<(), Box<dyn Error>> {
	micro::run(
		ContextSettings {
			window_mode: WindowMode::Windowed {
				size: UVec2::new(1280, 720),
			},
			..Default::default()
		},
		MainState::new,
	)
}
