pub mod align;
pub mod ellipse;
pub mod flex;
pub mod image;
pub mod list;
pub mod rectangle;

use glam::Vec2;

use crate::Context;

pub trait Widget {
	fn build(&self, ctx: &mut Context, constraints: Constraints) -> Box<dyn BuiltWidget>;
}

pub trait BuiltWidget {
	fn size(&self) -> Vec2;

	fn draw(&self, ctx: &mut Context);
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Constraints {
	pub min_size: Vec2,
	pub max_size: Vec2,
}
