use glam::Vec2;

use crate::{with_child_fns, Context};

use super::Widget;

#[derive(Debug)]
pub struct FractionalMaxSize {
	max_size: Vec2,
	children: Vec<Box<dyn Widget>>,
	size: Option<Vec2>,
}

impl FractionalMaxSize {
	pub fn new(max_size: impl Into<Vec2>) -> Self {
		Self {
			max_size: max_size.into(),
			children: vec![],
			size: None,
		}
	}

	pub fn horizontal(max_size: f32) -> Self {
		Self::new((max_size, 1.0))
	}

	pub fn vertical(max_size: f32) -> Self {
		Self::new((1.0, max_size))
	}

	with_child_fns!();
}

impl Widget for FractionalMaxSize {
	fn size(&mut self, ctx: &mut Context, max_size: Vec2) -> Vec2 {
		let size = max_size * self.max_size;
		self.size = Some(size);
		for child in &mut self.children {
			child.size(ctx, size);
		}
		size
	}

	fn draw(&self, ctx: &mut Context) -> anyhow::Result<()> {
		for child in &self.children {
			child.draw(ctx)?;
		}
		Ok(())
	}
}
