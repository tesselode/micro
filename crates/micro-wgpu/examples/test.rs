use std::error::Error;

use glam::vec2;
use micro_wgpu::{
	App, Context, ContextSettings,
	graphics::{
		canvas::{Canvas, CanvasSettings, RenderToCanvasSettings},
		graphics_pipeline::{GraphicsPipeline, GraphicsPipelineSettings},
		mesh::{Mesh, builder::ShapeStyle},
	},
	math::Circle,
};
use wgpu::{CompareFunction, StencilFaceState, StencilOperation, StencilState};

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
	write_stencil: GraphicsPipeline,
	read_stencil: GraphicsPipeline,
}

impl Test {
	fn new(ctx: &mut Context) -> Result<Self, Box<dyn Error>> {
		Ok(Self {
			canvas: Canvas::new(
				ctx,
				ctx.window_size(),
				CanvasSettings {
					sample_count: 8,
					..Default::default()
				},
			),
			write_stencil: GraphicsPipeline::new(
				ctx,
				GraphicsPipelineSettings {
					stencil_state: StencilState {
						front: StencilFaceState {
							compare: CompareFunction::Always,
							pass_op: StencilOperation::Replace,
							..Default::default()
						},
						back: StencilFaceState {
							compare: CompareFunction::Always,
							pass_op: StencilOperation::Replace,
							..Default::default()
						},
						read_mask: 255,
						write_mask: 255,
					},
					enable_color_writes: false,
					sample_count: 8,
					..Default::default()
				},
			),
			read_stencil: GraphicsPipeline::new(
				ctx,
				GraphicsPipelineSettings {
					stencil_state: StencilState {
						front: StencilFaceState {
							compare: CompareFunction::Equal,
							..Default::default()
						},
						back: StencilFaceState {
							compare: CompareFunction::Equal,
							..Default::default()
						},
						read_mask: 255,
						write_mask: 255,
					},
					sample_count: 8,
					..Default::default()
				},
			),
		})
	}
}

impl App for Test {
	type Error = Box<dyn Error>;

	fn draw(&mut self, ctx: &mut Context) -> Result<(), Self::Error> {
		let mesh = Mesh::circle(
			ctx,
			ShapeStyle::Fill,
			Circle {
				center: vec2(100.0, 100.0),
				radius: 50.0,
			},
		)?;

		{
			let ctx = &mut self
				.canvas
				.render_to(ctx, RenderToCanvasSettings::default());

			{
				let ctx = &mut ctx.push_graphics_pipeline(&self.write_stencil);
				mesh.stencil_reference(1).draw(ctx);
			}

			{
				let ctx = &mut ctx.push_graphics_pipeline(&self.read_stencil);
				mesh.stencil_reference(1).translated_x(50.0).draw(ctx);
			}
		}

		self.canvas.draw(ctx);

		Ok(())
	}
}
