use glam::Vec2;

use crate::{with_child_fns, Context};

use super::Widget;

#[derive(Debug)]
pub struct MaxSize {
	max_size: Vec2,
	children: Vec<Box<dyn Widget>>,
	size: Option<Vec2>,
}

impl MaxSize {
	pub fn new(max_size: impl Into<Vec2>) -> Self {
		Self {
			max_size: max_size.into(),
			children: vec![],
			size: None,
		}
	}

	pub fn horizontal(max_size: f32) -> Self {
		Self::new((max_size, f32::INFINITY))
	}

	pub fn vertical(max_size: f32) -> Self {
		Self::new((f32::INFINITY, max_size))
	}

	with_child_fns!();
}

impl Widget for MaxSize {
	fn size(&mut self, max_size: Vec2) -> Vec2 {
		let size = self.max_size.min(max_size);
		self.size = Some(size);
		for child in &mut self.children {
			child.size(size);
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
