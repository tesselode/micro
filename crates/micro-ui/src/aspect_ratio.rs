use micro::{
	Context,
	math::{Vec2, vec2},
};

use crate::{
	LayoutResult, Widget, WidgetInspector, WidgetMouseState, child_functions, common_functions,
	common_widget_trait_functions,
};

#[derive(Debug)]
pub struct AspectRatio {
	aspect_ratio: f32,
	children: Vec<Box<dyn Widget>>,
	mouse_state: Option<WidgetMouseState>,
	inspector: Option<WidgetInspector>,
}

impl AspectRatio {
	pub fn new(aspect_ratio: f32) -> Self {
		Self {
			aspect_ratio,
			children: vec![],
			mouse_state: None,
			inspector: None,
		}
	}

	pub fn square() -> Self {
		Self::new(1.0)
	}

	common_functions!();
	child_functions!();

	fn size(&self, parent_size: Vec2) -> Vec2 {
		let mut size = vec2(parent_size.x, parent_size.x / self.aspect_ratio);
		if size.y > parent_size.y {
			size /= size.y / parent_size.y;
		}
		size
	}
}

impl Widget for AspectRatio {
	common_widget_trait_functions!();

	fn name(&self) -> &'static str {
		"aspect ratio"
	}

	fn children(&self) -> &[Box<dyn Widget>] {
		&self.children
	}

	fn allotted_size_for_next_child(
		&self,
		allotted_size_from_parent: Vec2,
		_previous_child_sizes: &[Vec2],
	) -> Vec2 {
		self.size(allotted_size_from_parent)
	}

	fn layout(
		&self,
		_ctx: &mut Context,
		allotted_size_from_parent: Vec2,
		child_sizes: &[Vec2],
	) -> LayoutResult {
		let _span = tracy_client::span!();
		LayoutResult {
			size: self.size(allotted_size_from_parent),
			child_positions: std::iter::repeat_n(Vec2::ZERO, child_sizes.len()).collect(),
		}
	}
}
