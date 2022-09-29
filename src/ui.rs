pub mod aligned;
pub mod constrained;
pub mod container;
pub mod ellipse;
pub mod flex;
pub mod image;
pub mod list;
pub mod padded;
pub mod rectangle;
pub mod transformed;

use glam::Vec2;

use crate::Context;

pub trait Widget {
	fn build(&self, ctx: &mut Context, max_size: Vec2) -> Box<dyn BuiltWidget>;
}

pub trait BuiltWidget {
	fn size(&self) -> Vec2;

	fn draw(&self, ctx: &mut Context);
}
