use std::collections::HashMap;

use crate::{CommonWidgetState, mouse_input::MouseInput};

use indexmap::IndexMap;
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
	common_widget_state: IndexMap<String, CommonWidgetState>,
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
					&self.common_widget_state,
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
		for state in self.common_widget_state.values() {
			state.0.borrow_mut().used = false;
		}

		// bake the root widget
		let ctx = &mut ctx.push(settings.transform);
		let default_size = ctx.window_size().as_vec2();
		let mut baked_widget = BakedWidget::new(
			ctx,
			widget.custom_id().unwrap_or_else(|| "root".to_string()),
			&widget,
			settings.size.unwrap_or(default_size),
		);

		// mouse input
		self.mouse_input.update(ctx, settings.transform.inverse());
		baked_widget.use_mouse_input(
			&widget,
			self.mouse_input.clone(),
			&mut self.common_widget_state,
		);

		// draw
		baked_widget.draw(ctx, &widget);

		// draw debug
		if let Some(draw_debug_state) = self.draw_debug_state.take() {
			baked_widget.draw_debug(ctx, &draw_debug_state);
		}

		// report bounds and transforms
		baked_widget.report(
			Vec2::ZERO,
			Vec2::ZERO,
			Mat4::IDENTITY,
			&mut self.common_widget_state,
		);

		// save baked widget for debugging
		self.previous_baked_widget = Some(baked_widget);

		// remove unused widget states
		self.common_widget_state
			.retain(|_, state| state.0.borrow().used);
	}

	pub fn state(&self, id: impl AsRef<str>) -> Option<&CommonWidgetState> {
		self.common_widget_state.get(id.as_ref())
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
	id: String,
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
		raw_widget: &dyn Widget,
		allotted_size_from_parent: Vec2,
	) -> Self {
		let _span = tracy_client::span!();
		let mut children = vec![];
		let mut child_sizes = vec![];
		let mut unique_child_id_generator = UniqueChildIdGenerator::new();

		// bake children
		for child in raw_widget.children() {
			let allotted_size_for_child =
				raw_widget.allotted_size_for_next_child(allotted_size_from_parent, &child_sizes);
			let child_id = child.custom_id().unwrap_or_else(|| {
				let child_id_component = unique_child_id_generator.generate(child.name());
				format!("{}/{}", id, child_id_component)
			});
			let baked_child =
				BakedWidget::new(ctx, child_id, child.as_ref(), allotted_size_for_child);
			child_sizes.push(baked_child.layout_result.size);
			children.push(baked_child);
		}
		let layout_result = raw_widget.layout(ctx, allotted_size_from_parent, &child_sizes);

		// bake mask
		let mask = raw_widget.mask().map(|mask| {
			Box::new(BakedWidget::new(
				ctx,
				mask.custom_id()
					.unwrap_or_else(|| format!("{}/{}", id, "mask")),
				mask,
				layout_result.size,
			))
		});

		let transform = raw_widget.transform(layout_result.size);

		Self {
			name: raw_widget.name(),
			id,
			children,
			transform,
			mask,
			layout_result,
			allotted_size_from_parent,
		}
	}

	fn use_mouse_input(
		&mut self,
		raw_widget: &dyn Widget,
		mut mouse_input: MouseInput,
		common_widget_state: &mut IndexMap<String, CommonWidgetState>,
	) {
		let _span = tracy_client::span!();
		mouse_input = mouse_input.transformed(self.transform.inverse());
		common_widget_state
			.entry(self.id.clone())
			.or_default()
			.update_mouse_state(&mouse_input, self.layout_result.size);
		for (raw_child, baked_child, position) in izip!(
			raw_widget.children(),
			&mut self.children,
			&self.layout_result.child_positions
		) {
			baked_child.use_mouse_input(
				raw_child.as_ref(),
				mouse_input.translated(-position),
				common_widget_state,
			);
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
		common_widget_state: &mut IndexMap<String, CommonWidgetState>,
	) {
		let my_global_top_left = parent_global_top_left + my_offset;
		let my_global_transform = parent_global_transform
			* Mat4::from_translation(my_offset.extend(0.0))
			* self.transform;
		{
			let my_common_widget_state = common_widget_state.entry(self.id.clone()).or_default();
			let mut inner = my_common_widget_state.0.borrow_mut();
			inner.used = true;
			inner.bounds = Some(Rect::new(my_global_top_left, self.layout_result.size));
			inner.transform = Some(my_global_transform);
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
				common_widget_state,
			);
		}
		if let Some(mask) = &self.mask {
			mask.report(
				parent_global_top_left,
				Vec2::ZERO,
				my_global_transform,
				common_widget_state,
			);
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
	common_widget_state: &IndexMap<String, CommonWidgetState>,
) {
	let label = &widget.id;
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
		ui.collapsing("State", |ui| {
			ui.monospace(format!("{:#?}", common_widget_state[&widget.id]));
		});
		for (i, child) in widget.children.iter().enumerate() {
			show_debug_widget_info(
				ui,
				highlighted_widget_path,
				child,
				Some(widget.layout_result.child_positions[i]),
				common_widget_state,
			);
		}
	});
	if response.header_response.contains_pointer() {
		*highlighted_widget_path = Some(widget.id.clone());
	}
}
