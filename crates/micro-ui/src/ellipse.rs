use std::fmt::Debug;

use micro::{
	color::LinSrgba,
	graphics::mesh::{Mesh, ShapeStyle},
	math::Vec2,
};

use crate::{child_fns, sizing_fns};

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

	pub fn fill(self, color: impl Into<LinSrgba>) -> Self {
		Self {
			fill: Some(color.into()),
			..self
		}
	}

	pub fn stroke(self, width: f32, color: impl Into<LinSrgba>) -> Self {
		Self {
			stroke: Some((width, color.into())),
			..self
		}
	}

	pub fn mouse_event_channel(self, channel: &WidgetMouseEventChannel) -> Self {
		Self {
			mouse_event_channel: Some(channel.clone()),
			..self
		}
	}

	child_fns!();
	sizing_fns!();
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
		let _span = tracy_client::span!();
		self.sizing
			.allotted_size_for_children(allotted_size_from_parent)
	}

	fn layout(&self, allotted_size_from_parent: Vec2, child_sizes: &[Vec2]) -> LayoutResult {
		let _span = tracy_client::span!();
		LayoutResult {
			size: self
				.sizing
				.final_parent_size(allotted_size_from_parent, child_sizes.iter().copied()),
			child_positions: std::iter::repeat_n(Vec2::ZERO, child_sizes.len()).collect(),
		}
	}

	fn draw_before_children(&self, size: Vec2) {
		let _span = tracy_client::span!();
		if let Some(fill) = self.fill {
			Mesh::ellipse(ShapeStyle::Fill, size / 2.0, size / 2.0, 0.0)
				.color(fill)
				.draw()
		}
	}

	fn draw_after_children(&self, size: Vec2) {
		let _span = tracy_client::span!();
		if let Some((width, color)) = self.stroke {
			Mesh::ellipse(ShapeStyle::Stroke(width), size / 2.0, size / 2.0, 0.0)
				.color(color)
				.draw();
		}
	}
}
