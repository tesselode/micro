use std::fmt::Debug;

use micro::{
	Context,
	math::{Vec2, vec2},
};

use crate::child_fns;

use super::{AxisSizing, LayoutResult, Widget, WidgetMouseState};

#[derive(Debug)]
pub struct Stack {
	direction: Axis,
	gap: f32,
	cross_align: f32,
	cross_sizing: AxisSizing,
	children: Vec<Box<dyn Widget>>,
	mouse_state: Option<WidgetMouseState>,
}

impl Stack {
	pub fn horizontal() -> Self {
		Self {
			direction: Axis::Horizontal,
			gap: 0.0,
			cross_align: 0.0,
			cross_sizing: AxisSizing::Shrink,
			children: vec![],
			mouse_state: None,
		}
	}

	pub fn vertical() -> Self {
		Self {
			direction: Axis::Vertical,
			gap: 0.0,
			cross_align: 0.0,
			cross_sizing: AxisSizing::Shrink,
			children: vec![],
			mouse_state: None,
		}
	}

	pub fn gap(self, gap: f32) -> Self {
		Self { gap, ..self }
	}

	pub fn cross_align(self, cross_align: f32) -> Self {
		Self {
			cross_align,
			..self
		}
	}

	pub fn cross_sizing(self, cross_sizing: AxisSizing) -> Self {
		Self {
			cross_sizing,
			..self
		}
	}

	pub fn mouse_state(self, state: &WidgetMouseState) -> Self {
		Self {
			mouse_state: Some(state.clone()),
			..self
		}
	}

	child_fns!();
}

impl Widget for Stack {
	fn name(&self) -> &'static str {
		"stack"
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
		previous_child_sizes: &[Vec2],
	) -> Vec2 {
		let _span = tracy_client::span!();
		let total_child_main_axis_size =
			total_child_main_axis_size(self.direction, previous_child_sizes, self.gap);
		match self.direction {
			Axis::Horizontal => vec2(
				allotted_size_from_parent.x - total_child_main_axis_size,
				self.cross_sizing
					.allotted_size_for_children(allotted_size_from_parent.y),
			),
			Axis::Vertical => vec2(
				self.cross_sizing
					.allotted_size_for_children(allotted_size_from_parent.x),
				allotted_size_from_parent.y - total_child_main_axis_size,
			),
		}
	}

	fn layout(
		&self,
		_ctx: &mut Context,
		allotted_size_from_parent: Vec2,
		child_sizes: &[Vec2],
	) -> LayoutResult {
		let _span = tracy_client::span!();
		match self.direction {
			Axis::Horizontal => {
				let parent_size = vec2(
					total_child_main_axis_size(self.direction, child_sizes, self.gap),
					self.cross_sizing.final_parent_size(
						allotted_size_from_parent.y,
						child_sizes.iter().map(|size| size.y),
					),
				);
				let mut next_child_x = 0.0;
				let child_positions = child_sizes
					.iter()
					.map(|size| {
						let x = next_child_x;
						next_child_x += size.x + self.gap;
						vec2(x, (parent_size.y - size.y) * self.cross_align)
					})
					.collect();
				LayoutResult {
					size: parent_size,
					child_positions,
				}
			}
			Axis::Vertical => {
				let parent_size = vec2(
					self.cross_sizing.final_parent_size(
						allotted_size_from_parent.x,
						child_sizes.iter().map(|size| size.x),
					),
					total_child_main_axis_size(self.direction, child_sizes, self.gap),
				);
				let mut next_child_y = 0.0;
				let child_positions = child_sizes
					.iter()
					.map(|size| {
						let y = next_child_y;
						next_child_y += size.y + self.gap;
						vec2((parent_size.x - size.x) * self.cross_align, y)
					})
					.collect();
				LayoutResult {
					size: parent_size,
					child_positions,
				}
			}
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Axis {
	Horizontal,
	Vertical,
}

fn total_child_main_axis_size(direction: Axis, child_sizes: &[Vec2], gap: f32) -> f32 {
	let num_gaps = child_sizes.len().saturating_sub(1);
	let total_gap_size = num_gaps as f32 * gap;
	child_sizes
		.iter()
		.map(|size| match direction {
			Axis::Horizontal => size.x,
			Axis::Vertical => size.y,
		})
		.sum::<f32>()
		+ total_gap_size
}
