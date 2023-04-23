use std::error::Error;

use glam::{UVec2, Vec2};
use micro::{
	graphics::{
		canvas::{Canvas, CanvasSettings, RenderToCanvasSettings},
		color::Rgba,
		graphics_pipeline::{GraphicsPipeline, GraphicsPipelineSettings},
		mesh::{Mesh, ShapeStyle},
		DrawParams,
	},
	Context, ContextSettings, State,
};

struct MainState {
	canvas: Canvas,
	mesh: Mesh,
	graphics_pipeline: GraphicsPipeline,
}

impl MainState {
	fn new(ctx: &mut Context) -> Result<Self, Box<dyn Error>> {
		Ok(Self {
			canvas: Canvas::new(
				ctx,
				UVec2::new(200, 200),
				CanvasSettings { sample_count: 8 },
			),
			mesh: Mesh::circle(ctx, ShapeStyle::Fill, Vec2::ZERO, 200.0),
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

impl State<Box<dyn Error>> for MainState {
	fn draw(&mut self, ctx: &mut Context) -> Result<(), Box<dyn Error>> {
		self.canvas.render_to(
			ctx,
			RenderToCanvasSettings {
				clear_color: Some(Rgba::RED),
				clear_stencil_value: None,
			},
			|ctx| {
				self.mesh.draw(
					ctx,
					DrawParams::new()
						.color(Rgba::BLUE)
						.graphics_pipeline(&self.graphics_pipeline),
				)
			},
		);
		self.canvas.draw(ctx, Vec2::new(50.0, 50.0));
		Ok(())
	}
}

fn main() -> Result<(), Box<dyn Error>> {
	micro::run(ContextSettings::default(), MainState::new)
}
