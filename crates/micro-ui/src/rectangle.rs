use std::fmt::Debug;

use micro::{
	Context,
	color::LinSrgba,
	graphics::{GraphicsPipeline, mesh::Mesh},
	math::{Rect, Vec2},
};

use crate::{with_child_fns, with_sizing_fns};

use super::{LayoutResult, Sizing, Widget, WidgetMouseEventChannel};

#[derive(Debug)]
pub struct Rectangle {
	sizing: Sizing,
	fill: Option<LinSrgba>,
	stroke: Option<(f32, LinSrgba)>,
	children: Vec<Box<dyn Widget>>,
	mouse_event_channel: Option<WidgetMouseEventChannel>,
}

impl Rectangle {
	pub fn new() -> Self {
		Self::default()
	}

	pub fn with_fill(self, color: impl Into<LinSrgba>) -> Self {
		Self {
			fill: Some(color.into()),
			..self
		}
	}

	pub fn with_stroke(self, width: f32, color: impl Into<LinSrgba>) -> Self {
		Self {
			stroke: Some((width, color.into())),
			..self
		}
	}

	pub fn with_mouse_event_channel(self, channel: &WidgetMouseEventChannel) -> Self {
		Self {
			mouse_event_channel: Some(channel.clone()),
			..self
		}
	}

	with_child_fns!();
	with_sizing_fns!();
}

impl Default for Rectangle {
	fn default() -> Self {
		Self {
			sizing: Sizing::EXPAND,
			fill: Default::default(),
			stroke: Default::default(),
			children: Default::default(),
			mouse_event_channel: None,
		}
	}
}

impl Widget for Rectangle {
	fn name(&self) -> &'static str {
		"rectangle"
	}

	fn children(&self) -> &[Box<dyn Widget>] {
		&self.children
	}

	fn mouse_event_channel(&self) -> Option<&WidgetMouseEventChannel> {
		self.mouse_event_channel.as_ref()
	}

	fn allotted_size_for_next_child(
		&self,
		allotted_size_from_parent: Vec2,
		_previous_child_sizes: &[Vec2],
	) -> Vec2 {
		let _span = tracy_client::span!();
		self.sizing
			.allotted_size_for_children(allotted_size_from_parent)
	}

	fn layout(
		&self,
		_ctx: &mut Context,
		allotted_size_from_parent: Vec2,
		child_sizes: &[Vec2],
	) -> LayoutResult {
		let _span = tracy_client::span!();
		LayoutResult {
			size: self
				.sizing
				.final_parent_size(allotted_size_from_parent, child_sizes.iter().copied()),
			child_positions: std::iter::repeat(Vec2::ZERO)
				.take(child_sizes.len())
				.collect(),
		}
	}

	fn draw_before_children(
		&self,
		ctx: &mut Context,
		graphics_pipeline: &GraphicsPipeline,
		size: Vec2,
	) -> anyhow::Result<()> {
		let _span = tracy_client::span!();
		if let Some(fill) = self.fill {
			graphics_pipeline.draw(
				ctx,
				&Mesh::rectangle(ctx, Rect::new(Vec2::ZERO, size)).color(fill),
			);
		}
		Ok(())
	}

	fn draw_after_children(
		&self,
		ctx: &mut Context,
		graphics_pipeline: &GraphicsPipeline,
		size: Vec2,
	) -> anyhow::Result<()> {
		let _span = tracy_client::span!();
		if let Some((width, color)) = self.stroke {
			graphics_pipeline.draw(
				ctx,
				&Mesh::outlined_rectangle(ctx, width, Rect::new(Vec2::ZERO, size))?.color(color),
			);
		}
		Ok(())
	}
}
