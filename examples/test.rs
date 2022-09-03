use glam::Vec2;
use micro::{
	graphics::{
		color::Rgba,
		mesh::{Mesh, ShapeStyle},
		DrawParams,
	},
	math::Rect,
	Context, ContextSettings, State,
};

struct MainState;

impl State for MainState {
	fn draw(&mut self, ctx: &mut Context) {
		Mesh::styled_rectangle(
			ctx,
			ShapeStyle::Stroke(2.0),
			Rect::new(Vec2::new(-50.0, -50.0), Vec2::new(50.0, 50.0)),
		)
		.draw(
			ctx,
			DrawParams::new()
				.position(Vec2::splat(200.0))
				.scale(Vec2::splat(2.0))
				.rotation(1.0)
				.color(Rgba::RED),
		);
	}
}

fn main() {
	micro::run(ContextSettings::default(), |_| MainState)
}
