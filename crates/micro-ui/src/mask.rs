use micro::{Context, math::Vec2};

use crate::{child_fns, sizing_fns};

use super::{LayoutResult, Sizing, Widget, WidgetMouseState};

#[derive(Debug)]
pub struct Mask {
	sizing: Sizing,
	children: Vec<Box<dyn Widget>>,
	mask: Box<dyn Widget>,
	mouse_state: Option<WidgetMouseState>,
}

impl Mask {
	pub fn new(mask: impl Widget + 'static) -> Self {
		Self {
			sizing: Sizing::SHRINK,
			children: vec![],
			mask: Box::new(mask),
			mouse_state: None,
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

impl Widget for Mask {
	fn name(&self) -> &'static str {
		"mask"
	}

	fn children(&self) -> &[Box<dyn Widget>] {
		&self.children
	}

	fn mask(&self) -> Option<&dyn Widget> {
		Some(self.mask.as_ref())
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
	) -> super::LayoutResult {
		LayoutResult {
			size: self
				.sizing
				.final_parent_size(allotted_size_from_parent, child_sizes.iter().copied()),
			child_positions: std::iter::repeat_n(Vec2::ZERO, child_sizes.len()).collect(),
		}
	}
}
