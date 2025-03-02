use std::error::Error;

use glam::vec2;
use micro_wgpu::{
	App, Context, ContextSettings, Event,
	color::ColorConstants,
	graphics::{
		canvas::{Canvas, CanvasSettings, RenderToCanvasSettings},
		graphics_pipeline::{GraphicsPipeline, GraphicsPipelineSettings},
		mesh::{Mesh, builder::ShapeStyle},
	},
	input::Scancode,
	math::Circle,
};
use palette::{LinSrgb, LinSrgba};

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
				ctx.window_size() / 2,
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
		{
			let ctx = &mut self.canvas.render_to(
				ctx,
				RenderToCanvasSettings {
					clear_color: Some(LinSrgba::BLACK),
				},
			);
			let ctx = &mut ctx.push_graphics_pipeline(&self.graphics_pipeline);
			Mesh::circle(
				ctx,
				ShapeStyle::Fill,
				Circle {
					center: vec2(100.0, 100.0),
					radius: 50.0,
				},
			)?
			.draw(ctx);
		}

		self.canvas.draw(ctx);

		Ok(())
	}
}
