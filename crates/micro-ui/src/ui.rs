use std::collections::HashMap;

use crate::{WidgetInspector, WidgetState, mouse_input::MouseInput};

use indexmap::IndexMap;
use itertools::izip;
use micro::{
	Context,
	color::{LinSrgb, LinSrgba},
	graphics::{CompareFunction, StencilOperation, StencilState, mesh::Mesh},
	input::MouseButton,
	math::{Mat4, Rect, Vec2},
};

use super::{LayoutResult, Widget};

#[derive(Debug, Default)]
pub struct Ui {
	widget_state: IndexMap<String, WidgetState>,
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
				show_debug_widget_info(
					ui,
					&mut highlighted_widget_path,
					widget,
					None,
					&self.widget_state,
				);
			});
		self.draw_debug_state = Some(DrawDebugState {
			highlighted_widget_id: highlighted_widget_path,
		});
	}

	pub fn render(
		&mut self,
		ctx: &mut Context,
		settings: RenderUiSettings,
		widget: impl Widget + 'static,
	) {
		let _span = tracy_client::span!();

		// mark all states as unused
		for state in self.widget_state.values_mut() {
			state.used = false;
		}

		// bake the root widget
		let ctx = &mut ctx.push(settings.transform);
		let default_size = ctx.window_size().as_vec2();
		let mut baked_widget = BakedWidget::new(
			ctx,
			widget.id().unwrap_or_else(|| "root".to_string()),
			Box::new(widget),
			settings.size.unwrap_or(default_size),
			&mut self.widget_state,
		);

		// mouse input
		self.mouse_input.update(ctx, settings.transform.inverse());
		baked_widget.use_mouse_input(self.mouse_input.clone(), &mut self.widget_state);

		// draw
		baked_widget.draw(ctx, &mut self.widget_state);

		// draw debug
		if let Some(draw_debug_state) = self.draw_debug_state.take() {
			baked_widget.draw_debug(ctx, &draw_debug_state);
		}

		// report bounds and transforms
		baked_widget.report(
			Vec2::ZERO,
			Vec2::ZERO,
			Mat4::IDENTITY,
			&mut self.widget_state,
		);

		// save baked widget for debugging
		self.previous_baked_widget = Some(baked_widget);

		// remove unused widget states
		self.widget_state.retain(|_, state| state.used);
	}

	pub fn state(&self, id: impl AsRef<str>) -> Option<&WidgetState> {
		self.widget_state.get(id.as_ref())
	}
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct RenderUiSettings {
	pub size: Option<Vec2>,
	pub transform: Mat4,
}

#[derive(Debug)]
struct BakedWidget {
	id: String,
	raw: Box<dyn Widget>,
	inspector: Option<WidgetInspector>,
	children: Vec<BakedWidget>,
	transform: Mat4,
	mask: Option<Box<BakedWidget>>,
	layout_result: LayoutResult,
	allotted_size_from_parent: Vec2,
}

impl BakedWidget {
	fn new(
		ctx: &mut Context,
		id: String,
		mut raw: Box<dyn Widget>,
		allotted_size_from_parent: Vec2,
		widget_state: &mut IndexMap<String, WidgetState>,
	) -> Self {
		let _span = tracy_client::span!();
		let mut children = vec![];
		let mut child_sizes = vec![];
		let mut unique_child_id_generator = UniqueChildIdGenerator::new();

		// bake children
		for child in raw.children(ctx, widget_state.entry(id.clone()).or_default()) {
			let allotted_size_for_child = raw.allotted_size_for_next_child(
				ctx,
				allotted_size_from_parent,
				&child_sizes,
				widget_state.entry(id.clone()).or_default(),
			);
			let child_id = child.id().unwrap_or_else(|| {
				let child_id_component = unique_child_id_generator.generate(child.name());
				format!("{}/{}", id, child_id_component)
			});
			let baked_child =
				BakedWidget::new(ctx, child_id, child, allotted_size_for_child, widget_state);
			child_sizes.push(baked_child.layout_result.size);
			children.push(baked_child);
		}
		let layout_result = raw.layout(
			ctx,
			allotted_size_from_parent,
			&child_sizes,
			widget_state.entry(id.clone()).or_default(),
		);

		// bake mask
		let raw_mask = raw.mask(ctx, widget_state.entry(id.clone()).or_default());
		let mask = raw_mask.map(|mask| {
			Box::new(BakedWidget::new(
				ctx,
				mask.id().unwrap_or_else(|| format!("{}/{}", id, "mask")),
				mask,
				layout_result.size,
				widget_state,
			))
		});

		let inspector = raw.inspector();
		let transform = raw.transform(
			ctx,
			layout_result.size,
			widget_state.entry(id.clone()).or_default(),
		);

		Self {
			id,
			inspector,
			raw,
			children,
			transform,
			mask,
			layout_result,
			allotted_size_from_parent,
		}
	}

	fn use_mouse_input(
		&mut self,
		mut mouse_input: MouseInput,
		widget_state: &mut IndexMap<String, WidgetState>,
	) {
		let _span = tracy_client::span!();
		mouse_input = mouse_input.transformed(self.transform.inverse());
		widget_state
			.entry(self.id.clone())
			.or_default()
			.update_mouse_state(&mouse_input, self.layout_result.size);
		for (baked_child, position) in
			izip!(&mut self.children, &self.layout_result.child_positions)
		{
			baked_child.use_mouse_input(mouse_input.translated(-position), widget_state);
		}
	}

	fn draw(&mut self, ctx: &mut Context, widget_state: &mut IndexMap<String, WidgetState>) {
		let _span = tracy_client::span!();
		let ctx = &mut ctx.push(self.transform);
		if let Some(baked_mask) = &mut self.mask {
			{
				let ctx = &mut ctx.push(StencilState::write(StencilOperation::Replace, 1));
				baked_mask.draw(ctx, widget_state);
			}
			{
				let ctx = &mut ctx.push(StencilState::read(CompareFunction::Equal, 1));
				self.draw_non_mask_contents(ctx, widget_state);
			}
		} else {
			self.draw_non_mask_contents(ctx, widget_state);
		}
	}

	fn draw_non_mask_contents(
		&mut self,
		ctx: &mut Context,
		widget_state: &mut IndexMap<String, WidgetState>,
	) {
		self.raw.draw_before_children(
			ctx,
			self.layout_result.size,
			widget_state.entry(self.id.clone()).or_default(),
		);
		for (child, position) in self
			.children
			.iter_mut()
			.zip(self.layout_result.child_positions.iter().copied())
		{
			let ctx = &mut ctx.push_translation_2d(position);
			child.draw(ctx, widget_state);
		}
		self.raw.draw_after_children(
			ctx,
			self.layout_result.size,
			widget_state.entry(self.id.clone()).or_default(),
		);
	}

	fn draw_debug(&self, ctx: &mut Context, draw_debug_state: &DrawDebugState) {
		let ctx = &mut ctx.push(self.transform);
		if draw_debug_state
			.highlighted_widget_id
			.as_ref()
			.is_some_and(|id| *id == self.id)
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

	fn report(
		&self,
		parent_global_top_left: Vec2,
		my_offset: Vec2,
		parent_global_transform: Mat4,
		widget_state: &mut IndexMap<String, WidgetState>,
	) {
		let my_global_top_left = parent_global_top_left + my_offset;
		let my_global_transform = parent_global_transform
			* Mat4::from_translation(my_offset.extend(0.0))
			* self.transform;
		{
			let my_widget_state = widget_state.entry(self.id.clone()).or_default();
			my_widget_state.used = true;
			my_widget_state.bounds = Some(Rect::new(my_global_top_left, self.layout_result.size));
			my_widget_state.transform = Some(my_global_transform);
		}
		for (baked_child, child_offset) in self
			.children
			.iter()
			.zip(self.layout_result.child_positions.iter().copied())
		{
			baked_child.report(
				my_global_top_left,
				child_offset,
				my_global_transform,
				widget_state,
			);
		}
		if let Some(mask) = &self.mask {
			mask.report(
				parent_global_top_left,
				Vec2::ZERO,
				my_global_transform,
				widget_state,
			);
		}
		if let Some(inspector) = &self.inspector {
			inspector.populate_from_state(widget_state.entry(self.id.clone()).or_default());
		}
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct UniqueChildIdGenerator {
	name_counts: HashMap<&'static str, usize>,
}

impl UniqueChildIdGenerator {
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
	highlighted_widget_id: Option<String>,
}

fn show_debug_widget_info(
	ui: &mut micro::egui::Ui,
	highlighted_widget_path: &mut Option<String>,
	widget: &BakedWidget,
	position: Option<Vec2>,
	widget_state: &IndexMap<String, WidgetState>,
) {
	let label = format!(
		"{} ({})",
		widget.id.split("/").last().unwrap(),
		widget.raw.name()
	);
	let state = &widget_state[&widget.id];
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
		widget.raw.debug_info(ui, state);
		ui.horizontal(|ui| {
			ui.label("Hovered:");
			ui.monospace(format!("{}", state.hovered()));
		});
		ui.horizontal(|ui| {
			ui.label("Held:");
			for mouse_button in MouseButton::KNOWN {
				let short_label = match mouse_button {
					MouseButton::Left => "L",
					MouseButton::Middle => "M",
					MouseButton::Right => "R",
					MouseButton::X1 => "X1",
					MouseButton::X2 => "X2",
					MouseButton::Unknown => unreachable!(),
				};
				if state.held(mouse_button) {
					ui.strong(short_label);
				} else {
					ui.label(short_label);
				}
			}
		});
		for (i, child) in widget.children.iter().enumerate() {
			show_debug_widget_info(
				ui,
				highlighted_widget_path,
				child,
				Some(widget.layout_result.child_positions[i]),
				widget_state,
			);
		}
	});
	if response.header_response.contains_pointer() {
		*highlighted_widget_path = Some(widget.id.clone());
	}
}
