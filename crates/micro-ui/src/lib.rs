mod align;
mod ellipse;
mod graphics_pipeline_widget;
mod image;
mod macros;
mod padding;
mod polygon;
mod polyline;
mod rectangle;
mod sizing;
mod stack;
mod stencil_reference_widget;
mod text;
mod transform;
#[allow(clippy::module_inception)]
mod ui;

pub use align::*;
pub use ellipse::*;
pub use graphics_pipeline_widget::*;
pub use image::*;
pub use padding::*;
pub use polygon::*;
pub use polyline::*;
pub use rectangle::*;
pub use sizing::*;
pub use stack::*;
pub use stencil_reference_widget::*;
pub use text::*;
pub use transform::*;
pub use ui::Ui;

use std::{cell::RefCell, collections::VecDeque, fmt::Debug, rc::Rc};

use micro::{
	Context,
	graphics::GraphicsPipeline,
	math::{Mat4, Vec2},
};

#[allow(unused_variables)]
pub trait Widget: Debug {
	fn name(&self) -> &'static str;

	fn children(&self) -> &[Box<dyn Widget>];

	fn transform(&self, size: Vec2) -> Mat4 {
		Mat4::IDENTITY
	}

	fn graphics_pipeline(&self) -> Option<GraphicsPipeline> {
		None
	}

	fn stencil_reference(&self) -> Option<u8> {
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

	fn draw_before_children(&self, ctx: &mut Context, size: Vec2) -> anyhow::Result<()> {
		Ok(())
	}

	fn draw_after_children(&self, ctx: &mut Context, size: Vec2) -> anyhow::Result<()> {
		Ok(())
	}
}

#[derive(Debug, Clone, PartialEq)]
pub struct LayoutResult {
	pub size: Vec2,
	pub child_positions: Vec<Vec2>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WidgetMouseEvent {
	Hovered,
	Unhovered,
	ClickStarted,
	Clicked,
}
