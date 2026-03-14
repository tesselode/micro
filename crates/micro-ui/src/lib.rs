mod align;
mod aspect_ratio;
mod distribute;
mod ellipse;
mod image;
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
mod widget_state;

pub use align::*;
pub use aspect_ratio::*;
pub use distribute::*;
pub use ellipse::*;
pub use image::*;
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
pub use widget_state::*;

use std::fmt::Debug;

use micro::{
	Context,
	math::{Mat4, Vec2},
};

#[allow(unused_variables)]
pub trait Widget: Debug {
	fn name(&self) -> &'static str;

	fn custom_id(&self) -> Option<String>;

	fn children(&mut self, state: &mut WidgetState) -> Vec<Box<dyn Widget>>;

	fn transform(&mut self, size: Vec2, state: &mut WidgetState) -> Mat4 {
		Mat4::IDENTITY
	}

	fn mask(&mut self, state: &mut WidgetState) -> Option<Box<dyn Widget>> {
		None
	}

	fn allotted_size_for_next_child(
		&mut self,
		allotted_size_from_parent: Vec2,
		previous_child_sizes: &[Vec2],
		state: &mut WidgetState,
	) -> Vec2;

	fn layout(
		&mut self,
		ctx: &mut Context,
		allotted_size_from_parent: Vec2,
		child_sizes: &[Vec2],
		state: &mut WidgetState,
	) -> LayoutResult;

	fn draw_before_children(&mut self, ctx: &mut Context, size: Vec2, state: &mut WidgetState) {}

	fn draw_after_children(&mut self, ctx: &mut Context, size: Vec2, state: &mut WidgetState) {}
}

#[derive(Debug, Clone, PartialEq)]
pub struct LayoutResult {
	pub size: Vec2,
	pub child_positions: Vec<Vec2>,
}
