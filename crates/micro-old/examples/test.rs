use std::error::Error;

use glam::{UVec2, Vec2};
use micro_old::{
	graphics::{
		color::Rgba,
		mesh::{Mesh, ShapeStyle},
		DrawParams,
	},
	input::Scancode,
	math::triangulate_polygon,
	window::WindowMode,
	Context, ContextSettings, Event, State,
};

const VERTICES: &[Vec2] = &[
	Vec2::new(10.0, 10.0),
	Vec2::new(100.0, 10.0),
	Vec2::new(150.0, 100.0),
	Vec2::new(100.0, 150.0),
	Vec2::new(50.0, 100.0),
];

enum MainState {
	Polygon { mesh: Mesh },
	Triangles { meshes: Vec<Mesh> },
}

impl MainState {
	fn new(ctx: &mut Context) -> Result<Self, Box<dyn Error>> {
		Ok(Self::Polygon {
			mesh: Mesh::polygon(ctx, ShapeStyle::Stroke(4.0), VERTICES),
		})
	}
}

impl State<Box<dyn Error>> for MainState {
	fn event(&mut self, ctx: &mut Context, event: Event) -> Result<(), Box<dyn Error>> {
		if let Event::KeyPressed(Scancode::Space) = event {
			*self = Self::Triangles {
				meshes: triangulate_polygon(VERTICES)
					.iter()
					.map(|triangle| Mesh::polygon(ctx, ShapeStyle::Stroke(4.0), triangle))
					.collect(),
			}
		}
		Ok(())
	}

	fn draw(&mut self, ctx: &mut Context) -> Result<(), Box<dyn Error>> {
		ctx.clear(Rgba::BLACK);
		match self {
			MainState::Polygon { mesh } => mesh.draw(ctx, DrawParams::new()),
			MainState::Triangles { meshes } => {
				for mesh in meshes {
					mesh.draw(ctx, DrawParams::new());
				}
			}
		}
		Ok(())
	}
}

fn main() -> Result<(), Box<dyn Error>> {
	micro_old::run(
		ContextSettings {
			window_mode: WindowMode::Windowed {
				size: UVec2::new(500, 500),
			},
			resizable: true,
			..Default::default()
		},
		MainState::new,
	)
}
