use std::error::Error;

use glam::{UVec2, Vec2};
use micro::{
	graphics::{
		mesh::{Mesh, ShapeStyle},
		DrawParams,
	},
	input::Scancode,
	math::URect,
	window::WindowMode,
	Context, ContextSettings, Event, State,
};

struct MainState {
	mesh: Mesh,
}

impl MainState {
	fn new(ctx: &mut Context) -> Result<Self, Box<dyn Error>> {
		Ok(Self {
			mesh: Mesh::circle(
				ctx,
				ShapeStyle::Fill,
				Vec2::splat(1280.0 / 2.0),
				1280.0 / 2.0,
			),
		})
	}
}

impl State<Box<dyn Error>> for MainState {
	fn event(&mut self, ctx: &mut Context, event: Event) -> Result<(), Box<dyn Error>> {
		if let Event::KeyPressed(Scancode::Space) = event {
			ctx.set_window_mode(WindowMode::Windowed {
				size: UVec2::new(1280, 720),
			});
		}
		Ok(())
	}

	fn draw(&mut self, ctx: &mut Context) -> Result<(), Box<dyn Error>> {
		self.mesh.draw(ctx, DrawParams::new());
		Ok(())
	}
}

fn main() -> Result<(), Box<dyn Error>> {
	micro::run(ContextSettings::default(), MainState::new)
}
