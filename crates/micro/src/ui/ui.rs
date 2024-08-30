mod mouse_input;
mod widget_mouse_state;

use std::{collections::HashMap, path::PathBuf};

use glam::Vec2;
use indexmap::IndexMap;
use mouse_input::MouseInput;
use widget_mouse_state::{UpdateMouseStateResult, WidgetMouseState};

use crate::{
	graphics::{StencilAction, StencilTest},
	Context,
};

use super::{LayoutResult, Widget, WidgetMouseEvent};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Ui {
	mouse_input: MouseInput,
	widget_mouse_state: IndexMap<PathBuf, WidgetMouseState>,
}

impl Ui {
	pub fn new() -> Self {
		Self::default()
	}

	pub fn render(
		&mut self,
		ctx: &mut Context,
		size: Vec2,
		widget: impl Widget + 'static,
	) -> anyhow::Result<()> {
		let mut baked_widget = BakedWidget::new(ctx, PathBuf::new(), &widget, size);
		self.mouse_input.update(ctx);
		baked_widget.use_mouse_input(self.mouse_input, &mut self.widget_mouse_state);
		baked_widget.draw(ctx)?;
		Ok(())
	}
}

struct BakedWidget<'a> {
	path: PathBuf,
	widget: &'a dyn Widget,
	children: Vec<BakedWidget<'a>>,
	mask: Option<Box<BakedWidget<'a>>>,
	layout_result: LayoutResult,
}

impl<'a> BakedWidget<'a> {
	fn new(
		ctx: &mut Context,
		path: PathBuf,
		widget: &'a dyn Widget,
		allotted_size_from_parent: Vec2,
	) -> Self {
		let mut children = vec![];
		let mut child_sizes = vec![];
		let mut unique_child_name_generator = UniqueChildNameGenerator::new();
		for child in widget.children() {
			let allotted_size_for_child =
				widget.allotted_size_for_next_child(allotted_size_from_parent, &child_sizes);
			let unique_name = unique_child_name_generator.generate(child.name());
			let child_path = path.join(unique_name);
			let baked_child =
				BakedWidget::new(ctx, child_path, child.as_ref(), allotted_size_for_child);
			child_sizes.push(baked_child.layout_result.size);
			children.push(baked_child);
		}
		let layout_result = widget.layout(ctx, allotted_size_from_parent, &child_sizes);
		let mask = widget.mask().map(|mask| {
			Box::new(BakedWidget::new(
				ctx,
				path.join("mask"),
				mask,
				layout_result.size,
			))
		});
		Self {
			path,
			widget,
			children,
			mask,
			layout_result,
		}
	}

	fn use_mouse_input(
		&mut self,
		mut mouse_input: MouseInput,
		widget_mouse_state: &mut IndexMap<PathBuf, WidgetMouseState>,
	) {
		mouse_input =
			mouse_input.transformed(self.widget.transform(self.layout_result.size).inverse());
		let UpdateMouseStateResult {
			hovered,
			unhovered,
			click_started,
			clicked,
		} = widget_mouse_state
			.entry(self.path.clone())
			.or_default()
			.update(mouse_input, self.layout_result.size);
		if let Some(channel) = self.widget.mouse_event_channel() {
			if hovered {
				channel.push(WidgetMouseEvent::Hovered);
			}
			if unhovered {
				channel.push(WidgetMouseEvent::Unhovered);
			}
			if click_started {
				channel.push(WidgetMouseEvent::ClickStarted);
			}
			if clicked {
				channel.push(WidgetMouseEvent::Clicked);
			}
		}
		for (child, &position) in self
			.children
			.iter_mut()
			.zip(self.layout_result.child_positions.iter())
		{
			child.use_mouse_input(mouse_input.translated(-position), widget_mouse_state);
		}
	}

	fn draw(&self, ctx: &mut Context) -> anyhow::Result<()> {
		let ctx = &mut ctx.push_transform(self.widget.transform(self.layout_result.size));
		if let Some(mask) = &self.mask {
			{
				let ctx = &mut ctx.write_to_stencil(StencilAction::Replace(1));
				mask.draw(ctx)?;
			}
			{
				let ctx = &mut ctx.use_stencil(StencilTest::Equal, 1);
				self.draw_non_mask_contents(ctx)?;
			}
		} else {
			self.draw_non_mask_contents(ctx)?;
		}
		Ok(())
	}

	fn draw_non_mask_contents(&self, ctx: &mut Context) -> anyhow::Result<()> {
		self.widget.draw(ctx, self.layout_result.size)?;
		for (child, position) in self
			.children
			.iter()
			.zip(self.layout_result.child_positions.iter().copied())
		{
			let ctx = &mut ctx.push_translation_2d(position.round());
			child.draw(ctx)?;
		}
		Ok(())
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct UniqueChildNameGenerator {
	name_counts: HashMap<&'static str, usize>,
}

impl UniqueChildNameGenerator {
	fn new() -> Self {
		Self::default()
	}

	fn generate(&mut self, name: &'static str) -> String {
		let count = self.name_counts.entry(name).or_default();
		let unique_name = format!("{}{}", name, *count);
		*count += 1;
		unique_name
	}
}
