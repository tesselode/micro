use std::error::Error;

use glam::{UVec2, Vec2};
use micro::{
	graphics::{
		canvas::{Canvas, RenderToCanvasSettings},
		color::Rgba,
		mesh::{Mesh, ShapeStyle},
	},
	Context, ContextSettings, State,
};

struct MainState {
	canvas: Canvas,
	mesh: Mesh,
}

impl MainState {
	fn new(ctx: &mut Context) -> Result<Self, Box<dyn Error>> {
		Ok(Self {
			canvas: Canvas::new(ctx, UVec2::new(200, 200)),
			mesh: Mesh::circle(ctx, ShapeStyle::Fill, Vec2::ZERO, 200.0),
		})
	}
}

impl State<Box<dyn Error>> for MainState {
	fn draw(&mut self, ctx: &mut Context) -> Result<(), Box<dyn Error>> {
		self.canvas.render_to(
			ctx,
			RenderToCanvasSettings {
				clear_color: Some(Rgba::RED),
				clear_stencil_value: None,
			},
			|ctx| self.mesh.draw(ctx, Rgba::BLUE),
		);
		self.canvas.draw(ctx, Vec2::new(50.0, 50.0));
		Ok(())
	}
}

fn main() -> Result<(), Box<dyn Error>> {
	micro::run(ContextSettings::default(), MainState::new)
}
