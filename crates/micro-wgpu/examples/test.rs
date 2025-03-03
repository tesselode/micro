use std::error::Error;

use glam::{Vec2, vec2};
use micro_wgpu::{
	App, Context, ContextSettings,
	graphics::{
		canvas::{Canvas, CanvasSettings, RenderToCanvasSettings},
		graphics_pipeline::{GraphicsPipeline, GraphicsPipelineSettings},
		mesh::{Mesh, builder::ShapeStyle},
	},
	math::{Circle, URect},
};

fn main() -> Result<(), Box<dyn Error>> {
	micro_wgpu::run(
		ContextSettings {
			resizable: true,
			..Default::default()
		},
		Test::new,
	)
}

struct Test {
	canvas: Canvas,
	graphics_pipeline: GraphicsPipeline,
}

impl Test {
	fn new(ctx: &mut Context) -> Result<Self, Box<dyn Error>> {
		Ok(Self {
			canvas: Canvas::new(
				ctx,
				ctx.window_size(),
				CanvasSettings {
					sample_count: 8,
					..Default::default()
				},
			),
			graphics_pipeline: GraphicsPipeline::new(
				ctx,
				GraphicsPipelineSettings {
					sample_count: 8,
					..Default::default()
				},
			),
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
			let ctx = &mut ctx.push_scale_2d(Vec2::splat(2.0));
			Mesh::circle(
				ctx,
				ShapeStyle::Fill,
				Circle::new(vec2(100.0, 100.0), 100.0),
			)?
			.scissor_rect(URect::new((0, 0), (100, 100)))
			.draw(ctx);
		}

		self.canvas.draw(ctx);

		Ok(())
	}
}
