use std::error::Error;

use glam::vec2;
use micro::{
	App, Context, ContextSettings,
	color::ColorConstants,
	graphics::{
		Canvas, CanvasSettings, GraphicsPipeline, GraphicsPipelineBuilder, RenderToCanvasSettings,
		StencilState,
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
		let canvas = Canvas::new(
			ctx,
			ctx.window_size(),
			CanvasSettings {
				sample_count: 8,
				..Default::default()
			},
		);
		let graphics_pipeline = GraphicsPipelineBuilder::for_canvas(&canvas).build(ctx);
		Ok(Self {
			canvas,
			graphics_pipeline,
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
			let mesh = Mesh::circle(
				ctx,
				ShapeStyle::Fill,
				Circle {
					center: vec2(400.0, 300.0),
					radius: 100.0,
				},
			)?
			.color(LinSrgba::RED);
			self.graphics_pipeline.draw(ctx, mesh);
		}

		ctx.draw(self.canvas.clone());

		Ok(())
	}
}
