use glam::Vec2;

use crate::{
	graphics::{
		mesh::{Mesh, ShapeStyle},
		DrawParams,
	},
	Context,
};

use super::{BuiltWidget, Constraints, Widget};

pub struct Circle {
	pub radius: f32,
	pub style: ShapeStyle,
}

impl Circle {
	pub fn new(radius: f32, style: ShapeStyle) -> Self {
		Self { radius, style }
	}
}

impl Widget for Circle {
	fn build(&self, ctx: &mut Context, constraints: Constraints) -> Box<dyn BuiltWidget> {
		let radii = Vec2::new(
			self.radius
				.clamp(constraints.min_size.x / 2.0, constraints.max_size.x / 2.0),
			self.radius
				.clamp(constraints.min_size.y / 2.0, constraints.max_size.y / 2.0),
		);
		let center = radii;
		Box::new(BuiltCircle {
			size: radii * 2.0,
			mesh: Mesh::ellipse(ctx, self.style, center, radii, 0.0),
		})
	}
}

struct BuiltCircle {
	size: Vec2,
	mesh: Mesh,
}

impl BuiltWidget for BuiltCircle {
	fn size(&self) -> Vec2 {
		self.size
	}

	fn draw(&self, ctx: &mut Context) {
		self.mesh.draw(ctx, DrawParams::new());
	}
}
