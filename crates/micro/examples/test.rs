use std::error::Error;

use glam::vec2;
use micro::{
	App, Context, ContextSettings, Event,
	graphics::{
		GraphicsPipeline, GraphicsPipelineBuilder, Shader, Vertex2d,
		mesh::{Mesh, ShapeStyle},
		texture::{Texture, TextureSettings},
	},
	input::Scancode,
	math::{Circle, Rect},
};
use wgpu::{ShaderModuleDescriptor, include_wgsl};

fn main() -> Result<(), Box<dyn Error>> {
	micro::run(
		ContextSettings {
			max_queued_frames: 1,
			..Default::default()
		},
		Test::new,
	)
}

struct Test {
	graphics_pipeline: GraphicsPipeline<TestShader>,
	texture1: Texture,
	texture2: Texture,
}

impl Test {
	fn new(ctx: &mut Context) -> Result<Self, Box<dyn Error>> {
		let texture1 = Texture::from_file(
			ctx,
			"resources/spritesheet_default.png",
			TextureSettings::default(),
		)?;
		Ok(Self {
			graphics_pipeline: GraphicsPipelineBuilder::new(ctx).build(ctx),
			texture1,
			texture2: Texture::from_file(ctx, "resources/water.png", TextureSettings::default())?,
		})
	}
}

impl App for Test {
	type Error = Box<dyn Error>;

	fn event(&mut self, ctx: &mut Context, event: Event) -> Result<(), Self::Error> {
		if let Event::KeyPressed {
			key: Scancode::Space,
			..
		} = event
		{
			self.graphics_pipeline.set_texture(ctx, 0, &self.texture2);
		}
		Ok(())
	}

	fn draw(&mut self, ctx: &mut Context) -> Result<(), Self::Error> {
		self.graphics_pipeline.draw(
			ctx,
			&Mesh::rectangle(
				ctx,
				Rect {
					top_left: ctx.mouse_position().as_vec2(),
					size: vec2(100.0, 150.0),
				},
			),
		);
		Ok(())
	}
}

struct TestShader;

impl Shader for TestShader {
	const DESCRIPTOR: ShaderModuleDescriptor<'_> = include_wgsl!("shader.wgsl");
	const NUM_TEXTURES: u32 = 1;

	type Vertex = Vertex2d;

	type Params = i32;
}
