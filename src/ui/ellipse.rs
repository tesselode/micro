use glam::Vec2;

use crate::{
	graphics::{
		color::Rgba,
		mesh::{Mesh, ShapeStyle},
	},
	Context,
};

use super::{BuiltWidget, Widget};

pub struct Ellipse {
	pub fill: Option<Rgba>,
	pub stroke: Option<(f32, Rgba)>,
}

impl Ellipse {
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

impl Default for Ellipse {
	fn default() -> Self {
		Self::new()
	}
}

impl Widget for Ellipse {
	fn build(&self, ctx: &mut Context, max_size: Vec2) -> Box<dyn BuiltWidget> {
		let radii = max_size / 2.0;
		let center = radii;
		Box::new(BuiltEllipse {
			size: max_size,
			fill: self.fill.map(|color| ColoredMesh {
				mesh: Mesh::ellipse(ctx, ShapeStyle::Fill, center, radii, 0.0),
				color,
			}),
			stroke: self.stroke.map(|(width, color)| ColoredMesh {
				mesh: Mesh::ellipse(ctx, ShapeStyle::Stroke(width), center, radii, 0.0),
				color,
			}),
		})
	}
}

struct BuiltEllipse {
	size: Vec2,
	fill: Option<ColoredMesh>,
	stroke: Option<ColoredMesh>,
}

impl BuiltWidget for BuiltEllipse {
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
