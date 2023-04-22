use std::error::Error;

use glam::Vec2;
use micro::{
	graphics::{
		color::Rgba,
		graphics_pipeline::{GraphicsPipeline, GraphicsPipelineSettings},
		mesh::Mesh,
		shader::DefaultShader,
		BlendAlphaMode, BlendMode, DrawParams,
	},
	math::Rect,
	Context, ContextSettings, State,
};

struct MainState {
	mesh: Mesh,
	graphics_pipeline: GraphicsPipeline,
}

impl MainState {
	fn new(ctx: &mut Context) -> Result<Self, Box<dyn Error>> {
		Ok(Self {
			mesh: Mesh::rectangle(ctx, Rect::xywh(50.0, 50.0, 100.0, 150.0)),
			graphics_pipeline: GraphicsPipeline::new(
				ctx,
				GraphicsPipelineSettings {
					blend_mode: BlendMode::Subtract(BlendAlphaMode::AlphaMultiply),
					..Default::default()
				},
			),
		})
	}
}

impl State<Box<dyn Error>> for MainState {
	fn draw(&mut self, ctx: &mut Context) -> Result<(), Box<dyn Error>> {
		self.mesh.draw(
			ctx,
			DrawParams::new()
				.color(Rgba::rgb(0.5, 0.0, 0.0))
				.graphics_pipeline(Some(self.graphics_pipeline.clone())),
		);
		self.mesh.draw(
			ctx,
			DrawParams::<DefaultShader>::new()
				.position(Vec2::new(25.0, 25.0))
				.color(Rgba::rgb(0.5, 0.0, 0.0))
				.graphics_pipeline(Some(self.graphics_pipeline.clone())),
		);
		Ok(())
	}
}

fn main() -> Result<(), Box<dyn Error>> {
	micro::run(ContextSettings::default(), MainState::new)
}
