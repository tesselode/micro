use std::fmt::Debug;

use glam::Vec2;
use palette::LinSrgba;

use crate::{
	graphics::mesh::Mesh, math::Rect, with_child_fns, with_mouse_event_fns, with_sizing_fns,
	Context,
};

use super::{LayoutResult, MouseEvents, Sizing, Widget};

#[derive(Debug)]
pub struct Rectangle<Event> {
	sizing: Sizing,
	fill: Option<LinSrgba>,
	stroke: Option<(f32, LinSrgba)>,
	children: Vec<Box<dyn Widget<Event>>>,
	mouse_events: MouseEvents<Event>,
}

impl<Event> Rectangle<Event> {
	pub fn new() -> Self {
		Self::default()
	}

	pub fn with_fill(self, color: impl Into<LinSrgba>) -> Self {
		Self {
			fill: Some(color.into()),
			..self
		}
	}

	pub fn with_stroke(self, width: f32, color: impl Into<LinSrgba>) -> Self {
		Self {
			stroke: Some((width, color.into())),
			..self
		}
	}

	with_child_fns!();
	with_sizing_fns!();
	with_mouse_event_fns!();
}

impl<Event> Default for Rectangle<Event> {
	fn default() -> Self {
		Self {
			sizing: Sizing::MAX,
			fill: Default::default(),
			stroke: Default::default(),
			children: Default::default(),
			mouse_events: Default::default(),
		}
	}
}

impl<Event: Debug + Copy> Widget<Event> for Rectangle<Event> {
	fn name(&self) -> &'static str {
		"rectangle"
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
		LayoutResult {
			size: self
				.sizing
				.final_parent_size(allotted_size_from_parent, child_sizes.iter().copied()),
			child_positions: std::iter::repeat(Vec2::ZERO)
				.take(child_sizes.len())
				.collect(),
		}
	}

	fn draw(&self, ctx: &mut Context, size: Vec2) -> anyhow::Result<()> {
		if let Some(fill) = self.fill {
			Mesh::rectangle(ctx, Rect::new(Vec2::ZERO, size))
				.color(fill)
				.draw(ctx);
		}
		if let Some((width, color)) = self.stroke {
			Mesh::outlined_rectangle(ctx, width, Rect::new(Vec2::ZERO, size))?
				.color(color)
				.draw(ctx);
		}
		Ok(())
	}
}
