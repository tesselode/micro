use glam::{Mat3, Vec2};

use crate::Context;

use super::{BuiltWidget, Widget};

pub struct Aligned {
	pub child: Box<dyn Widget>,
	pub alignment: Vec2,
}

impl Aligned {
	pub fn new(alignment: Vec2, child: impl Widget + 'static) -> Self {
		Self {
			child: Box::new(child),
			alignment,
		}
	}

	pub fn center(child: impl Widget + 'static) -> Self {
		Self::new(Vec2::new(0.5, 0.5), child)
	}

	pub fn top_left(child: impl Widget + 'static) -> Self {
		Self::new(Vec2::ZERO, child)
	}

	pub fn top_right(child: impl Widget + 'static) -> Self {
		Self::new(Vec2::new(1.0, 0.0), child)
	}

	pub fn bottom_left(child: impl Widget + 'static) -> Self {
		Self::new(Vec2::new(0.0, 1.0), child)
	}

	pub fn bottom_right(child: impl Widget + 'static) -> Self {
		Self::new(Vec2::ONE, child)
	}
}

impl Widget for Aligned {
	fn build(&self, ctx: &mut Context, max_size: Vec2) -> Box<dyn BuiltWidget> {
		let child = self.child.build(ctx, max_size);
		let parent_size = max_size;
		let position = parent_size * self.alignment - child.size() * self.alignment;
		Box::new(BuiltAligned {
			size: parent_size,
			child,
			child_position: position,
		})
	}
}

struct BuiltAligned {
	size: Vec2,
	child: Box<dyn BuiltWidget>,
	child_position: Vec2,
}

impl BuiltWidget for BuiltAligned {
	fn size(&self) -> Vec2 {
		self.size
	}

	fn draw(&self, ctx: &mut Context) {
		ctx.with_transform(Mat3::from_translation(self.child_position), |ctx| {
			self.child.draw(ctx);
		});
	}
}
