use glam::{Mat3, Vec2};

use crate::Context;

use super::{BuiltWidget, Widget};

pub struct Container {
	pub children: Vec<(Vec2, Box<dyn Widget>)>,
}

impl Container {
	pub fn new() -> Self {
		Self { children: vec![] }
	}

	pub fn with_child(mut self, position: Vec2, child: impl Widget + 'static) -> Self {
		self.children.push((position, Box::new(child)));
		self
	}
}

impl Default for Container {
	fn default() -> Self {
		Self::new()
	}
}

impl Widget for Container {
	fn build(&self, ctx: &mut Context, max_size: Vec2) -> Box<dyn BuiltWidget> {
		Box::new(BuiltContainer {
			size: max_size,
			children: self
				.children
				.iter()
				.map(|(position, child)| (*position, child.build(ctx, max_size - *position)))
				.collect(),
		})
	}
}

struct BuiltContainer {
	size: Vec2,
	children: Vec<(Vec2, Box<dyn BuiltWidget>)>,
}

impl BuiltWidget for BuiltContainer {
	fn size(&self) -> Vec2 {
		self.size
	}

	fn draw(&self, ctx: &mut Context) {
		for (position, child) in &self.children {
			ctx.with_transform(Mat3::from_translation(*position), |ctx| {
				child.draw(ctx);
			});
		}
	}
}
