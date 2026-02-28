use std::{collections::HashMap, path::PathBuf};

use crate::{WidgetInspector, WidgetInspectorInner, mouse_input::MouseInput};

use itertools::izip;
use micro::{
	Context,
	color::{LinSrgb, LinSrgba},
	graphics::{CompareFunction, StencilOperation, StencilState, mesh::Mesh},
	math::{Mat4, Rect, Vec2},
};

use super::{LayoutResult, Widget};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Ui {
	previous_baked_widget: Option<BakedWidget>,
	mouse_input: MouseInput,
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

	pub fn render(
		&mut self,
		ctx: &mut Context,
		settings: RenderUiSettings,
		widget: impl Widget + 'static,
	) {
		let _span = tracy_client::span!();
		let ctx = &mut ctx.push(settings.transform);
		let default_size = ctx.window_size().as_vec2();
		let mut baked_widget = BakedWidget::new(
			ctx,
			PathBuf::new(),
			&widget,
			settings.size.unwrap_or(default_size),
		);
		self.mouse_input.update(ctx, settings.transform.inverse());
		baked_widget.use_mouse_input(&widget, self.mouse_input.clone());
		baked_widget.draw(ctx, &widget);
		if let Some(draw_debug_state) = self.draw_debug_state.take() {
			baked_widget.draw_debug(ctx, &draw_debug_state);
		}
		baked_widget.report(Vec2::ZERO, Vec2::ZERO, Mat4::IDENTITY);
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
	transform: Mat4,
	mask: Option<Box<BakedWidget>>,
	inspector: Option<WidgetInspector>,
	layout_result: LayoutResult,
	allotted_size_from_parent: Vec2,
}

impl BakedWidget {
	fn new(
		ctx: &mut Context,
		path: PathBuf,
		raw_widget: &dyn Widget,
		allotted_size_from_parent: Vec2,
	) -> Self {
		let _span = tracy_client::span!();
		let mut children = vec![];
		let mut child_sizes = vec![];
		let mut unique_child_name_generator = UniqueChildNameGenerator::new();
		for child in raw_widget.children() {
			let allotted_size_for_child =
				raw_widget.allotted_size_for_next_child(allotted_size_from_parent, &child_sizes);
			let unique_name = unique_child_name_generator.generate(child.name());
			let child_path = path.join(unique_name);
			let baked_child =
				BakedWidget::new(ctx, child_path, child.as_ref(), allotted_size_for_child);
			child_sizes.push(baked_child.layout_result.size);
			children.push(baked_child);
		}
		let layout_result = raw_widget.layout(ctx, allotted_size_from_parent, &child_sizes);
		let mask = raw_widget.mask().map(|mask| {
			Box::new(BakedWidget::new(
				ctx,
				path.join("mask"),
				mask,
				layout_result.size,
			))
		});
		let transform = raw_widget.transform(layout_result.size);
		Self {
			name: raw_widget.name(),
			path,
			children,
			transform,
			mask,
			inspector: raw_widget.inspector(),
			layout_result,
			allotted_size_from_parent,
		}
	}

	fn use_mouse_input(&mut self, raw_widget: &dyn Widget, mut mouse_input: MouseInput) {
		let _span = tracy_client::span!();
		mouse_input = mouse_input.transformed(self.transform.inverse());
		if let Some(mouse_state) = raw_widget.mouse_state() {
			mouse_state.update(&mouse_input, self.layout_result.size);
		}
		for (raw_child, baked_child, position) in izip!(
			raw_widget.children(),
			&mut self.children,
			&self.layout_result.child_positions
		) {
			baked_child.use_mouse_input(raw_child.as_ref(), mouse_input.translated(-position));
		}
	}

	fn draw(&self, ctx: &mut Context, raw_widget: &dyn Widget) {
		let _span = tracy_client::span!();
		let ctx = &mut ctx.push(self.transform);
		if let Some((raw_mask, baked_mask)) = raw_widget.mask().zip(self.mask.as_ref()) {
			{
				let ctx = &mut ctx.push(StencilState::write(StencilOperation::Replace, 1));
				baked_mask.draw(ctx, raw_mask);
			}
			{
				let ctx = &mut ctx.push(StencilState::read(CompareFunction::Equal, 1));
				self.draw_non_mask_contents(ctx, raw_widget);
			}
		} else {
			self.draw_non_mask_contents(ctx, raw_widget);
		}
	}

	fn draw_non_mask_contents(&self, ctx: &mut Context, raw_widget: &dyn Widget) {
		raw_widget.draw_before_children(ctx, self.layout_result.size);
		for (raw_child, baked_child, position) in izip!(
			raw_widget.children(),
			&self.children,
			self.layout_result.child_positions.iter().copied()
		) {
			let ctx = &mut ctx.push_translation_2d(position);
			baked_child.draw(ctx, raw_child.as_ref());
		}
		raw_widget.draw_after_children(ctx, self.layout_result.size);
	}

	fn draw_debug(&self, ctx: &mut Context, draw_debug_state: &DrawDebugState) {
		if draw_debug_state
			.highlighted_widget_path
			.as_ref()
			.is_some_and(|path| *path == self.path)
		{
			Mesh::rectangle(ctx, Rect::new(Vec2::ZERO, self.layout_result.size))
				.color(LinSrgba::new(1.0, 1.0, 0.0, 0.25))
				.draw(ctx);
		}
		Mesh::outlined_rectangle(ctx, 2.0, Rect::new(Vec2::ZERO, self.layout_result.size))
			.color(LinSrgb::new(1.0, 0.0, 1.0))
			.draw(ctx);
		for (baked_child, position) in self
			.children
			.iter()
			.zip(self.layout_result.child_positions.iter().copied())
		{
			let ctx = &mut ctx.push_translation_2d(position.round());
			baked_child.draw_debug(ctx, draw_debug_state);
		}
	}

	fn report(&self, parent_global_top_left: Vec2, my_offset: Vec2, parent_global_transform: Mat4) {
		let my_global_top_left = parent_global_top_left + my_offset;
		let my_global_transform = parent_global_transform
			* Mat4::from_translation(my_offset.extend(0.0))
			* self.transform;
		if let Some(inspector) = &self.inspector {
			*inspector.0.borrow_mut() = WidgetInspectorInner::Inspected {
				bounds: Rect::new(my_global_top_left, self.layout_result.size),
				transform: my_global_transform,
			};
		}
		for (baked_child, child_offset) in self
			.children
			.iter()
			.zip(self.layout_result.child_positions.iter().copied())
		{
			baked_child.report(my_global_top_left, child_offset, my_global_transform);
		}
		if let Some(mask) = &self.mask {
			mask.report(parent_global_top_left, Vec2::ZERO, my_global_transform);
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
