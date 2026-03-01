mod align;
mod aspect_ratio;
mod distribute;
mod dynamic_transform;
mod ellipse;
mod image;
mod inspector;
mod macros;
mod manually_positioned;
mod mask;
mod mouse_input;
mod padding;
mod polygon;
mod polyline;
mod rectangle;
mod sizing;
mod stack;
mod text;
mod transform;
mod ui;
mod widget_mouse_state;

pub use align::*;
pub use aspect_ratio::*;
pub use distribute::*;
pub use dynamic_transform::*;
pub use ellipse::*;
pub use image::*;
pub use inspector::*;
pub use manually_positioned::*;
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
pub use widget_mouse_state::*;

use std::fmt::Debug;

use micro::{
	Context,
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

	fn mouse_state(&self) -> Option<WidgetMouseState>;

	fn inspector(&self) -> Option<WidgetInspector>;

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
