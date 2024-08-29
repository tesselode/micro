use std::fmt::Debug;

use glam::Vec2;
use palette::LinSrgba;

use crate::{
	graphics::mesh::{Mesh, ShapeStyle},
	with_child_fns, with_sizing_fns, Context,
};

use super::{LayoutResult, Sizing, Widget, WidgetMouseEventChannel};

#[derive(Debug)]
pub struct Ellipse {
	sizing: Sizing,
	fill: Option<LinSrgba>,
	stroke: Option<(f32, LinSrgba)>,
	children: Vec<Box<dyn Widget>>,
	mouse_event_channel: Option<WidgetMouseEventChannel>,
}

impl Ellipse {
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

impl Default for Ellipse {
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

impl Widget for Ellipse {
	fn name(&self) -> &'static str {
		"ellipse"
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
		self.sizing
			.allotted_size_for_children(allotted_size_from_parent)
	}

	fn layout(
		&self,
		_ctx: &mut Context,
		allotted_size_from_parent: Vec2,
		child_sizes: &[Vec2],
	) -> LayoutResult {
		LayoutResult {
			size: self
				.sizing
				.final_parent_size(allotted_size_from_parent, child_sizes.iter().copied()),
			child_positions: std::iter::repeat(Vec2::ZERO)
				.take(child_sizes.len())
				.collect(),
		}
	}

	fn draw(&self, ctx: &mut Context, size: Vec2) -> anyhow::Result<()> {
		if let Some(fill) = self.fill {
			Mesh::ellipse(ctx, ShapeStyle::Fill, size / 2.0, size / 2.0, 0.0)?
				.color(fill)
				.draw(ctx);
		}
		if let Some((width, color)) = self.stroke {
			Mesh::ellipse(ctx, ShapeStyle::Stroke(width), size / 2.0, size / 2.0, 0.0)?
				.color(color)
				.draw(ctx);
		}
		Ok(())
	}
}
