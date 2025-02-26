use std::error::Error;

use micro_wgpu::{
	App, Context, ContextSettings, Event,
	color::ColorConstants,
	graphics::mesh::{Mesh, builder::ShapeStyle},
	input::Scancode,
	math::Circle,
};
use palette::LinSrgb;

fn main() -> Result<(), Box<dyn Error>> {
	micro_wgpu::run(
		ContextSettings {
			resizable: true,
			..Default::default()
		},
		Test::new,
	)
}

struct Test {}

impl Test {
	fn new(_ctx: &mut Context) -> Result<Self, Box<dyn Error>> {
		Ok(Self {})
	}
}

impl App for Test {
	type Error = Box<dyn Error>;

	fn event(&mut self, ctx: &mut Context, event: Event) -> Result<(), Self::Error> {
		if let Event::KeyPressed {
			key: Scancode::Return,
			..
		} = event
		{
			ctx.set_clear_color(LinSrgb::BLUE);
		}
		Ok(())
	}

	fn draw(&mut self, ctx: &mut Context) -> Result<(), Self::Error> {
		Mesh::circle(ctx, ShapeStyle::Fill, Circle::around_zero(50.0))?
			.color(LinSrgb::RED)
			.translated_2d(ctx.mouse_position().as_vec2())
			.draw(ctx);
		Ok(())
	}
}
