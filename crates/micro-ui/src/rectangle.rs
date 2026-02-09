use std::fmt::Debug;

use micro::{
	Context,
	color::LinSrgba,
	graphics::mesh::Mesh,
	math::{Rect, Vec2},
};

use crate::{child_fns, sizing_fns};

use super::{LayoutResult, Sizing, Widget, WidgetMouseState};

#[derive(Debug)]
pub struct Rectangle {
	sizing: Sizing,
	fill: Option<LinSrgba>,
	stroke: Option<(f32, LinSrgba)>,
	children: Vec<Box<dyn Widget>>,
	mouse_state: Option<WidgetMouseState>,
}

impl Rectangle {
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

	pub fn fill_if(self, condition: bool, color: impl Into<LinSrgba>) -> Self {
		if condition { self.fill(color) } else { self }
	}

	pub fn stroke_if(self, condition: bool, width: f32, color: impl Into<LinSrgba>) -> Self {
		if condition {
			self.stroke(width, color)
		} else {
			self
		}
	}

	pub fn mouse_state(self, state: &WidgetMouseState) -> Self {
		Self {
			mouse_state: Some(state.clone()),
			..self
		}
	}

	child_fns!();
	sizing_fns!();
}

impl Default for Rectangle {
	fn default() -> Self {
		Self {
			sizing: Sizing::EXPAND,
			fill: Default::default(),
			stroke: Default::default(),
			children: Default::default(),
			mouse_state: None,
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

	fn mouse_state(&self) -> Option<WidgetMouseState> {
		self.mouse_state.clone()
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
			child_positions: std::iter::repeat_n(Vec2::ZERO, child_sizes.len()).collect(),
		}
	}

	fn draw_before_children(&self, ctx: &mut Context, size: Vec2) {
		let _span = tracy_client::span!();
		if let Some(fill) = self.fill {
			Mesh::rectangle(ctx, Rect::new(Vec2::ZERO, size))
				.color(fill)
				.draw(ctx);
		}
	}

	fn draw_after_children(&self, ctx: &mut Context, size: Vec2) {
		let _span = tracy_client::span!();
		if let Some((width, color)) = self.stroke {
			Mesh::outlined_rectangle(ctx, width, Rect::new(Vec2::ZERO, size))
				.color(color)
				.draw(ctx);
		}
	}
}
