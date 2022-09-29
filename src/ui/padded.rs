use glam::{Mat3, Vec2};

use crate::Context;

use super::{BuiltWidget, Widget};

pub struct Padded {
	pub left: f32,
	pub right: f32,
	pub top: f32,
	pub bottom: f32,
	pub child: Box<dyn Widget>,
}

impl Padded {
	pub fn new(left: f32, right: f32, top: f32, bottom: f32, child: impl Widget + 'static) -> Self {
		Self {
			left,
			right,
			top,
			bottom,
			child: Box::new(child),
		}
	}

	pub fn left(padding: f32, child: impl Widget + 'static) -> Self {
		Self::new(padding, 0.0, 0.0, 0.0, child)
	}

	pub fn right(padding: f32, child: impl Widget + 'static) -> Self {
		Self::new(0.0, padding, 0.0, 0.0, child)
	}

	pub fn top(padding: f32, child: impl Widget + 'static) -> Self {
		Self::new(0.0, 0.0, padding, 0.0, child)
	}

	pub fn bottom(padding: f32, child: impl Widget + 'static) -> Self {
		Self::new(0.0, 0.0, 0.0, padding, child)
	}

	pub fn horizontal(padding: f32, child: impl Widget + 'static) -> Self {
		Self::new(padding, padding, 0.0, 0.0, child)
	}

	pub fn vertical(padding: f32, child: impl Widget + 'static) -> Self {
		Self::new(0.0, 0.0, padding, padding, child)
	}

	pub fn all(padding: f32, child: impl Widget + 'static) -> Self {
		Self::new(padding, padding, padding, padding, child)
	}
}

impl Widget for Padded {
	fn build(&self, ctx: &mut Context, max_size: Vec2) -> Box<dyn BuiltWidget> {
		let child_max_size = max_size - Vec2::new(self.left + self.right, self.top + self.bottom);
		Box::new(BuiltPadded {
			size: max_size,
			child: self.child.build(ctx, child_max_size),
			child_position: Vec2::new(self.left, self.top),
		})
	}
}

struct BuiltPadded {
	size: Vec2,
	child: Box<dyn BuiltWidget>,
	child_position: Vec2,
}

impl BuiltWidget for BuiltPadded {
	fn size(&self) -> Vec2 {
		self.size
	}

	fn draw(&self, ctx: &mut Context) {
		ctx.with_transform(Mat3::from_translation(self.child_position), |ctx| {
			self.child.draw(ctx);
		});
	}
}
