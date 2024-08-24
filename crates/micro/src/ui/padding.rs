use glam::{vec2, Vec2};

use crate::{with_child_fns, Context};

use super::Widget;

#[derive(Debug, Default)]
pub struct Padding {
	left: f32,
	top: f32,
	right: f32,
	bottom: f32,
	children: Vec<Box<dyn Widget>>,
	shrink_wrap: bool,
	size: Option<Vec2>,
}

impl Padding {
	pub fn new(left: f32, top: f32, right: f32, bottom: f32) -> Self {
		Self {
			left,
			top,
			right,
			bottom,
			children: vec![],
			shrink_wrap: false,
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

	pub fn shrink_wrap(self) -> Self {
		Self {
			shrink_wrap: true,
			..self
		}
	}

	with_child_fns!();
}

impl Widget for Padding {
	fn size(&mut self, ctx: &mut Context, max_size: Vec2) -> Vec2 {
		let child_max_size = max_size - vec2(self.left + self.right, self.top + self.bottom);
		let shrink_wrap_size = self
			.children
			.iter_mut()
			.map(|child| child.size(ctx, child_max_size))
			.reduce(Vec2::max)
			.unwrap_or_default()
			+ vec2(self.left + self.right, self.top + self.bottom);
		let size = match self.shrink_wrap {
			true => shrink_wrap_size,
			false => max_size,
		};
		self.size = Some(size);
		size
	}

	fn draw(&self, ctx: &mut Context) -> anyhow::Result<()> {
		let ctx = &mut ctx.push_translation_2d(vec2(self.left, self.top));
		for child in &self.children {
			child.draw(ctx)?;
		}
		Ok(())
	}
}
