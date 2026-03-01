use micro::{
	Context,
	math::{Mat4, Vec2},
};

use crate::{
	WidgetInspector, child_functions, common_functions, common_widget_trait_functions,
	sizing_functions,
};

use super::{LayoutResult, Sizing, Widget, WidgetMouseState};

/// Transforms drawing of the children based on the size of the parent.
#[derive(Debug)]
pub struct DynamicTransform {
	sizing: Sizing,
	transform: fn(Vec2) -> Mat4,
	children: Vec<Box<dyn Widget>>,
	mouse_state: Option<WidgetMouseState>,
	inspector: Option<WidgetInspector>,
}

impl DynamicTransform {
	pub fn new(transform: fn(Vec2) -> Mat4) -> Self {
		Self {
			sizing: Sizing::SHRINK,
			transform,
			children: vec![],
			mouse_state: None,
			inspector: None,
		}
	}

	common_functions!();
	child_functions!();
	sizing_functions!();
}

impl Widget for DynamicTransform {
	common_widget_trait_functions!();

	fn name(&self) -> &'static str {
		"dynamic transform"
	}

	fn children(&self) -> &[Box<dyn Widget>] {
		&self.children
	}

	fn transform(&self, size: Vec2) -> Mat4 {
		(self.transform)(size)
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
}
