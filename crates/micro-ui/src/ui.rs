mod mouse_input;
mod widget_mouse_state;

use std::{collections::HashMap, path::PathBuf};

use indexmap::IndexMap;
use itertools::izip;
use micro::{
	color::{LinSrgb, LinSrgba},
	graphics::mesh::Mesh,
	math::{Mat4, Rect, Vec2},
	push, push_translation_2d, window_size,
};
use mouse_input::MouseInput;
use widget_mouse_state::{UpdateMouseStateResult, WidgetMouseState};

use super::{LayoutResult, Widget, WidgetMouseEvent};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Ui {
	previous_baked_widget: Option<BakedWidget>,
	mouse_input: MouseInput,
	widget_mouse_state: IndexMap<PathBuf, WidgetMouseState>,
	draw_debug_state: Option<DrawDebugState>,
}

impl Ui {
	pub fn new() -> Self {
		Self::default()
	}

	pub fn show_debug_window(&mut self, egui_ctx: &micro::egui::Context, open: &mut bool) {
		let Some(widget) = &self.previous_baked_widget else {
			return;
		};
		let mut highlighted_widget_path = None;
		micro::egui::Window::new("UI")
			.open(open)
			.scroll(true)
			.show(egui_ctx, |ui| {
				show_debug_widget_info(ui, &mut highlighted_widget_path, widget, None);
			});
		self.draw_debug_state = Some(DrawDebugState {
			highlighted_widget_path,
		});
	}

	pub fn render(&mut self, settings: RenderUiSettings, widget: impl Widget + 'static) {
		let _span = tracy_client::span!();
		push!(settings.transform);
		let size = window_size().as_vec2();
		let mut baked_widget =
			BakedWidget::new(PathBuf::new(), &widget, settings.size.unwrap_or(size));
		self.mouse_input.update(settings.transform.inverse());
		baked_widget.use_mouse_input(&widget, self.mouse_input, &mut self.widget_mouse_state);
		baked_widget.draw(&widget);
		if let Some(draw_debug_state) = self.draw_debug_state.take() {
			baked_widget.draw_debug(&draw_debug_state);
		}
		self.previous_baked_widget = Some(baked_widget);
	}
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct RenderUiSettings {
	pub size: Option<Vec2>,
	pub transform: Mat4,
}

#[derive(Debug, Clone, PartialEq)]
struct BakedWidget {
	name: &'static str,
	path: PathBuf,
	children: Vec<BakedWidget>,
	layout_result: LayoutResult,
	allotted_size_from_parent: Vec2,
}

impl BakedWidget {
	fn new(path: PathBuf, raw_widget: &dyn Widget, allotted_size_from_parent: Vec2) -> Self {
		let _span = tracy_client::span!();
		let mut children = vec![];
		let mut child_sizes = vec![];
		let mut unique_child_name_generator = UniqueChildNameGenerator::new();
		for child in raw_widget.children() {
			let allotted_size_for_child =
				raw_widget.allotted_size_for_next_child(allotted_size_from_parent, &child_sizes);
			let unique_name = unique_child_name_generator.generate(child.name());
			let child_path = path.join(unique_name);
			let baked_child = BakedWidget::new(child_path, child.as_ref(), allotted_size_for_child);
			child_sizes.push(baked_child.layout_result.size);
			children.push(baked_child);
		}
		let layout_result = raw_widget.layout(allotted_size_from_parent, &child_sizes);
		Self {
			name: raw_widget.name(),
			path,
			children,
			layout_result,
			allotted_size_from_parent,
		}
	}

	fn use_mouse_input(
		&mut self,
		raw_widget: &dyn Widget,
		mut mouse_input: MouseInput,
		widget_mouse_state: &mut IndexMap<PathBuf, WidgetMouseState>,
	) {
		let _span = tracy_client::span!();
		mouse_input =
			mouse_input.transformed(raw_widget.transform(self.layout_result.size).inverse());
		let UpdateMouseStateResult {
			hovered,
			unhovered,
			click_started,
			clicked,
		} = widget_mouse_state
			.entry(self.path.clone())
			.or_default()
			.update(mouse_input, self.layout_result.size);
		if let Some(channel) = raw_widget.mouse_event_channel() {
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
		for (raw_child, baked_child, position) in izip!(
			raw_widget.children(),
			&mut self.children,
			&self.layout_result.child_positions
		) {
			baked_child.use_mouse_input(
				raw_child.as_ref(),
				mouse_input.translated(-position),
				widget_mouse_state,
			);
		}
	}

	fn draw(&self, raw_widget: &dyn Widget) {
		let _span = tracy_client::span!();
		push!(raw_widget.transform(self.layout_result.size));
		raw_widget.draw_before_children(self.layout_result.size);
		for (raw_child, baked_child, position) in izip!(
			raw_widget.children(),
			&self.children,
			self.layout_result.child_positions.iter().copied()
		) {
			push_translation_2d!(position.round());
			baked_child.draw(raw_child.as_ref());
		}
		raw_widget.draw_after_children(self.layout_result.size);
	}

	fn draw_debug(&self, draw_debug_state: &DrawDebugState) {
		if draw_debug_state
			.highlighted_widget_path
			.as_ref()
			.is_some_and(|path| *path == self.path)
		{
			Mesh::rectangle(Rect::new(Vec2::ZERO, self.layout_result.size))
				.color(LinSrgba::new(1.0, 1.0, 0.0, 0.25))
				.draw();
		}
		Mesh::outlined_rectangle(2.0, Rect::new(Vec2::ZERO, self.layout_result.size))
			.color(LinSrgb::new(1.0, 0.0, 1.0))
			.draw();
		for (baked_child, position) in self
			.children
			.iter()
			.zip(self.layout_result.child_positions.iter().copied())
		{
			push_translation_2d!(position.round());
			baked_child.draw_debug(draw_debug_state);
		}
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct DrawDebugState {
	highlighted_widget_path: Option<PathBuf>,
}

fn show_debug_widget_info(
	ui: &mut micro::egui::Ui,
	highlighted_widget_path: &mut Option<PathBuf>,
	widget: &BakedWidget,
	position: Option<Vec2>,
) {
	let label = widget
		.path
		.file_name()
		.map(|name| name.to_str().unwrap().to_string())
		.unwrap_or_else(|| "Root".to_string());
	let response = ui.collapsing(label, |ui| {
		ui.horizontal(|ui| {
			ui.label("Allotted size from parent:");
			ui.monospace(format!("{}", widget.allotted_size_from_parent));
		});
		ui.horizontal(|ui| {
			ui.label("Size:");
			ui.monospace(format!("{}", widget.layout_result.size));
		});
		if let Some(position) = position {
			ui.horizontal(|ui| {
				ui.label("Position:");
				ui.monospace(format!("{}", position));
			});
		}
		for (i, child) in widget.children.iter().enumerate() {
			show_debug_widget_info(
				ui,
				highlighted_widget_path,
				child,
				Some(widget.layout_result.child_positions[i]),
			);
		}
	});
	if response.header_response.contains_pointer() {
		*highlighted_widget_path = Some(widget.path.clone());
	}
}
