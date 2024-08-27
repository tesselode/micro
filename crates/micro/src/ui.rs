mod align;
mod macros;
mod rectangle;
mod sizing;
mod stack;
#[allow(clippy::module_inception)]
mod ui;

pub use align::*;
pub use rectangle::*;
pub use sizing::*;
pub use stack::*;
pub use ui::Ui;

use std::fmt::Debug;

use glam::Vec2;

use crate::Context;

#[allow(unused_variables)]
pub trait Widget: Debug {
	fn name(&self) -> &'static str;

	fn children(&self) -> &[Box<dyn Widget>];

	fn allotted_size_for_next_child(
		&self,
		allotted_size_from_parent: Vec2,
		previous_child_sizes: &[Vec2],
	) -> Vec2;

	fn layout(&self, allotted_size_from_parent: Vec2, child_sizes: &[Vec2]) -> LayoutResult;

	fn draw(&self, ctx: &mut Context, size: Vec2) -> anyhow::Result<()> {
		Ok(())
	}
}

#[derive(Debug, Clone, PartialEq)]
pub struct LayoutResult {
	pub size: Vec2,
	pub child_positions: Vec<Vec2>,
}
