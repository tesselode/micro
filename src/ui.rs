pub mod align;
pub mod constrained;
mod constraints;
pub mod ellipse;
pub mod flex;
pub mod image;
pub mod list;
pub mod rectangle;

pub use constraints::*;

use glam::Vec2;

use crate::Context;

pub trait Widget {
	fn build(&self, ctx: &mut Context, constraints: Constraints) -> Box<dyn BuiltWidget>;
}

pub trait BuiltWidget {
	fn size(&self) -> Vec2;

	fn draw(&self, ctx: &mut Context);
}
