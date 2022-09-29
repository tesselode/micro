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
	pub fill: Option<Rgba>,
	pub stroke: Option<(f32, Rgba)>,
}

impl Rectangle {
	pub fn new() -> Self {
		Self {
			fill: None,
			stroke: None,
		}
	}

	pub fn with_fill(self, color: Rgba) -> Self {
		Self {
			fill: Some(color),
			..self
		}
	}

	pub fn with_stroke(self, width: f32, color: Rgba) -> Self {
		Self {
			stroke: Some((width, color)),
			..self
		}
	}
}

impl Default for Rectangle {
	fn default() -> Self {
		Self::new()
	}
}

impl Widget for Rectangle {
	fn build(&self, ctx: &mut Context, constraints: Constraints) -> Box<dyn BuiltWidget> {
		let rect = Rect::from_top_left_and_size(Vec2::ZERO, constraints.max_size);
		Box::new(BuiltRectangle {
			size: constraints.max_size,
			fill: self.fill.map(|color| ColoredMesh {
				mesh: Mesh::styled_rectangle(ctx, ShapeStyle::Fill, rect),
				color,
			}),
			stroke: self.stroke.map(|(width, color)| ColoredMesh {
				mesh: Mesh::styled_rectangle(ctx, ShapeStyle::Stroke(width), rect),
				color,
			}),
		})
	}
}

struct BuiltRectangle {
	size: Vec2,
	fill: Option<ColoredMesh>,
	stroke: Option<ColoredMesh>,
}

impl BuiltWidget for BuiltRectangle {
	fn size(&self) -> Vec2 {
		self.size
	}

	fn draw(&self, ctx: &mut Context) {
		if let Some(ColoredMesh { mesh, color }) = &self.fill {
			mesh.draw(ctx, *color);
		}
		if let Some(ColoredMesh { mesh, color }) = &self.stroke {
			mesh.draw(ctx, *color);
		}
	}
}

struct ColoredMesh {
	mesh: Mesh,
	color: Rgba,
}
