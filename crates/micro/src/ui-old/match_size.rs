use glam::Vec2;

use crate::{with_child_fns, Context};

use super::Widget;

#[derive(Debug)]
pub struct MatchSize {
	children: Vec<Box<dyn Widget>>,
	sizing_child_index: Option<usize>,
}

impl MatchSize {
	pub fn new() -> Self {
		Self {
			children: vec![],
			sizing_child_index: None,
		}
	}

	pub fn with_sizing_child(mut self, child: impl Widget + 'static) -> Self {
		self.children.push(Box::new(child));
		self.sizing_child_index = Some(self.children.len() - 1);
		self
	}

	with_child_fns!();
}

impl Default for MatchSize {
	fn default() -> Self {
		Self::new()
	}
}

impl Widget for MatchSize {
	fn size(&mut self, ctx: &mut Context, allotted_size: Vec2) -> Vec2 {
		let sizing_child_index = self.sizing_child_index.expect("no sizing child set");
		let size = self.children[sizing_child_index].size(ctx, allotted_size);
		for (i, child) in self.children.iter_mut().enumerate() {
			if i == sizing_child_index {
				continue;
			}
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
