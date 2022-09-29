use glam::Vec2;

use crate::Context;

use super::{BuiltWidget, Widget};

pub struct Constrained {
	pub max_size: Vec2,
	pub child: Box<dyn Widget>,
}

impl Constrained {
	pub fn new(max_size: Vec2, child: impl Widget + 'static) -> Self {
		Self {
			max_size,
			child: Box::new(child),
		}
	}
}

impl Widget for Constrained {
	fn build(&self, ctx: &mut Context, max_size: Vec2) -> Box<dyn BuiltWidget> {
		self.child.build(ctx, self.max_size.min(max_size))
	}
}
