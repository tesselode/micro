use crate::Context;

use super::{BuiltWidget, Constraints, Widget};

pub struct Constrained {
	pub constraints: Constraints,
	pub child: Box<dyn Widget>,
}

impl Constrained {
	pub fn new(constraints: Constraints, child: impl Widget + 'static) -> Self {
		Self {
			constraints,
			child: Box::new(child),
		}
	}
}

impl Widget for Constrained {
	fn build(&self, ctx: &mut Context, constraints: Constraints) -> Box<dyn BuiltWidget> {
		self.child.build(ctx, self.constraints.union(constraints))
	}
}
