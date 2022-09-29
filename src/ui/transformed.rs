use glam::{Mat3, Vec2};

use crate::Context;

use super::{BuiltWidget, Widget};

pub struct Transformed {
	pub transform: Mat3,
	pub child: Box<dyn Widget>,
}

impl Transformed {
	pub fn new(transform: Mat3, child: impl Widget + 'static) -> Self {
		Self {
			transform,
			child: Box::new(child),
		}
	}

	pub fn translated(translation: Vec2, child: impl Widget + 'static) -> Self {
		Self::new(Mat3::from_translation(translation), child)
	}

	pub fn scaled(scale: Vec2, child: impl Widget + 'static) -> Self {
		Self::new(Mat3::from_scale(scale), child)
	}

	pub fn rotated(rotation: f32, child: impl Widget + 'static) -> Self {
		Self::new(Mat3::from_rotation_z(rotation), child)
	}
}

impl Widget for Transformed {
	fn build(&self, ctx: &mut Context, max_size: Vec2) -> Box<dyn BuiltWidget> {
		let built_child = self.child.build(ctx, max_size);
		let size = built_child.size();
		Box::new(BuiltTransformed {
			size,
			transform: self.transform,
			child: built_child,
		})
	}
}

struct BuiltTransformed {
	size: Vec2,
	transform: Mat3,
	child: Box<dyn BuiltWidget>,
}

impl BuiltWidget for BuiltTransformed {
	fn size(&self) -> Vec2 {
		self.size
	}

	fn draw(&self, ctx: &mut Context) {
		ctx.with_transform(self.transform, |ctx| {
			self.child.draw(ctx);
		});
	}
}
