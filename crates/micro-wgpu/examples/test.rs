use std::error::Error;

use bytemuck::{Pod, Zeroable};
use glam::vec2;
use micro_wgpu::{
	App, Context, ContextSettings, Event,
	color::ColorConstants,
	graphics::{
		Shader, Vertex2d,
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
	graphics_pipeline: GraphicsPipeline<WigglyShader>,
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
					shader_params: WigglyShaderParams { wiggliness: 10.0 },
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
			self.graphics_pipeline
				.set_shader_params(ctx, WigglyShaderParams { wiggliness: 20.0 });
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

struct WigglyShader;

impl Shader for WigglyShader {
	const SOURCE: &'static str = include_str!("wiggly.wgsl");

	type Vertex = Vertex2d;

	type Params = WigglyShaderParams;
}

#[derive(Debug, Clone, Copy, PartialEq, Pod, Zeroable, Default)]
#[repr(C)]
struct WigglyShaderParams {
	wiggliness: f32,
}
