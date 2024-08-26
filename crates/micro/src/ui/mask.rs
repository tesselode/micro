use std::path::Path;

use glam::Vec2;

use crate::{
	graphics::{StencilAction, StencilTest},
	with_child_fns, with_sizing_fns, Context,
};

use super::{ChildPathGenerator, Sizing, UiState, Widget};

#[derive(Debug)]
pub struct Mask {
	sizing: Sizing,
	mask_children: Vec<Box<dyn Widget>>,
	children: Vec<Box<dyn Widget>>,
}

impl Mask {
	pub fn new() -> Self {
		Self::default()
	}

	pub fn with_mask_child(mut self, child: impl Widget + 'static) -> Self {
		self.mask_children.push(Box::new(child));
		self
	}

	pub fn with_mask_children(
		mut self,
		children: impl IntoIterator<Item = impl Widget + 'static>,
	) -> Self {
		for child in children {
			self.mask_children.push(Box::new(child));
		}
		self
	}

	with_child_fns!();
	with_sizing_fns!();
}

impl Default for Mask {
	fn default() -> Self {
		Self {
			sizing: Sizing::MIN,
			mask_children: Default::default(),
			children: Default::default(),
		}
	}
}

impl Widget for Mask {
	fn name(&self) -> &'static str {
		"mask"
	}

	fn size(
		&mut self,
		ctx: &mut Context,
		state: &mut UiState,
		path: &Path,
		allotted_size: Vec2,
	) -> Vec2 {
		let mut child_path_generator = ChildPathGenerator::new();
		let allotted_size_for_children = self.sizing.allotted_size_for_children(allotted_size);
		let mut child_sizes = self
			.children
			.iter_mut()
			.map(|child| {
				let child_path = path.join(child_path_generator.generate(child.name()));
				child.size(ctx, state, &child_path, allotted_size_for_children)
			})
			.collect::<Vec<_>>();
		let mut mask_child_path_generator = ChildPathGenerator::new();
		child_sizes.extend(self.mask_children.iter_mut().map(|child| {
			let child_path = path
				.join("mask")
				.join(mask_child_path_generator.generate(child.name()));
			child.size(ctx, state, &child_path, allotted_size_for_children)
		}));
		self.sizing
			.final_parent_size(allotted_size, child_sizes.iter().copied())
	}

	fn draw(&self, ctx: &mut Context, state: &mut UiState, path: &Path) -> anyhow::Result<()> {
		ctx.clear_stencil();
		{
			let mut mask_child_path_generator = ChildPathGenerator::new();
			let ctx = &mut ctx.write_to_stencil(StencilAction::Replace(1));
			for child in &self.mask_children {
				let child_path = path
					.join("mask")
					.join(mask_child_path_generator.generate(child.name()));
				child.draw(ctx, state, &child_path)?;
			}
		}
		{
			let mut child_path_generator = ChildPathGenerator::new();
			let ctx = &mut ctx.use_stencil(StencilTest::Equal, 1);
			for child in &self.children {
				let child_path = path.join(child_path_generator.generate(child.name()));
				child.draw(ctx, state, &child_path)?;
			}
		}
		Ok(())
	}
}
