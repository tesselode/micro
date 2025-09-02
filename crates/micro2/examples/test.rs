use micro2::{
	App, Context, ContextSettings, Event,
	graphics::{
		StencilState,
		mesh::{Mesh, ShapeStyle},
		text::{Font, FontSettings, LayoutSettings, Text},
	},
	input::Scancode,
	math::{Circle, Rect},
};
use wgpu::{CompareFunction, StencilOperation};

fn main() {
	micro2::run(ContextSettings::default(), Test::new);
}

struct Test {}

impl Test {
	fn new(_ctx: &mut Context) -> Self {
		Self {}
	}
}

impl App for Test {
	fn event(&mut self, ctx: &mut Context, event: Event) {
		if let Event::KeyPressed {
			key: Scancode::Escape,
			..
		} = event
		{
			ctx.quit();
		}
	}

	fn draw(&mut self, ctx: &mut Context) {
		{
			let ctx =
				&mut ctx.push_stencil_state(StencilState::write(StencilOperation::Replace, 1));
			Mesh::circle(
				ctx,
				ShapeStyle::Fill,
				Circle {
					center: ctx.window_size().as_vec2() / 2.0,
					radius: 100.0,
				},
			)
			.unwrap()
			.draw(ctx);
		}
		{
			let ctx = &mut ctx.push_stencil_state(StencilState::read(CompareFunction::Equal, 1));
			Mesh::rectangle(ctx, Rect::new((0.0, 0.0), ctx.window_size().as_vec2())).draw(ctx);
		}
	}
}
