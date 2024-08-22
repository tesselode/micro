use glam::Vec2;
use palette::LinSrgba;

use crate::{graphics::mesh::Mesh, Context};

use super::Widget;

#[derive(Debug)]
pub struct Polyline {
	points: Vec<Vec2>,
	stroke_width: f32,
	color: LinSrgba,
	size: Vec2,
}

impl Polyline {
	pub fn new(
		points: impl IntoIterator<Item = impl Into<Vec2>>,
		stroke_width: f32,
		color: impl Into<LinSrgba>,
	) -> Self {
		let points = points
			.into_iter()
			.map(|point| point.into())
			.collect::<Vec<_>>();
		let size = points.iter().copied().reduce(Vec2::max).unwrap_or_default();
		Self {
			points,
			stroke_width,
			color: color.into(),
			size,
		}
	}
}

impl Widget for Polyline {
	fn size(&mut self, _ctx: &mut Context, _max_size: Vec2) -> Vec2 {
		self.size
	}

	fn draw(&self, ctx: &mut Context) -> anyhow::Result<()> {
		Mesh::simple_polyline(ctx, self.stroke_width, self.points.iter().copied())?
			.color(self.color)
			.draw(ctx);
		Ok(())
	}
}
