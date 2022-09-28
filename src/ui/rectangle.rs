use glam::Vec2;

use crate::{
	graphics::{
		color::Rgba,
		mesh::{Mesh, ShapeStyle},
	},
	math::Rect,
	Context,
};

use super::{BuiltWidget, Constraints, Widget};

pub struct Rectangle {
	pub style: ShapeStyle,
	pub color: Rgba,
}

impl Rectangle {
	pub fn new(style: ShapeStyle, color: Rgba) -> Self {
		Self { style, color }
	}
}

impl Widget for Rectangle {
	fn build(&self, ctx: &mut Context, constraints: Constraints) -> Box<dyn BuiltWidget> {
		Box::new(BuiltRectangle {
			size: constraints.max_size,
			mesh: Mesh::styled_rectangle(
				ctx,
				self.style,
				Rect::from_top_left_and_size(Vec2::ZERO, constraints.max_size),
			),
			color: self.color,
		})
	}
}

struct BuiltRectangle {
	size: Vec2,
	mesh: Mesh,
	color: Rgba,
}

impl BuiltWidget for BuiltRectangle {
	fn size(&self) -> Vec2 {
		self.size
	}

	fn draw(&self, ctx: &mut Context) {
		self.mesh.draw(ctx, self.color);
	}
}
