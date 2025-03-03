use std::error::Error;

use micro_wgpu::{
	App, Context, ContextSettings,
	graphics::{
		StencilState,
		graphics_pipeline::GraphicsPipeline,
		mesh::{Mesh, builder::ShapeStyle},
	},
	math::Circle,
};
use wgpu::{CompareFunction, StencilOperation};

fn main() -> Result<(), Box<dyn Error>> {
	micro_wgpu::run(ContextSettings::default(), Test::new)
}

struct Test {
	mesh: Mesh,
	write_stencil_pipeline: GraphicsPipeline,
	read_stencil_pipeline: GraphicsPipeline,
}

impl Test {
	pub fn new(ctx: &mut Context) -> Result<Self, Box<dyn Error>> {
		Ok(Self {
			mesh: Mesh::circle(ctx, ShapeStyle::Fill, Circle::around_zero(100.0))?,
			write_stencil_pipeline: GraphicsPipeline::builder()
				.stencil_state(StencilState {
					compare: CompareFunction::Always,
					on_fail: StencilOperation::Replace,
					on_depth_fail: StencilOperation::Replace,
					on_pass: StencilOperation::Replace,
					read_mask: 255,
					write_mask: 255,
				})
				.enable_color_writes(false)
				.build(ctx),
			read_stencil_pipeline: GraphicsPipeline::builder()
				.stencil_state(StencilState {
					compare: CompareFunction::Equal,
					on_fail: StencilOperation::Keep,
					on_depth_fail: StencilOperation::Keep,
					on_pass: StencilOperation::Keep,
					read_mask: 255,
					write_mask: 255,
				})
				.build(ctx),
		})
	}
}

impl App for Test {
	type Error = Box<dyn Error>;

	fn draw(&mut self, ctx: &mut Context) -> Result<(), Self::Error> {
		let ctx = &mut ctx.push_stencil_reference(1);
		{
			let ctx = &mut ctx.push_graphics_pipeline(&self.write_stencil_pipeline);
			self.mesh.translated_2d((200.0, 200.0)).draw(ctx);
		}
		{
			let ctx = &mut ctx.push_graphics_pipeline(&self.read_stencil_pipeline);
			self.mesh.translated_2d((300.0, 200.0)).draw(ctx);
		}
		Ok(())
	}
}
