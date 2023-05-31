use glam::Vec2;
use micro::{
	graphics::{
		canvas::{Canvas, CanvasSettings, RenderToCanvasSettings},
		color::Rgba,
		graphics_pipeline::{GraphicsPipeline, GraphicsPipelineSettings},
		mesh::{Mesh, MeshTexture, ShapeStyle},
		shader::{DefaultShader, Shader},
		texture::{Texture, TextureSettings},
		DrawParams,
	},
	Context, ContextSettings, State,
};

#[derive(Clone)]
pub struct MultiplyShader;

impl Shader for MultiplyShader {
	const SOURCE: &'static str = include_str!("multiply.wgsl");

	const NUM_TEXTURES: u32 = 1;

	type Params = i32;
}

struct MainState {
	texture: Texture,
	multiply_graphics_pipeline: GraphicsPipeline<MultiplyShader>,
}

impl MainState {
	fn new(ctx: &mut Context) -> Self {
		let texture = Texture::from_file(
			ctx,
			"crates/micro/examples/tree.png",
			TextureSettings::default(),
		)
		.unwrap();
		let canvas_graphics_pipeline = GraphicsPipeline::<DefaultShader>::new(
			ctx,
			GraphicsPipelineSettings {
				sample_count: 8,
				..Default::default()
			},
		);
		let canvas = Canvas::new(
			ctx,
			ctx.window_size(),
			CanvasSettings {
				sample_count: 8,
				..Default::default()
			},
		);
		canvas.render_to(
			ctx,
			RenderToCanvasSettings {
				clear_color: Some(Rgba::BLACK),
				clear_stencil_value: None,
			},
			|ctx| {
				Mesh::circle(
					ctx,
					ShapeStyle::Fill,
					Vec2::splat(200.0),
					500.0,
					Rgba::WHITE,
				)
				.draw(ctx, &canvas_graphics_pipeline);
			},
		);
		let multiply_graphics_pipeline = GraphicsPipeline::new(
			ctx,
			GraphicsPipelineSettings {
				textures: vec![MeshTexture::Canvas(canvas.clone())],
				..Default::default()
			},
		);
		Self {
			texture,
			multiply_graphics_pipeline,
		}
	}
}

impl State for MainState {
	fn draw(&mut self, ctx: &mut Context) {
		self.texture.draw(ctx, &self.multiply_graphics_pipeline);
	}
}

fn main() {
	micro::run(ContextSettings::default(), MainState::new)
}
