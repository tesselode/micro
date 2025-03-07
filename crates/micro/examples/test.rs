use std::error::Error;

use glam::vec2;
use micro::{
	App, Context, ContextSettings,
	color::ColorConstants,
	graphics::{
		Canvas, CanvasSettings, GraphicsPipeline, RenderToCanvasSettings, StencilState,
		mesh::{Mesh, ShapeStyle},
	},
	math::Circle,
};
use palette::LinSrgba;
use wgpu::{CompareFunction, StencilOperation};

fn main() -> Result<(), Box<dyn Error>> {
	micro::run(ContextSettings::default(), Test::new)
}

struct Test {
	canvas: Canvas,
	graphics_pipeline: GraphicsPipeline,
}

impl Test {
	pub fn new(ctx: &mut Context) -> Result<Self, Box<dyn Error>> {
		Ok(Self {
			canvas: Canvas::new(
				ctx,
				ctx.window_size(),
				CanvasSettings {
					hdr: true,
					..Default::default()
				},
			),
			graphics_pipeline: GraphicsPipeline::builder().hdr(true).build(ctx),
		})
	}
}

impl App for Test {
	type Error = Box<dyn Error>;

	fn draw(&mut self, ctx: &mut Context) -> Result<(), Self::Error> {
		{
			let ctx = &mut self
				.canvas
				.render_to(ctx, RenderToCanvasSettings::default());
			let ctx = &mut ctx.push_graphics_pipeline(&self.graphics_pipeline);
			Mesh::circle(
				ctx,
				ShapeStyle::Fill,
				Circle {
					center: vec2(400.0, 300.0),
					radius: 100.0,
				},
			)?
			.color(LinSrgba::RED)
			.draw(ctx);
		}

		self.canvas.draw(ctx);

		Ok(())
	}
}
