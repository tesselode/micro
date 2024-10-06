use micro::{
	math::{vec2, Vec2},
	Context,
};

use crate::{with_child_fns, with_sizing_fns};

use super::{LayoutResult, Sizing, Widget, WidgetMouseEventChannel};

#[derive(Debug)]
pub struct Padding {
	sizing: Sizing,
	left: f32,
	top: f32,
	right: f32,
	bottom: f32,
	children: Vec<Box<dyn Widget>>,
	mouse_event_channel: Option<WidgetMouseEventChannel>,
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
			mouse_event_channel: None,
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

	pub fn with_mouse_event_channel(self, channel: &WidgetMouseEventChannel) -> Self {
		Self {
			mouse_event_channel: Some(channel.clone()),
			..self
		}
	}

	with_child_fns!();
	with_sizing_fns!();

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
			mouse_event_channel: None,
		}
	}
}

impl Widget for Padding {
	fn name(&self) -> &'static str {
		"padding"
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
			- self.total_padding()
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
				.final_parent_size(allotted_size_from_parent, child_sizes.iter().copied())
				+ self.total_padding(),
			child_positions: std::iter::repeat(vec2(self.left, self.top))
				.take(child_sizes.len())
				.collect(),
		}
	}
}
