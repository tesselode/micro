use glam::Vec2;
use palette::LinSrgba;

use crate::{graphics::mesh::Mesh, math::Rect, with_child_fns, with_sizing_fns, Context};

use super::{Sizing, Widget};

#[derive(Debug)]
pub struct Rectangle {
	sizing: Sizing,
	fill: Option<LinSrgba>,
	stroke: Option<(f32, LinSrgba)>,
	children: Vec<Box<dyn Widget>>,
	size: Option<Vec2>,
}

impl Rectangle {
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
}

impl Default for Rectangle {
	fn default() -> Self {
		Self {
			sizing: Sizing::MAX,
			fill: Default::default(),
			stroke: Default::default(),
			children: Default::default(),
			size: Default::default(),
		}
	}
}

impl Widget for Rectangle {
	fn size(&mut self, ctx: &mut Context, allotted_size: Vec2) -> Vec2 {
		let allotted_size_for_children = self.sizing.allotted_size_for_children(allotted_size);
		let child_sizes = self
			.children
			.iter_mut()
			.map(|child| child.size(ctx, allotted_size_for_children));
		let parent_size = self.sizing.final_parent_size(allotted_size, child_sizes);
		self.size = Some(parent_size);
		parent_size
	}

	fn draw(&self, ctx: &mut Context) -> anyhow::Result<()> {
		if let Some(fill) = self.fill {
			Mesh::rectangle(ctx, Rect::new(Vec2::ZERO, self.size.unwrap()))
				.color(fill)
				.draw(ctx);
		}
		for child in &self.children {
			child.draw(ctx)?;
		}
		if let Some((width, color)) = self.stroke {
			Mesh::outlined_rectangle(ctx, width, Rect::new(Vec2::ZERO, self.size.unwrap()))?
				.color(color)
				.draw(ctx);
		}
		Ok(())
	}
}
