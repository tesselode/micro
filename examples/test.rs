use std::error::Error;

use micro::{
	graphics::{
		color::Rgba,
		mesh::{Mesh, MeshBuilder, ShapeStyle},
	},
	Context, State,
};
use vek::{Mat4, Vec2, Vec3};

struct MainState {
	mesh: Mesh,
}

impl MainState {
	fn new(ctx: &mut Context) -> Result<Self, Box<dyn Error>> {
		Ok(Self {
			mesh: MeshBuilder::new()
				.with_circle(ShapeStyle::Fill, Vec2::zero(), 64.0)?
				.build(ctx)?,
		})
	}
}

impl State<Box<dyn Error>> for MainState {
	fn draw(&mut self, ctx: &mut Context) -> Result<(), Box<dyn Error>> {
		ctx.with_transform(
			Mat4::scaling_3d(Vec3::new(2.0, 2.0, 1.0)),
			|ctx| -> Result<(), Box<dyn Error>> {
				self.mesh.draw(ctx, Rgba::BLUE);
				Ok(())
			},
		)?;
		self.mesh.draw(ctx, Rgba::RED);
		Ok(())
	}
}

fn main() -> Result<(), Box<dyn Error>> {
	Context::new(Default::default())?.run(MainState::new)
}
