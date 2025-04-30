use std::error::Error;

use bytemuck::{Pod, Zeroable};
use glam::{Vec2, vec2};
use micro::{
	App, Context, ContextSettings,
	graphics::{
		Canvas, CanvasSettings, GraphicsPipeline, GraphicsPipelineBuilder, RenderToCanvasSettings,
		Shader, Vertex2d,
		mesh::{Mesh, ShapeStyle},
	},
	math::Circle,
};
use wgpu::{ShaderModuleDescriptor, TextureFormat, include_wgsl};

fn main() -> Result<(), Box<dyn Error>> {
	micro::run(ContextSettings::default(), Test::new)
}

struct Test {
	canvas: Canvas,
	graphics_pipeline: GraphicsPipeline<TestShader>,
}

impl Test {
	fn new(ctx: &mut Context) -> Result<Self, Box<dyn Error>> {
		let canvas = Canvas::new(
			ctx,
			ctx.window_size(),
			CanvasSettings {
				sample_count: 4,
				..Default::default()
			},
		);
		let graphics_pipeline = GraphicsPipelineBuilder::for_canvas(&canvas)
			.with_storage_buffer(&[
				Instance {
					translation: vec2(50.0, 50.0),
				},
				Instance {
					translation: vec2(100.0, 100.0),
				},
			])
			.sample_count(4)
			.build(ctx);
		Ok(Self {
			canvas,
			graphics_pipeline,
		})
	}
}

impl App for Test {
	type Error = Box<dyn Error>;

	fn draw(&mut self, ctx: &mut Context) -> Result<(), Self::Error> {
		/* let ctx = &mut self
		.canvas
		.render_to(ctx, RenderToCanvasSettings::default()); */
		let mesh = Mesh::circle(ctx, ShapeStyle::Fill, Circle::around_zero(10.0))?;
		self.graphics_pipeline.draw_instanced(ctx, 2, &mesh);
		Ok(())
	}
}

struct TestShader;

impl Shader for TestShader {
	const DESCRIPTOR: ShaderModuleDescriptor<'_> = include_wgsl!("shader.wgsl");
	const NUM_STORAGE_BUFFERS: u32 = 1;

	type Vertex = Vertex2d;
	type Params = i32;
}

#[derive(Debug, Clone, Copy, PartialEq, Pod, Zeroable, Default)]
#[repr(C)]
struct Instance {
	translation: Vec2,
}
