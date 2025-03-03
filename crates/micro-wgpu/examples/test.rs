use std::error::Error;

use bytemuck::{Pod, Zeroable};
use glam::{Vec2, vec2};
use micro_wgpu::{
	App, Context, ContextSettings, Event,
	graphics::{
		HasVertexAttributes, InstanceBuffer, Shader, Vertex2d,
		graphics_pipeline::GraphicsPipeline,
		mesh::{Mesh, builder::ShapeStyle},
	},
	input::Scancode,
	math::Circle,
};
use wgpu::{PresentMode, VertexAttribute, vertex_attr_array};

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
	graphics_pipeline: GraphicsPipeline<InstancedShader>,
	instance_buffer: InstanceBuffer,
}

impl Test {
	fn new(ctx: &mut Context) -> Result<Self, Box<dyn Error>> {
		Ok(Self {
			graphics_pipeline: GraphicsPipeline::builder()
				.with_instance_buffer::<InstanceInfo>()
				.build(ctx),
			instance_buffer: InstanceBuffer::new(
				ctx,
				&[
					InstanceInfo {
						translation: Vec2::ZERO,
					},
					InstanceInfo {
						translation: vec2(100.0, 0.0),
					},
					InstanceInfo {
						translation: vec2(200.0, 0.0),
					},
				],
			),
		})
	}
}

impl App for Test {
	type Error = Box<dyn Error>;

	fn draw(&mut self, ctx: &mut Context) -> Result<(), Self::Error> {
		let ctx = &mut ctx.push_graphics_pipeline(&self.graphics_pipeline);
		Mesh::circle(ctx, ShapeStyle::Fill, Circle::new(vec2(50.0, 50.0), 50.0))?.draw_instanced(
			ctx,
			3,
			vec![self.instance_buffer.clone()],
		);
		Ok(())
	}
}

pub struct InstancedShader;

impl Shader for InstancedShader {
	const SOURCE: &'static str = include_str!("shader.wgsl");

	type Vertex = Vertex2d;

	type Params = i32;
}

#[derive(Debug, Clone, Copy, PartialEq, Pod, Zeroable)]
#[repr(C)]
pub struct InstanceInfo {
	translation: Vec2,
}

impl HasVertexAttributes for InstanceInfo {
	fn attributes() -> Vec<VertexAttribute> {
		vertex_attr_array![3 => Float32x2].into()
	}
}
