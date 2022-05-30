use std::error::Error;

use micro::{
	graphics::{
		color::Rgba,
		mesh::{Mesh, MeshBuilder, ShapeStyle},
		stencil::{StencilAction, StencilTest},
	},
	Context, ContextSettings, State,
};
use vek::Vec2;

struct MainState {
	mesh: Mesh,
}

impl MainState {
	fn new(ctx: &Context) -> Self {
		Self {
			mesh: MeshBuilder::new()
				.with_circle(ShapeStyle::Fill, Vec2::zero(), 200.0)
				.build(ctx),
		}
	}
}

impl State<Box<dyn Error>> for MainState {
	fn draw(&mut self, ctx: &mut Context) -> Result<(), Box<dyn Error>> {
		ctx.clear(Rgba::BLACK);
		ctx.write_to_stencil(StencilAction::Increment, |ctx| {
			self.mesh.draw(ctx, Vec2::new(300.0, 300.0));
			self.mesh.draw(ctx, Vec2::new(400.0, 300.0));
		});
		ctx.with_stencil(StencilTest::NotEqual, 1, |ctx| {
			self.mesh.draw(ctx, Vec2::new(500.0, 300.0));
		});
		Ok(())
	}
}

fn main() -> Result<(), Box<dyn Error>> {
	micro::run(ContextSettings::default(), |ctx| Ok(MainState::new(ctx)))
}
