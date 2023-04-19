use std::error::Error;

use bytemuck::{Pod, Zeroable};
use glam::{UVec2, Vec2};
use micro::{
	graphics::{
		color::Rgba,
		graphics_pipeline::{GraphicsPipeline, GraphicsPipelineSettings},
		mesh::Mesh,
		shader::Shader,
		stencil::StencilState,
		text::{Font, FontSettings, LayoutSettings, Text},
		DrawParams,
	},
	math::Rect,
	window::WindowMode,
	Context, ContextSettings, State,
};
use wgpu::{CompareFunction, StencilOperation};

#[derive(Clone)]
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
	text: Text,
	mesh: Mesh,
	write_pipeline: GraphicsPipeline,
	read_pipeline: GraphicsPipeline,
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
		let write_pipeline = GraphicsPipeline::new(
			ctx,
			GraphicsPipelineSettings {
				stencil_state: StencilState {
					compare: CompareFunction::Always,
					fail_op: StencilOperation::IncrementClamp,
					pass_op: StencilOperation::IncrementClamp,
					read_mask: 0xff,
					write_mask: 0xff,
				},
				enable_color_writes: false,
				..Default::default()
			},
		);
		let read_pipeline = GraphicsPipeline::new(
			ctx,
			GraphicsPipelineSettings {
				stencil_state: StencilState {
					compare: CompareFunction::Equal,
					fail_op: StencilOperation::Keep,
					pass_op: StencilOperation::Keep,
					read_mask: 0xff,
					write_mask: 0xff,
				},
				..Default::default()
			},
		);
		Ok(Self {
			text,
			mesh: Mesh::rectangle(ctx, Rect::xywh(50.0, 50.0, 100.0, 150.0)),
			write_pipeline,
			read_pipeline,
		})
	}
}

impl State<Box<dyn Error>> for MainState {
	fn draw(&mut self, ctx: &mut Context) -> Result<(), Box<dyn Error>> {
		self.mesh.draw(ctx, &self.write_pipeline);
		self.text.draw(
			ctx,
			DrawParams::new()
				.position(Vec2::new(50.0, 50.0))
				.graphics_pipeline(self.read_pipeline.clone())
				.stencil_reference(1),
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
