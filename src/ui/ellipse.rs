use glam::Vec2;

use crate::{
	graphics::{
		color::Rgba,
		mesh::{Mesh, ShapeStyle},
	},
	Context,
};

use super::{BuiltWidget, Constraints, Widget};

pub struct Ellipse {
	pub radii: Vec2,
	pub style: ShapeStyle,
	pub color: Rgba,
}

impl Ellipse {
	pub fn new(radii: Vec2, style: ShapeStyle) -> Self {
		Self {
			radii,
			style,
			color: Rgba::WHITE,
		}
	}

	pub fn circle(radius: f32, style: ShapeStyle) -> Self {
		Self::new(Vec2::splat(radius), style)
	}

	pub fn with_color(self, color: Rgba) -> Self {
		Self { color, ..self }
	}
}

impl Widget for Ellipse {
	fn build(&self, ctx: &mut Context, constraints: Constraints) -> Box<dyn BuiltWidget> {
		let clamped_radii = self
			.radii
			.clamp(constraints.min_size / 2.0, constraints.max_size / 2.0);
		let center = clamped_radii;
		Box::new(BuiltCircle {
			size: clamped_radii * 2.0,
			mesh: Mesh::ellipse(ctx, self.style, center, clamped_radii, 0.0),
			color: self.color,
		})
	}
}

struct BuiltCircle {
	size: Vec2,
	mesh: Mesh,
	color: Rgba,
}

impl BuiltWidget for BuiltCircle {
	fn size(&self) -> Vec2 {
		self.size
	}

	fn draw(&self, ctx: &mut Context) {
		self.mesh.draw(ctx, self.color);
	}
}
