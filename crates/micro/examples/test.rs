use std::error::Error;

use bytemuck::{Pod, Zeroable};
use glam::{Vec2, vec2};
use micro::{
	App, Context, ContextSettings,
	graphics::{
		GraphicsPipeline, GraphicsPipelineBuilder, Shader, Vertex2d,
		mesh::{Mesh, ShapeStyle},
	},
	math::Circle,
};
use wgpu::{ShaderModuleDescriptor, include_wgsl};

fn main() -> Result<(), Box<dyn Error>> {
	micro::run(ContextSettings::default(), Test::new)
}

struct Test {
	graphics_pipeline: GraphicsPipeline<TestShader>,
}

impl Test {
	fn new(ctx: &mut Context) -> Result<Self, Box<dyn Error>> {
		Ok(Self {
			graphics_pipeline: GraphicsPipelineBuilder::new(ctx)
				.with_storage_buffer(&[
					Instance {
						translation: vec2(50.0, 50.0),
					},
					Instance {
						translation: vec2(100.0, 100.0),
					},
				])
				.build(ctx),
		})
	}
}

impl App for Test {
	type Error = Box<dyn Error>;

	fn draw(&mut self, ctx: &mut Context) -> Result<(), Self::Error> {
		self.graphics_pipeline.draw_instanced(
			ctx,
			2,
			&Mesh::circle(ctx, ShapeStyle::Fill, Circle::around_zero(10.0))?,
		);
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
