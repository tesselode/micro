use glam::Vec2;

use crate::{
	graphics::{StencilAction, StencilTest},
	with_child_fns, Context,
};

use super::Widget;

#[derive(Debug, Default)]
pub struct Mask {
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
}

impl Widget for Mask {
	fn size(&mut self, ctx: &mut Context, max_size: Vec2) -> Vec2 {
		for child in &mut self.mask_children {
			child.size(ctx, max_size);
		}
		for child in &mut self.children {
			child.size(ctx, max_size);
		}
		max_size
	}

	fn draw(&self, ctx: &mut Context) -> anyhow::Result<()> {
		ctx.clear_stencil();
		{
			let ctx = &mut ctx.write_to_stencil(StencilAction::Replace(1));
			for child in &self.mask_children {
				child.draw(ctx)?;
			}
		}
		{
			let ctx = &mut ctx.use_stencil(StencilTest::Equal, 1);
			for child in &self.children {
				child.draw(ctx)?;
			}
		}
		Ok(())
	}
}
