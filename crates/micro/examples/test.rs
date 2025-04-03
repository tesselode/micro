use std::error::Error;

use bytemuck::{Pod, Zeroable};
use glam::{Vec2, vec2};
use micro::{
	App, Context, ContextSettings,
	graphics::{
		GraphicsPipeline, GraphicsPipelineBuilder, HasVertexAttributes, Instanced, Shader,
		Vertex2d,
		mesh::{Mesh, ShapeStyle},
	},
	math::Circle,
};
use wgpu::{ShaderModuleDescriptor, VertexAttribute, include_wgsl, vertex_attr_array};

fn main() -> Result<(), Box<dyn Error>> {
	micro::run(ContextSettings::default(), Test::new)
}

struct Test {
	graphics_pipeline: GraphicsPipeline<InstancedShader>,
}

impl Test {
	fn new(ctx: &mut Context) -> Result<Self, Box<dyn Error>> {
		Ok(Self {
			graphics_pipeline: GraphicsPipelineBuilder::new(ctx).build(ctx),
		})
	}
}

impl App for Test {
	type Error = Box<dyn Error>;

	fn draw(&mut self, ctx: &mut Context) -> Result<(), Self::Error> {
		self.graphics_pipeline.draw_instanced(
			ctx,
			Mesh::circle(ctx, ShapeStyle::Fill, Circle::around_zero(10.0))?,
			&[
				Instance {
					translation: vec2(50.0, 50.0),
				},
				Instance {
					translation: vec2(100.0, 100.0),
				},
				Instance {
					translation: vec2(200.0, 200.0),
				},
			],
		);
		Ok(())
	}
}

struct InstancedShader;

impl Shader for InstancedShader {
	const DESCRIPTOR: ShaderModuleDescriptor<'_> = include_wgsl!("shader.wgsl");

	type Kind = Instanced<Instance>;

	type Vertex = Vertex2d;

	type Params = i32;
}

#[derive(Debug, Clone, Copy, PartialEq, Pod, Zeroable)]
#[repr(C)]
struct Instance {
	translation: Vec2,
}

impl HasVertexAttributes for Instance {
	fn attributes() -> Vec<VertexAttribute> {
		vertex_attr_array![3 => Float32x2].into()
	}
}
