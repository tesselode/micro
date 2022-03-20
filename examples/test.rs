use std::error::Error;

use micro::{
	graphics::{
		color::Rgba,
		mesh::{Mesh, MeshBuilder, ShapeStyle},
		DrawParams,
	},
	math::Rect,
	Context, ContextSettings, State,
};
use vek::Mat4;

struct MainState {
	mesh: Mesh,
}

impl MainState {
	fn new(ctx: &mut Context) -> Result<Self, Box<dyn Error>> {
		let mesh = MeshBuilder::new()
			.with_rectangle(
				ShapeStyle::Stroke(8.0),
				Rect::xywh(50.0, 50.0, 200.0, 300.0),
			)?
			.build(ctx)?;
		Ok(Self { mesh })
	}
}

impl State<Box<dyn Error>> for MainState {
	fn draw(&mut self, ctx: &mut Context) -> Result<(), Box<dyn Error>> {
		ctx.clear(Rgba::BLACK);
		self.mesh.draw(ctx, DrawParams::new());
		Ok(())
	}
}

fn main() -> Result<(), Box<dyn Error>> {
	Context::new(ContextSettings {
		window_size: (1280, 720),
		vsync: false,
		..Default::default()
	})?
	.run(MainState::new)?;
	Ok(())
}
