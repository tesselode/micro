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
	pub style: ShapeStyle,
	pub color: Rgba,
}

impl Ellipse {
	pub fn new(style: ShapeStyle, color: Rgba) -> Self {
		Self { style, color }
	}
}

impl Widget for Ellipse {
	fn build(&self, ctx: &mut Context, constraints: Constraints) -> Box<dyn BuiltWidget> {
		let radii = constraints.max_size / 2.0;
		let center = radii;
		Box::new(BuiltEllipse {
			size: constraints.max_size,
			mesh: Mesh::ellipse(ctx, self.style, center, radii, 0.0),
			color: self.color,
		})
	}
}

struct BuiltEllipse {
	size: Vec2,
	mesh: Mesh,
	color: Rgba,
}

impl BuiltWidget for BuiltEllipse {
	fn size(&self) -> Vec2 {
		self.size
	}

	fn draw(&self, ctx: &mut Context) {
		self.mesh.draw(ctx, self.color);
	}
}
