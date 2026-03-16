use std::fmt::Debug;

use micro::{Context, egui, math::Vec2};

use crate::{
	WidgetInspector, WidgetState, child_functions, common_functions, common_widget_trait_functions,
	sizing_functions,
};

use super::{LayoutResult, Sizing, Widget};

#[derive(Debug)]
pub struct Align {
	id: Option<String>,
	inspector: Option<WidgetInspector>,
	parent_anchor: Vec2,
	child_anchor: Vec2,
	sizing: Sizing,
	children: Vec<Box<dyn Widget>>,
}

macro_rules! align_constructors {
	($($name:ident: $align:expr),*$(,)?) => {
		$(
			pub fn $name() -> Self {
				Self::simple($align)
			}
		)*
	};
}

impl Align {
	pub fn new(parent_anchor: impl Into<Vec2>, child_anchor: impl Into<Vec2>) -> Self {
		Self {
			id: None,
			inspector: None,
			parent_anchor: parent_anchor.into(),
			child_anchor: child_anchor.into(),
			sizing: Sizing::EXPAND,
			children: vec![],
		}
	}

	pub fn simple(anchor: impl Into<Vec2>) -> Self {
		let anchor = anchor.into();
		Self::new(anchor, anchor)
	}

	align_constructors! {
		top_left: (0.0, 0.0),
		top_center: (0.5, 0.0),
		top_right: (1.0, 0.0),
		middle_right: (1.0, 0.5),
		bottom_right: (1.0, 1.0),
		bottom_center: (0.5, 1.0),
		bottom_left: (0.0, 1.0),
		middle_left: (0.0, 0.5),
		center: (0.5, 0.5),
	}

	common_functions!();
	child_functions!();
	sizing_functions!();
}

impl Widget for Align {
	common_widget_trait_functions!();

	fn name(&self) -> &'static str {
		"align"
	}

	fn children(&mut self, _ctx: &mut Context, _state: &mut WidgetState) -> Vec<Box<dyn Widget>> {
		self.children.drain(..).collect()
	}

	fn allotted_size_for_next_child(
		&mut self,
		_ctx: &mut Context,
		allotted_size_from_parent: Vec2,
		_previous_child_sizes: &[Vec2],
		_state: &mut WidgetState,
	) -> Vec2 {
		let _span = tracy_client::span!();
		self.sizing
			.allotted_size_for_children(allotted_size_from_parent)
	}

	fn layout(
		&mut self,
		_ctx: &mut Context,
		allotted_size_from_parent: Vec2,
		child_sizes: &[Vec2],
		_state: &mut WidgetState,
	) -> LayoutResult {
		let _span = tracy_client::span!();
		let parent_size = self
			.sizing
			.final_parent_size(allotted_size_from_parent, child_sizes.iter().copied());
		let child_positions = child_sizes
			.iter()
			.copied()
			.map(|child_size| parent_size * self.parent_anchor - child_size * self.child_anchor)
			.collect();
		LayoutResult {
			size: parent_size,
			child_positions,
		}
	}

	fn debug_info(&self, egui_ui: &mut egui::Ui, _state: &WidgetState) {
		self.sizing.debug_info(egui_ui);
		egui_ui.horizontal(|ui| {
			ui.label("Parent anchor:");
			ui.monospace(format!("{}", self.parent_anchor));
		});
		egui_ui.horizontal(|ui| {
			ui.label("Child anchor:");
			ui.monospace(format!("{}", self.child_anchor));
		});
	}
}
