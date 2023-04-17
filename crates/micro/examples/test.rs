use std::error::Error;

use bytemuck::{Pod, Zeroable};
use glam::{UVec2, Vec2};
use micro::{
	graphics::{
		graphics_pipeline::{GraphicsPipeline, GraphicsPipelineSettings},
		shader::Shader,
		text::{Font, FontSettings, LayoutSettings, Text},
		DrawParams,
	},
	window::WindowMode,
	Context, ContextSettings, State,
};

struct WavyShader;

#[repr(C)]
#[derive(Clone, Copy, Zeroable, Pod)]
struct WavyShaderParams {
	amplitude: f32,
	frequency: f32,
}

impl Shader for WavyShader {
	const SOURCE: &'static str = include_str!("wavy.wgsl");

	type Params = WavyShaderParams;
}

struct MainState {
	font: Font,
	text: Text,
	graphics_pipeline: GraphicsPipeline,
}

impl MainState {
	fn new(ctx: &mut Context) -> Result<Self, Box<dyn Error>> {
		let font = Font::from_file(
			ctx,
			"crates/micro/examples/Roboto-Regular.ttf",
			FontSettings {
				scale: 50.0,
				..Default::default()
			},
		)?;
		let text = Text::new(ctx, &font, "Hello, world!", LayoutSettings::default());
		let graphics_pipeline = GraphicsPipeline::new(
			ctx,
			GraphicsPipelineSettings::<WavyShader> {
				shader_params: WavyShaderParams {
					amplitude: 0.005,
					frequency: 100.0,
				},
			},
		);
		Ok(Self {
			font,
			text,
			graphics_pipeline,
		})
	}
}

impl State<Box<dyn Error>> for MainState {
	fn draw(&mut self, ctx: &mut Context) -> Result<(), Box<dyn Error>> {
		self.text.draw(
			ctx,
			DrawParams::new()
				.graphics_pipeline(self.graphics_pipeline.clone())
				.position(Vec2::new(50.0, 50.0)),
		);
		Ok(())
	}
}

fn main() -> Result<(), Box<dyn Error>> {
	micro::run(
		ContextSettings {
			window_mode: WindowMode::Windowed {
				size: UVec2::new(1280, 720),
			},
			..Default::default()
		},
		MainState::new,
	)
}
