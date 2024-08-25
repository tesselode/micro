mod align;
mod ellipse;
mod image;
mod macros;
mod mask;
mod match_size;
mod padding;
mod polygon;
mod polyline;
mod rectangle;
mod sizing;
mod stack;
mod text;
mod transform;

pub use align::*;
pub use ellipse::*;
pub use image::*;
pub use mask::*;
pub use match_size::*;
pub use padding::*;
pub use polygon::*;
pub use polyline::*;
pub use rectangle::*;
pub use sizing::*;
pub use stack::*;
pub use text::{TextSettings, TextShadow, TextSizeReporting, TextSizing, TextWidget as Text};
pub use transform::*;

use std::fmt::Debug;

use glam::Vec2;

use crate::Context;

pub trait Widget: Debug {
	fn size(&mut self, ctx: &mut Context, allotted_size: Vec2) -> Vec2;

	fn draw(&self, ctx: &mut Context) -> anyhow::Result<()>;

	fn render(&mut self, ctx: &mut Context, size: Vec2) -> anyhow::Result<()> {
		self.size(ctx, size);
		self.draw(ctx)
	}
}
