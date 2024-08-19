use glam::Vec2;
use palette::LinSrgba;

use crate::{graphics::mesh::Mesh, math::Rect, with_child_fns, Context};

use super::Widget;

#[derive(Debug, Default)]
pub struct Rectangle {
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
}

impl Widget for Rectangle {
	fn size(&mut self, max_size: Vec2) -> Vec2 {
		self.size = Some(max_size);
		for child in &mut self.children {
			child.size(max_size);
		}
		max_size
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
