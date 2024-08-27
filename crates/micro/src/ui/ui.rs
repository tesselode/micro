mod mouse_input;
mod widget_mouse_state;

use std::{collections::HashMap, path::PathBuf};

use glam::Vec2;
use indexmap::IndexMap;
use mouse_input::MouseInput;
use widget_mouse_state::{UpdateMouseStateResult, WidgetMouseState};

use crate::Context;

use super::{LayoutResult, MouseEvents, Widget};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Ui {
	mouse_input: MouseInput,
	widget_mouse_state: IndexMap<PathBuf, WidgetMouseState>,
}

impl Ui {
	pub fn new() -> Self {
		Self::default()
	}

	pub fn render<Event>(
		&mut self,
		ctx: &mut Context,
		size: Vec2,
		widget: impl Widget<Event> + 'static,
	) -> anyhow::Result<Vec<Event>> {
		let mut baked_widget = BakedWidget::new(PathBuf::new(), &widget, size);
		self.mouse_input.update(ctx);
		let mut emitted_events = vec![];
		baked_widget.use_mouse_input(
			self.mouse_input,
			&mut self.widget_mouse_state,
			&mut emitted_events,
		);
		baked_widget.draw(ctx)?;
		Ok(emitted_events)
	}
}

struct BakedWidget<'a, Event> {
	path: PathBuf,
	widget: &'a dyn Widget<Event>,
	children: Vec<BakedWidget<'a, Event>>,
	layout_result: LayoutResult,
}

impl<'a, Event> BakedWidget<'a, Event> {
	fn new(path: PathBuf, widget: &'a dyn Widget<Event>, allotted_size_from_parent: Vec2) -> Self {
		let mut children = vec![];
		let mut child_sizes = vec![];
		let mut unique_child_name_generator = UniqueChildNameGenerator::new();
		for child in widget.children() {
			let allotted_size_for_child =
				widget.allotted_size_for_next_child(allotted_size_from_parent, &child_sizes);
			let unique_name = unique_child_name_generator.generate(child.name());
			let child_path = path.join(unique_name);
			let baked_child = BakedWidget::new(child_path, child.as_ref(), allotted_size_for_child);
			child_sizes.push(baked_child.layout_result.size);
			children.push(baked_child);
		}
		let layout_result = widget.layout(allotted_size_from_parent, &child_sizes);
		Self {
			path,
			widget,
			children,
			layout_result,
		}
	}

	fn use_mouse_input(
		&mut self,
		mouse_input: MouseInput,
		widget_mouse_state: &mut IndexMap<PathBuf, WidgetMouseState>,
		emitted_events: &mut Vec<Event>,
	) {
		let MouseEvents {
			click,
			hover,
			unhover,
		} = self.widget.mouse_events();
		let UpdateMouseStateResult {
			clicked,
			hovered,
			unhovered,
		} = widget_mouse_state
			.entry(self.path.clone())
			.or_default()
			.update(mouse_input, self.layout_result.size);
		if hovered {
			emitted_events.extend(hover);
		}
		if unhovered {
			emitted_events.extend(unhover);
		}
		if clicked {
			emitted_events.extend(click);
		}
		for (child, &position) in self
			.children
			.iter_mut()
			.zip(self.layout_result.child_positions.iter())
		{
			child.use_mouse_input(
				mouse_input.translated(-position),
				widget_mouse_state,
				emitted_events,
			);
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
