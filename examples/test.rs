use glam::Vec2;
use micro::{
	graphics::{color::Rgba, mesh::ShapeStyle},
	ui::{align::Align, circle::Circle, list::List, Constraints, Widget},
	Context, ContextSettings, State,
};

struct MainState;

impl MainState {
	fn new(_ctx: &mut Context) -> Self {
		Self
	}
}

impl State for MainState {
	fn draw(&mut self, ctx: &mut Context) {
		ctx.clear(Rgba::BLACK);
		Align::center(
			List::horizontal()
				.with_child(Circle::new(50.0, ShapeStyle::Fill))
				.with_child(Circle::new(25.0, ShapeStyle::Fill))
				.with_child(Circle::new(75.0, ShapeStyle::Fill)),
		)
		.build(
			ctx,
			Constraints {
				min_size: Vec2::ZERO,
				max_size: ctx.window_size().as_vec2(),
			},
		)
		.draw(ctx);
	}
}

fn main() {
	micro::run(ContextSettings::default(), MainState::new);
}
