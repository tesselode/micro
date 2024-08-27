use glam::Vec2;

use crate::Context;

use super::{LayoutResult, Widget};

pub struct Ui;

impl Ui {
	pub fn render(
		&mut self,
		ctx: &mut Context,
		size: Vec2,
		widget: impl Widget + 'static,
	) -> anyhow::Result<()> {
		let baked_widget = BakedWidget::new(&widget, size);
		baked_widget.draw(ctx)?;
		Ok(())
	}
}

struct BakedWidget<'a> {
	widget: &'a dyn Widget,
	children: Vec<BakedWidget<'a>>,
	layout_result: LayoutResult,
}

impl<'a> BakedWidget<'a> {
	fn new(widget: &'a dyn Widget, allotted_size_from_parent: Vec2) -> Self {
		let mut children = vec![];
		let mut child_sizes = vec![];
		for child in widget.children() {
			let allotted_size_for_child =
				widget.allotted_size_for_next_child(allotted_size_from_parent, &child_sizes);
			let baked_child = BakedWidget::new(child.as_ref(), allotted_size_for_child);
			child_sizes.push(baked_child.layout_result.size);
			children.push(baked_child);
		}
		let layout_result = widget.layout(allotted_size_from_parent, &child_sizes);
		Self {
			widget,
			children,
			layout_result,
		}
	}

	fn draw(&self, ctx: &mut Context) -> anyhow::Result<()> {
		self.widget.draw(ctx, self.layout_result.size)?;
		for (child, position) in self
			.children
			.iter()
			.zip(self.layout_result.child_positions.iter().copied())
		{
			let ctx = &mut ctx.push_translation_2d(position);
			child.draw(ctx)?;
		}
		Ok(())
	}
}
