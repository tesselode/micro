use std::error::Error;

use glam::vec2;
use micro_wgpu::{
	App, Context, ContextSettings,
	graphics::{
		canvas::{Canvas, CanvasSettings, RenderToCanvasSettings},
		graphics_pipeline::{GraphicsPipeline, GraphicsPipelineSettings},
		mesh::{Mesh, builder::ShapeStyle},
		text::{Font, FontSettings, LayoutSettings, Text},
	},
	math::Circle,
};
use wgpu::{CompareFunction, StencilFaceState, StencilOperation, StencilState};

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
	text: Text,
}

impl Test {
	fn new(ctx: &mut Context) -> Result<Self, Box<dyn Error>> {
		let font = Font::from_file(
			ctx,
			"resources/NotoSans-Regular.ttf",
			FontSettings::default(),
		)?;
		Ok(Self {
			text: Text::new(ctx, &font, "Hello, world!", LayoutSettings::default()),
		})
	}
}

impl App for Test {
	type Error = Box<dyn Error>;

	fn draw(&mut self, ctx: &mut Context) -> Result<(), Self::Error> {
		self.text.range(0..5).draw(ctx);

		Ok(())
	}
}
