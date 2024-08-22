mod align;
mod axis;
mod cross_sizing;
mod ellipse;
mod fractional_max_size;
mod image;
mod max_size;
mod padding;
mod polygon;
mod polyline;
mod rectangle;
mod stack;
mod text;
mod transform;

pub use align::*;
pub use axis::*;
pub use cross_sizing::*;
pub use ellipse::*;
pub use fractional_max_size::*;
pub use image::*;
pub use max_size::*;
pub use padding::*;
pub use polygon::*;
pub use polyline::*;
pub use rectangle::*;
pub use stack::*;
pub use text::{TextSettings, TextShadow, TextSizing, TextWidget as Text};
pub use transform::*;

use std::fmt::Debug;

use glam::Vec2;

use crate::Context;

pub trait Widget: Debug {
	fn size(&mut self, ctx: &mut Context, max_size: Vec2) -> Vec2;

	fn draw(&self, ctx: &mut Context) -> anyhow::Result<()>;

	fn render(&mut self, ctx: &mut Context, size: Vec2) -> anyhow::Result<()> {
		self.size(ctx, size);
		self.draw(ctx)
	}
}

#[macro_export]
macro_rules! with_child_fns {
	() => {
		pub fn with_child(mut self, child: impl Widget + 'static) -> Self {
			self.children.push(Box::new(child));
			self
		}

		pub fn with_children(
			mut self,
			children: impl IntoIterator<Item = impl Widget + 'static>,
		) -> Self {
			for child in children {
				self.children.push(Box::new(child));
			}
			self
		}
	};
}
