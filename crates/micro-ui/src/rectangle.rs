use std::fmt::Debug;

use micro::{
	Context,
	color::LinSrgba,
	graphics::mesh::Mesh,
	math::{Rect, Vec2},
};

use crate::{
	WidgetInspector, WidgetState, child_functions, common_functions, common_widget_trait_functions,
	sizing_functions,
};

use super::{LayoutResult, Sizing, Widget};

#[derive(Debug)]
pub struct Rectangle {
	id: Option<String>,
	inspector: Option<WidgetInspector>,
	sizing: Sizing,
	fill: Option<LinSrgba>,
	stroke: Option<(f32, LinSrgba)>,
	children: Vec<Box<dyn Widget>>,
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

	common_functions!();
	child_functions!();
	sizing_functions!();
}

impl Default for Rectangle {
	fn default() -> Self {
		Self {
			id: None,
			inspector: None,
			sizing: Sizing::EXPAND,
			fill: Default::default(),
			stroke: Default::default(),
			children: Default::default(),
		}
	}
}

impl Widget for Rectangle {
	common_widget_trait_functions!();

	fn name(&self) -> &'static str {
		"rectangle"
	}

	fn children(&mut self, _ctx: &mut Context, _state: &mut WidgetState) -> Vec<Box<dyn Widget>> {
		self.children.drain(..).collect()
	}

	fn allotted_size_for_next_child(
		&mut self,
		_ctx: &mut Context,
		allotted_size_from_parent: Vec2,
		_previous_child_sizes: &[Vec2],
		_state: &mut WidgetState,
	) -> Vec2 {
		let _span = tracy_client::span!();
		self.sizing
			.allotted_size_for_children(allotted_size_from_parent)
	}

	fn layout(
		&mut self,
		_ctx: &mut Context,
		allotted_size_from_parent: Vec2,
		child_sizes: &[Vec2],
		_state: &mut WidgetState,
	) -> LayoutResult {
		let _span = tracy_client::span!();
		LayoutResult {
			size: self
				.sizing
				.final_parent_size(allotted_size_from_parent, child_sizes.iter().copied()),
			child_positions: std::iter::repeat_n(Vec2::ZERO, child_sizes.len()).collect(),
		}
	}

	fn draw_before_children(&mut self, ctx: &mut Context, size: Vec2, _state: &mut WidgetState) {
		let _span = tracy_client::span!();
		if let Some(fill) = self.fill {
			Mesh::rectangle(ctx, Rect::new(Vec2::ZERO, size))
				.color(fill)
				.draw(ctx);
		}
	}

	fn draw_after_children(&mut self, ctx: &mut Context, size: Vec2, _state: &mut WidgetState) {
		let _span = tracy_client::span!();
		if let Some((width, color)) = self.stroke {
			Mesh::outlined_rectangle(ctx, width, Rect::new(Vec2::ZERO, size))
				.color(color)
				.draw(ctx);
		}
	}
}
