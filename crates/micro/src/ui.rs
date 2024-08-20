mod axis;
mod cross_sizing;
mod max_size;
mod padding;
mod rectangle;
mod stack;

pub use axis::*;
pub use cross_sizing::*;
pub use max_size::*;
pub use padding::*;
pub use rectangle::*;
pub use stack::*;

use std::fmt::Debug;

use glam::Vec2;

use crate::Context;

pub trait Widget: Debug {
	fn size(&mut self, max_size: Vec2) -> Vec2;

	fn draw(&self, ctx: &mut Context) -> anyhow::Result<()>;
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
