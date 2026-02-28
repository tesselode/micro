use micro::{
	Context,
	math::{Vec2, vec2},
};

use crate::{
	WidgetInspector, child_functions, common_functions, common_widget_trait_functions,
	sizing_functions,
};

use super::{LayoutResult, Sizing, Widget, WidgetMouseState};

#[derive(Debug)]
pub struct Padding {
	sizing: Sizing,
	left: f32,
	top: f32,
	right: f32,
	bottom: f32,
	children: Vec<Box<dyn Widget>>,
	mouse_state: Option<WidgetMouseState>,
	inspector: Option<WidgetInspector>,
}

impl Padding {
	pub fn new(left: f32, top: f32, right: f32, bottom: f32) -> Self {
		Self {
			sizing: Sizing::SHRINK,
			left,
			top,
			right,
			bottom,
			children: vec![],
			mouse_state: None,
			inspector: None,
		}
	}

	pub fn all(padding: f32) -> Self {
		Self::new(padding, padding, padding, padding)
	}

	pub fn symmetric(padding: Vec2) -> Self {
		Self::new(padding.x, padding.y, padding.x, padding.y)
	}

	pub fn horizontal(padding: f32) -> Self {
		Self::symmetric(vec2(padding, 0.0))
	}

	pub fn vertical(padding: f32) -> Self {
		Self::symmetric(vec2(0.0, padding))
	}

	pub fn left(padding: f32) -> Self {
		Self::new(padding, 0.0, 0.0, 0.0)
	}

	pub fn top(padding: f32) -> Self {
		Self::new(0.0, padding, 0.0, 0.0)
	}

	pub fn right(padding: f32) -> Self {
		Self::new(0.0, 0.0, padding, 0.0)
	}

	pub fn bottom(padding: f32) -> Self {
		Self::new(0.0, 0.0, 0.0, padding)
	}

	common_functions!();
	child_functions!();
	sizing_functions!();

	fn total_padding(&self) -> Vec2 {
		vec2(self.left + self.right, self.top + self.bottom)
	}
}

impl Default for Padding {
	fn default() -> Self {
		Self {
			sizing: Sizing::SHRINK,
			left: Default::default(),
			top: Default::default(),
			right: Default::default(),
			bottom: Default::default(),
			children: Default::default(),
			mouse_state: None,
			inspector: None,
		}
	}
}

impl Widget for Padding {
	common_widget_trait_functions!();

	fn name(&self) -> &'static str {
		"padding"
	}

	fn children(&self) -> &[Box<dyn Widget>] {
		&self.children
	}

	fn allotted_size_for_next_child(
		&self,
		allotted_size_from_parent: Vec2,
		_previous_child_sizes: &[Vec2],
	) -> Vec2 {
		let _span = tracy_client::span!();
		self.sizing
			.allotted_size_for_children(allotted_size_from_parent)
			- self.total_padding()
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
				.final_parent_size(allotted_size_from_parent, child_sizes.iter().copied())
				+ self.total_padding(),
			child_positions: std::iter::repeat_n(vec2(self.left, self.top), child_sizes.len())
				.collect(),
		}
	}
}
