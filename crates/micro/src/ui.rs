mod align;
mod ellipse;
mod image;
mod mask;
mod match_size;
mod padding;
mod polygon;
mod polyline;
mod rectangle;
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
pub use stack::*;
pub use text::{TextSettings, TextShadow, TextSizing, TextWidget as Text};
pub use transform::*;

use std::fmt::Debug;

use glam::{vec2, Vec2};

use crate::Context;

pub trait Widget: Debug {
	fn size(&mut self, ctx: &mut Context, allotted_size: Vec2) -> Vec2;

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

#[macro_export]
macro_rules! with_sizing_fns {
	() => {
		pub fn with_sizing(self, sizing: $crate::ui::Sizing) -> Self {
			Self { sizing, ..self }
		}

		pub fn with_horizontal_sizing(mut self, sizing: $crate::ui::AxisSizing) -> Self {
			self.sizing.horizontal = sizing;
			self
		}

		pub fn with_vertical_sizing(mut self, sizing: $crate::ui::AxisSizing) -> Self {
			self.sizing.vertical = sizing;
			self
		}

		pub fn with_max_size(self, size: impl Into<Vec2>) -> Self {
			let size: Vec2 = size.into();
			Self {
				sizing: $crate::ui::Sizing {
					horizontal: $crate::ui::AxisSizing::Max(size.x),
					vertical: $crate::ui::AxisSizing::Max(size.y),
				},
				..self
			}
		}

		pub fn with_fractional_size(self, fraction: impl Into<Vec2>) -> Self {
			let fraction: Vec2 = fraction.into();
			Self {
				sizing: $crate::ui::Sizing {
					horizontal: $crate::ui::AxisSizing::Fractional(fraction.x),
					vertical: $crate::ui::AxisSizing::Fractional(fraction.y),
				},
				..self
			}
		}
	};
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Sizing {
	pub horizontal: AxisSizing,
	pub vertical: AxisSizing,
}

impl Sizing {
	pub const MIN: Self = Self {
		horizontal: AxisSizing::Shrink,
		vertical: AxisSizing::Shrink,
	};
	pub const MAX: Self = Self {
		horizontal: AxisSizing::Expand,
		vertical: AxisSizing::Expand,
	};

	pub fn allotted_size_for_children(self, allotted_size_for_parent: Vec2) -> Vec2 {
		vec2(
			self.horizontal
				.allotted_size_for_children(allotted_size_for_parent.x),
			self.vertical
				.allotted_size_for_children(allotted_size_for_parent.y),
		)
	}

	pub fn final_parent_size(
		self,
		allotted_size_for_parent: Vec2,
		child_sizes: impl Iterator<Item = Vec2>,
	) -> Vec2 {
		let child_max_size = child_sizes.reduce(Vec2::max).unwrap_or_default();
		vec2(
			match self.horizontal {
				AxisSizing::Shrink => child_max_size.x,
				AxisSizing::Expand => allotted_size_for_parent.x,
				AxisSizing::Max(size) => size.min(allotted_size_for_parent.x),
				AxisSizing::Fractional(fraction) => fraction * allotted_size_for_parent.x,
			},
			match self.vertical {
				AxisSizing::Shrink => child_max_size.y,
				AxisSizing::Expand => allotted_size_for_parent.y,
				AxisSizing::Max(size) => size.min(allotted_size_for_parent.y),
				AxisSizing::Fractional(fraction) => fraction * allotted_size_for_parent.y,
			},
		)
	}
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AxisSizing {
	Shrink,
	Expand,
	Max(f32),
	Fractional(f32),
}

impl AxisSizing {
	pub fn allotted_size_for_children(self, allotted_size_for_parent: f32) -> f32 {
		match self {
			AxisSizing::Shrink => allotted_size_for_parent,
			AxisSizing::Expand => allotted_size_for_parent,
			AxisSizing::Max(size) => size.min(allotted_size_for_parent),
			AxisSizing::Fractional(fraction) => allotted_size_for_parent * fraction,
		}
	}

	pub fn final_parent_size(
		self,
		allotted_size_for_parent: f32,
		child_sizes: impl Iterator<Item = f32>,
	) -> f32 {
		match self {
			AxisSizing::Shrink => child_sizes.reduce(f32::max).unwrap_or_default(),
			AxisSizing::Expand => allotted_size_for_parent,
			AxisSizing::Max(size) => size.min(allotted_size_for_parent),
			AxisSizing::Fractional(fraction) => allotted_size_for_parent * fraction,
		}
	}
}
