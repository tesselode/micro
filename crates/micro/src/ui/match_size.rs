use glam::{vec2, Vec2};

use crate::{with_child_fns, Context};

use super::Widget;

#[derive(Debug)]
pub struct MatchSize {
	match_x: bool,
	match_y: bool,
	children: Vec<Box<dyn Widget>>,
	sizing_child_index: Option<usize>,
}

impl MatchSize {
	pub fn both_axes() -> Self {
		Self {
			match_x: true,
			match_y: true,
			children: vec![],
			sizing_child_index: None,
		}
	}

	pub fn horizontal() -> Self {
		Self {
			match_x: true,
			match_y: false,
			children: vec![],
			sizing_child_index: None,
		}
	}

	pub fn vertical() -> Self {
		Self {
			match_x: false,
			match_y: true,
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
		Self::both_axes()
	}
}

impl Widget for MatchSize {
	fn size(&mut self, ctx: &mut Context, max_size: Vec2) -> Vec2 {
		let sizing_child_index = self.sizing_child_index.expect("no sizing child set");
		let sizing_child_size = self.children[sizing_child_index].size(ctx, max_size);
		let size = vec2(
			if self.match_x {
				sizing_child_size.x
			} else {
				max_size.x
			},
			if self.match_y {
				sizing_child_size.y
			} else {
				max_size.y
			},
		);
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
