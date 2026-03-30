use std::fmt::Debug;

use crate::graphics::{Canvas, RenderToCanvasSettings};

use super::DrawCommand;

#[derive(Clone, PartialEq)]
pub(super) struct RenderPass {
	pub kind: RenderPassKind,
	pub draw_commands: Vec<DrawCommand>,
}

impl Debug for RenderPass {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("RenderPass")
			.field("kind", &self.kind)
			.field(
				"draw_commands",
				&format!("draw commands: [{}]", self.draw_commands.len()),
			)
			.finish()
	}
}

#[derive(Clone, PartialEq)]
pub(super) enum RenderPassKind {
	MainSurface,
	Canvas {
		canvas: Canvas,
		settings: RenderToCanvasSettings,
	},
}

impl Debug for RenderPassKind {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::MainSurface => write!(f, "MainSurface"),
			Self::Canvas { canvas, settings } => f
				.debug_struct("Canvas")
				.field("canvas", &format!("Canvas ('{}')", &canvas.label))
				.field("settings", settings)
				.finish(),
		}
	}
}

pub(super) struct CanvasRenderPass {
	pub canvas: Canvas,
	pub settings: RenderToCanvasSettings,
	pub draw_commands: Vec<DrawCommand>,
}
