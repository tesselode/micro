use glam::{vec2, Vec2};

use crate::{with_child_fns, with_sizing_fns, Context};

use super::{Sizing, Widget};

#[derive(Debug)]
pub struct Padding {
	sizing: Sizing,
	left: f32,
	top: f32,
	right: f32,
	bottom: f32,
	children: Vec<Box<dyn Widget>>,
	size: Option<Vec2>,
}

impl Padding {
	pub fn new(left: f32, top: f32, right: f32, bottom: f32) -> Self {
		Self {
			sizing: Sizing::MIN,
			left,
			top,
			right,
			bottom,
			children: vec![],
			size: None,
		}
	}

	pub fn all(padding: f32) -> Self {
		Self::new(padding, padding, padding, padding)
	}

	pub fn symmetric(padding: Vec2) -> Self {
		Self::new(padding.x, padding.y, padding.x, padding.y)
	}

	pub fn horizontal(padding: f32) -> Self {
		Self::symmetric(vec2(padding, 0.0))
	}

	pub fn vertical(padding: f32) -> Self {
		Self::symmetric(vec2(0.0, padding))
	}

	pub fn left(padding: f32) -> Self {
		Self::new(padding, 0.0, 0.0, 0.0)
	}

	pub fn top(padding: f32) -> Self {
		Self::new(0.0, padding, 0.0, 0.0)
	}

	pub fn right(padding: f32) -> Self {
		Self::new(0.0, 0.0, padding, 0.0)
	}

	pub fn bottom(padding: f32) -> Self {
		Self::new(0.0, 0.0, 0.0, padding)
	}

	with_child_fns!();
	with_sizing_fns!();
}

impl Default for Padding {
	fn default() -> Self {
		Self {
			sizing: Sizing::MIN,
			left: Default::default(),
			top: Default::default(),
			right: Default::default(),
			bottom: Default::default(),
			children: Default::default(),
			size: Default::default(),
		}
	}
}

impl Widget for Padding {
	fn size(&mut self, ctx: &mut Context, allotted_size: Vec2) -> Vec2 {
		let total_padding = vec2(self.left + self.right, self.top + self.bottom);
		let allotted_size_for_children =
			self.sizing.allotted_size_for_children(allotted_size) - total_padding;
		let child_sizes = self
			.children
			.iter_mut()
			.map(|child| child.size(ctx, allotted_size_for_children));
		let parent_size = self.sizing.final_parent_size(allotted_size, child_sizes) + total_padding;
		self.size = Some(parent_size);
		parent_size
	}

	fn draw(&self, ctx: &mut Context) -> anyhow::Result<()> {
		let ctx = &mut ctx.push_translation_2d(vec2(self.left, self.top));
		for child in &self.children {
			child.draw(ctx)?;
		}
		Ok(())
	}
}
