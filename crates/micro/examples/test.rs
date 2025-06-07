use std::error::Error;

use bytemuck::{Pod, Zeroable};
use glam::{Vec2, vec2};
use micro::{
	App, Context, ContextSettings,
	graphics::{
		Canvas, CanvasSettings, GraphicsPipeline, GraphicsPipelineBuilder, RenderToCanvasSettings,
		Shader, Vertex2d,
		mesh::{Mesh, ShapeStyle},
		texture::{Texture, TextureSettings},
	},
	math::Circle,
};
use wgpu::{ShaderModuleDescriptor, TextureFormat, include_wgsl};

fn main() -> Result<(), Box<dyn Error>> {
	micro::run(ContextSettings::default(), Test::new)
}

struct Test {
	texture: Texture,
}

impl Test {
	fn new(ctx: &mut Context) -> Result<Self, Box<dyn Error>> {
		Ok(Self {
			texture: Texture::from_file(
				ctx,
				"resources/spritesheet_default.png",
				TextureSettings::default(),
			)?,
		})
	}
}

impl App for Test {
	type Error = Box<dyn Error>;

	fn draw(&mut self, ctx: &mut Context) -> Result<(), Self::Error> {
		self.texture.draw(ctx);
		Ok(())
	}
}
