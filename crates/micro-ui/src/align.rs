use std::fmt::Debug;

use micro::{Context, math::Vec2};

use crate::{child_fns, sizing_fns};

use super::{LayoutResult, Sizing, Widget, WidgetMouseEventChannel};

#[derive(Debug)]
pub struct Align {
	parent_anchor: Vec2,
	child_anchor: Vec2,
	sizing: Sizing,
	children: Vec<Box<dyn Widget>>,
	mouse_event_channel: Option<WidgetMouseEventChannel>,
}

macro_rules! align_constructors {
	($($name:ident: $align:expr),*$(,)?) => {
		$(
			pub fn $name() -> Self {
				Self::simple($align)
			}
		)*
	};
}

impl Align {
	pub fn new(parent_anchor: impl Into<Vec2>, child_anchor: impl Into<Vec2>) -> Self {
		Self {
			parent_anchor: parent_anchor.into(),
			child_anchor: child_anchor.into(),
			sizing: Sizing::EXPAND,
			children: vec![],
			mouse_event_channel: None,
		}
	}

	pub fn simple(anchor: impl Into<Vec2>) -> Self {
		let anchor = anchor.into();
		Self::new(anchor, anchor)
	}

	align_constructors! {
		top_left: (0.0, 0.0),
		top_center: (0.5, 0.0),
		top_right: (1.0, 0.0),
		middle_right: (1.0, 0.5),
		bottom_right: (1.0, 1.0),
		bottom_center: (0.5, 1.0),
		bottom_left: (0.0, 1.0),
		middle_left: (0.0, 0.5),
		center: (0.5, 0.5),
	}

	pub fn mouse_event_channel(self, channel: &WidgetMouseEventChannel) -> Self {
		Self {
			mouse_event_channel: Some(channel.clone()),
			..self
		}
	}

	child_fns!();
	sizing_fns!();
}

impl Widget for Align {
	fn name(&self) -> &'static str {
		"align"
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
		let parent_size = self
			.sizing
			.final_parent_size(allotted_size_from_parent, child_sizes.iter().copied());
		let child_positions = child_sizes
			.iter()
			.copied()
			.map(|child_size| parent_size * self.parent_anchor - child_size * self.child_anchor)
			.collect();
		LayoutResult {
			size: parent_size,
			child_positions,
		}
	}
}
