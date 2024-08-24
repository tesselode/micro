use glam::Vec2;
use palette::LinSrgba;

use crate::{
	graphics::mesh::{Mesh, ShapeStyle},
	Context,
};

use super::Widget;

#[derive(Debug)]
pub struct Polygon {
	points: Vec<Vec2>,
	size: Vec2,
	fill: Option<LinSrgba>,
	stroke: Option<(f32, LinSrgba)>,
}

impl Polygon {
	pub fn new(points: impl IntoIterator<Item = impl Into<Vec2>>) -> Self {
		let points = points
			.into_iter()
			.map(|point| point.into())
			.collect::<Vec<_>>();
		let size = points.iter().copied().reduce(Vec2::max).unwrap_or_default();
		Self {
			points,
			size,
			fill: None,
			stroke: None,
		}
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
}

impl Widget for Polygon {
	fn size(&mut self, _ctx: &mut Context, _allotted_size: Vec2) -> Vec2 {
		self.size
	}

	fn draw(&self, ctx: &mut Context) -> anyhow::Result<()> {
		if let Some(fill) = self.fill {
			Mesh::simple_polygon(ctx, ShapeStyle::Fill, self.points.iter().copied())?
				.color(fill)
				.draw(ctx);
		}
		if let Some((width, color)) = self.stroke {
			Mesh::simple_polygon(ctx, ShapeStyle::Stroke(width), self.points.iter().copied())?
				.color(color)
				.draw(ctx);
		}
		Ok(())
	}
}
