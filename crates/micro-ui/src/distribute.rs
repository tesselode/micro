use micro::{
	Context,
	math::{Vec2, vec2},
};

use crate::{
	AxisSizing, LayoutResult, Widget, WidgetInspector, WidgetMouseState, child_functions,
	common_functions, common_widget_trait_functions,
};

#[derive(Debug)]
pub struct Distribute {
	direction: Axis,
	cross_align: f32,
	cross_sizing: AxisSizing,
	children: Vec<Box<dyn Widget>>,
	mouse_state: Option<WidgetMouseState>,
	inspector: Option<WidgetInspector>,
}

impl Distribute {
	pub fn horizontal() -> Self {
		Self {
			direction: Axis::Horizontal,
			cross_align: 0.0,
			cross_sizing: AxisSizing::Shrink,
			children: vec![],
			mouse_state: None,
			inspector: None,
		}
	}

	pub fn vertical() -> Self {
		Self {
			direction: Axis::Vertical,
			cross_align: 0.0,
			cross_sizing: AxisSizing::Shrink,
			children: vec![],
			mouse_state: None,
			inspector: None,
		}
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

	common_functions!();
	child_functions!();
}

impl Widget for Distribute {
	common_widget_trait_functions!();

	fn name(&self) -> &'static str {
		"stack"
	}

	fn children(&self) -> &[Box<dyn Widget>] {
		&self.children
	}

	fn allotted_size_for_next_child(
		&self,
		allotted_size_from_parent: Vec2,
		previous_child_sizes: &[Vec2],
	) -> Vec2 {
		let _span = tracy_client::span!();
		match self.direction {
			Axis::Horizontal => {
				let total_child_width = previous_child_sizes.iter().map(|size| size.x).sum::<f32>();
				vec2(
					allotted_size_from_parent.x - total_child_width,
					self.cross_sizing
						.allotted_size_for_children(allotted_size_from_parent.y),
				)
			}
			Axis::Vertical => {
				let total_child_height =
					previous_child_sizes.iter().map(|size| size.y).sum::<f32>();
				vec2(
					self.cross_sizing
						.allotted_size_for_children(allotted_size_from_parent.x),
					allotted_size_from_parent.y - total_child_height,
				)
			}
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
					allotted_size_from_parent.x,
					self.cross_sizing.final_parent_size(
						allotted_size_from_parent.y,
						child_sizes.iter().map(|size| size.y),
					),
				);
				let total_child_width = child_sizes.iter().map(|size| size.x).sum::<f32>();
				let gap = (allotted_size_from_parent.x - total_child_width)
					/ child_sizes.len().saturating_sub(1).max(1) as f32;
				let mut next_child_x = 0.0;
				let child_positions = child_sizes
					.iter()
					.map(|size| {
						let x = next_child_x;
						next_child_x += size.x + gap;
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
					allotted_size_from_parent.y,
				);
				let total_child_height = child_sizes.iter().map(|size| size.y).sum::<f32>();
				let gap = (allotted_size_from_parent.y - total_child_height)
					/ child_sizes.len().saturating_sub(1).max(1) as f32;
				let mut next_child_y = 0.0;
				let child_positions = child_sizes
					.iter()
					.map(|size| {
						let y = next_child_y;
						next_child_y += size.y + gap;
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
