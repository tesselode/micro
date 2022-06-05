use std::error::Error;

use micro::{
	graphics::{
		canvas::{Canvas, CanvasSettings, Msaa},
		color::Rgba,
		mesh::{Mesh, MeshBuilder, ShapeStyle},
		stencil::{StencilAction, StencilTest},
		DrawParams,
	},
	Context, ContextSettings, State,
};
use vek::Vec2;

struct MainState;

impl State<Box<dyn Error>> for MainState {
	fn draw(&mut self, ctx: &mut Context) -> Result<(), Box<dyn Error>> {
		ctx.clear(Rgba::BLACK);
		MeshBuilder::new()
			.with_color(Rgba::RED, |builder| {
				builder.add_circle(ShapeStyle::Fill, Vec2::new(100.0, 100.0), 64.0);
			})
			.with_color(Rgba::GREEN, |builder| {
				builder.add_circle(ShapeStyle::Fill, Vec2::new(200.0, 300.0), 32.0);
			})
			.build(ctx)
			.draw(ctx, DrawParams::new());
		Ok(())
	}
}

fn main() -> Result<(), Box<dyn Error>> {
	micro::run(ContextSettings::default(), |_| Ok(MainState))
}
