use std::fmt::Debug;

use glam::Vec2;

use crate::{with_child_fns, with_mouse_event_fns, with_sizing_fns};

use super::{LayoutResult, MouseEvents, Sizing, Widget};

#[derive(Debug)]
pub struct Align<Event> {
	align: Vec2,
	sizing: Sizing,
	children: Vec<Box<dyn Widget<Event>>>,
	mouse_events: MouseEvents<Event>,
}

macro_rules! align_constructors {
	($($name:ident: $align:expr),*$(,)?) => {
		$(
			pub fn $name() -> Self {
				Self::new($align)
			}
		)*
	};
}

impl<Event> Align<Event> {
	pub fn new(align: impl Into<Vec2>) -> Self {
		Self {
			align: align.into(),
			sizing: Sizing::MAX,
			children: vec![],
			mouse_events: MouseEvents::default(),
		}
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

	with_child_fns!();
	with_sizing_fns!();
	with_mouse_event_fns!();
}

impl<Event: Debug + Copy> Widget<Event> for Align<Event> {
	fn name(&self) -> &'static str {
		"align"
	}

	fn children(&self) -> &[Box<dyn Widget<Event>>] {
		&self.children
	}

	fn mouse_events(&self) -> MouseEvents<Event> {
		self.mouse_events
	}

	fn allotted_size_for_next_child(
		&self,
		allotted_size_from_parent: Vec2,
		_previous_child_sizes: &[Vec2],
	) -> Vec2 {
		self.sizing
			.allotted_size_for_children(allotted_size_from_parent)
	}

	fn layout(&self, allotted_size_from_parent: Vec2, child_sizes: &[Vec2]) -> LayoutResult {
		let parent_size = self
			.sizing
			.final_parent_size(allotted_size_from_parent, child_sizes.iter().copied());
		let child_positions = child_sizes
			.iter()
			.copied()
			.map(|size| (parent_size - size) * self.align)
			.collect();
		LayoutResult {
			size: parent_size,
			child_positions,
		}
	}
}
