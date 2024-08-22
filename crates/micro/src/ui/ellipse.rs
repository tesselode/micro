use glam::Vec2;
use palette::LinSrgba;

use crate::{
	graphics::mesh::{Mesh, ShapeStyle},
	with_child_fns, Context,
};

use super::Widget;

#[derive(Debug, Default)]
pub struct Ellipse {
	fill: Option<LinSrgba>,
	stroke: Option<(f32, LinSrgba)>,
	children: Vec<Box<dyn Widget>>,
	size: Option<Vec2>,
}

impl Ellipse {
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

impl Widget for Ellipse {
	fn size(&mut self, ctx: &mut Context, max_size: Vec2) -> Vec2 {
		self.size = Some(max_size);
		for child in &mut self.children {
			child.size(ctx, max_size);
		}
		max_size
	}

	fn draw(&self, ctx: &mut Context) -> anyhow::Result<()> {
		let size = self.size.unwrap();
		if let Some(fill) = self.fill {
			Mesh::ellipse(ctx, ShapeStyle::Fill, size / 2.0, size / 2.0, 0.0)?
				.color(fill)
				.draw(ctx);
		}
		for child in &self.children {
			child.draw(ctx)?;
		}
		if let Some((width, color)) = self.stroke {
			Mesh::ellipse(ctx, ShapeStyle::Stroke(width), size / 2.0, size / 2.0, 0.0)?
				.color(color)
				.draw(ctx);
		}
		Ok(())
	}
}
