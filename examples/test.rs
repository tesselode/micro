use std::error::Error;

use micro::{
	graphics::{
		canvas::{Canvas, CanvasSettings, Msaa},
		color::Rgba,
		mesh::{Mesh, MeshBuilder, ShapeStyle},
	},
	Context, ContextSettings, State,
};
use vek::{Mat4, Vec2, Vec3};

struct MainState {
	mesh: Mesh,
	canvas: Canvas,
}

impl MainState {
	fn new(ctx: &mut Context) -> Result<Self, Box<dyn Error>> {
		Ok(Self {
			mesh: MeshBuilder::new()
				.with_circle(ShapeStyle::Fill, Vec2::zero(), 64.0)
				.build(ctx),
			canvas: Canvas::new(
				ctx,
				Vec2::new(200, 200),
				CanvasSettings {
					msaa: Msaa::X16,
					..Default::default()
				},
			),
		})
	}
}

impl State<Box<dyn Error>> for MainState {
	fn draw(&mut self, ctx: &mut Context) -> Result<(), Box<dyn Error>> {
		ctx.clear(Rgba::BLACK);
		self.canvas
			.render_to(ctx, |ctx| -> Result<(), Box<dyn Error>> {
				ctx.clear(Rgba::new(0.1, 0.1, 0.1, 1.0));
				ctx.with_transform(
					Mat4::scaling_3d(Vec3::new(2.0, 2.0, 1.0)),
					|ctx| -> Result<(), Box<dyn Error>> {
						self.mesh.draw(ctx, Rgba::BLUE);
						Ok(())
					},
				)?;
				self.mesh.draw(ctx, Rgba::RED);
				Ok(())
			})?;
		self.canvas.draw(ctx, Vec2::new(100.0, 100.0));
		Ok(())
	}
}

fn main() -> Result<(), Box<dyn Error>> {
	micro::run(
		ContextSettings {
			resizable: true,
			..Default::default()
		},
		MainState::new,
	)
}
