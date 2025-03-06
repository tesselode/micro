use std::error::Error;

use micro::{
	App, Context, ContextSettings,
	color::{ColorConstants, LinSrgb},
	graphics::{
		CompareFunction, StencilOperation, StencilState, graphics_pipeline::GraphicsPipeline,
	},
};
use micro_ui::{Ellipse, GraphicsPipelineWidget, Padding, Rectangle, StencilReferenceWidget, Ui};

fn main() -> Result<(), Box<dyn Error>> {
	micro::run(ContextSettings::default(), Test::new)
}

struct Test {
	ui: Ui,
	write_stencil_pipeline: GraphicsPipeline,
	read_stencil_pipeline: GraphicsPipeline,
}

impl Test {
	pub fn new(ctx: &mut Context) -> Result<Self, Box<dyn Error>> {
		Ok(Self {
			ui: Ui::new(),
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
		self.ui.render(
			ctx,
			ctx.window_size().as_vec2(),
			Padding::new(100.0, 100.0, 0.0, 0.0).with_child(
				StencilReferenceWidget::new(1)
					.with_child(
						GraphicsPipelineWidget::new(&self.write_stencil_pipeline).with_child(
							Rectangle::new()
								.with_fill(LinSrgb::RED)
								.with_max_size((100.0, 50.0)),
						),
					)
					.with_child(
						GraphicsPipelineWidget::new(&self.read_stencil_pipeline).with_child(
							Ellipse::new()
								.with_fill(LinSrgb::BLUE)
								.with_max_size((75.0, 75.0)),
						),
					),
			),
		)?;
		Ok(())
	}
}
