mod align;
mod ellipse;
mod image;
mod macros;
mod mask;
mod padding;
mod polygon;
mod polyline;
mod rectangle;
mod sizing;
mod stack;
mod text;
mod transform;
mod ui;

pub use align::*;
pub use ellipse::*;
pub use image::*;
pub use mask::*;
pub use padding::*;
pub use polygon::*;
pub use polyline::*;
pub use rectangle::*;
pub use sizing::*;
pub use stack::*;
pub use text::*;
pub use transform::*;
pub use ui::*;

use std::{cell::RefCell, collections::VecDeque, fmt::Debug, rc::Rc};

use micro::{
	Context,
	input::MouseButton,
	math::{Mat4, Vec2},
};

#[allow(unused_variables)]
pub trait Widget: Debug {
	fn name(&self) -> &'static str;

	fn children(&self) -> &[Box<dyn Widget>];

	fn transform(&self, size: Vec2) -> Mat4 {
		Mat4::IDENTITY
	}

	fn mask(&self) -> Option<&dyn Widget> {
		None
	}

	fn mouse_event_channel(&self) -> Option<&WidgetMouseEventChannel>;

	fn allotted_size_for_next_child(
		&self,
		allotted_size_from_parent: Vec2,
		previous_child_sizes: &[Vec2],
	) -> Vec2;

	fn layout(
		&self,
		ctx: &mut Context,
		allotted_size_from_parent: Vec2,
		child_sizes: &[Vec2],
	) -> LayoutResult;

	fn draw_before_children(&self, ctx: &mut Context, size: Vec2) {}

	fn draw_after_children(&self, ctx: &mut Context, size: Vec2) {}
}

#[derive(Debug, Clone, PartialEq)]
pub struct LayoutResult {
	pub size: Vec2,
	pub child_positions: Vec<Vec2>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct WidgetMouseEventChannel(Rc<RefCell<VecDeque<WidgetMouseEvent>>>);

impl WidgetMouseEventChannel {
	pub fn new() -> Self {
		Self(Rc::new(RefCell::new(VecDeque::new())))
	}

	pub fn push(&self, event: WidgetMouseEvent) {
		self.0.borrow_mut().push_back(event);
	}

	pub fn pop(&self) -> Option<WidgetMouseEvent> {
		self.0.borrow_mut().pop_front()
	}
}

impl Default for WidgetMouseEventChannel {
	fn default() -> Self {
		Self::new()
	}
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WidgetMouseEvent {
	Hovered,
	Unhovered,
	ClickStarted {
		button: MouseButton,
		relative_pos: Vec2,
	},
	Clicked {
		button: MouseButton,
		relative_pos: Vec2,
	},
}
