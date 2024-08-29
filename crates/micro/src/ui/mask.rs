use glam::Vec2;

use crate::{with_child_fns, with_sizing_fns, Context};

use super::{LayoutResult, Sizing, Widget, WidgetMouseEventChannel};

#[derive(Debug)]
pub struct Mask {
	sizing: Sizing,
	children: Vec<Box<dyn Widget>>,
	mask: Box<dyn Widget>,
	mouse_event_channel: Option<WidgetMouseEventChannel>,
}

impl Mask {
	pub fn new(mask: impl Widget + 'static) -> Self {
		Self {
			sizing: Sizing::SHRINK,
			children: vec![],
			mask: Box::new(mask),
			mouse_event_channel: None,
		}
	}

	pub fn with_mouse_event_channel(self, channel: &WidgetMouseEventChannel) -> Self {
		Self {
			mouse_event_channel: Some(channel.clone()),
			..self
		}
	}

	with_child_fns!();
	with_sizing_fns!();
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

	fn mouse_event_channel(&self) -> Option<&WidgetMouseEventChannel> {
		self.mouse_event_channel.as_ref()
	}

	fn allotted_size_for_next_child(
		&self,
		allotted_size_from_parent: glam::Vec2,
		_previous_child_sizes: &[glam::Vec2],
	) -> glam::Vec2 {
		self.sizing
			.allotted_size_for_children(allotted_size_from_parent)
	}

	fn layout(
		&self,
		_ctx: &mut Context,
		allotted_size_from_parent: glam::Vec2,
		child_sizes: &[glam::Vec2],
	) -> super::LayoutResult {
		LayoutResult {
			size: self
				.sizing
				.final_parent_size(allotted_size_from_parent, child_sizes.iter().copied()),
			child_positions: std::iter::repeat(Vec2::ZERO)
				.take(child_sizes.len())
				.collect(),
		}
	}
}
