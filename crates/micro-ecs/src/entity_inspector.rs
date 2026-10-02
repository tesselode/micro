use std::fmt::Debug;

use hecs::{Entity, EntityRef, World};
use indexmap::{IndexMap, IndexSet};
use micro::egui::{TextEdit, Ui};

use crate::Ecs;

#[derive(Debug, Clone, Copy)]
pub struct InspectableComponent {
	pub name: &'static str,
	pub count: fn(&mut World) -> usize,
	pub exists: fn(EntityRef<'_>) -> bool,
	pub inspect: fn(&mut Ui, EntityRef<'_>),
}

inventory::collect!(InspectableComponent);

pub(super) fn inspectable_components() -> IndexMap<&'static str, InspectableComponent> {
	let mut inspectable_components = inventory::iter::<InspectableComponent>
		.into_iter()
		.map(|component| (component.name, *component))
		.collect::<IndexMap<&'static str, InspectableComponent>>();
	inspectable_components.sort_by_key(|name, _| *name);
	inspectable_components
}

impl<Globals, EcsContext, EcsEvent> Ecs<Globals, EcsContext, EcsEvent> {
	pub fn show_entity_inspector(&mut self, egui_ctx: &micro::egui::Context, open: &mut bool) {
		micro::egui::Window::new("Entities")
			.open(open)
			.scroll(true)
			.show(egui_ctx, |ui| {
				ui.columns(2, |columns| {
					self.show_filter_section(&mut columns[0]);
					self.show_entities_section(&mut columns[1]);
				});
			});

		let mut closed_windows = IndexSet::new();
		for &entity in &self.inspecting_entities {
			let Ok(entity_ref) = self.world.entity(entity) else {
				closed_windows.insert(entity);
				continue;
			};
			let closed = self.show_entity_window(entity_ref, egui_ctx);
			if closed {
				closed_windows.insert(entity);
			}
		}
		for entity in closed_windows {
			self.inspecting_entities.swap_remove(&entity);
		}
	}

	fn show_filter_section(&mut self, ui: &mut Ui) {
		ui.horizontal(|ui| {
			ui.heading("Filter");
			#[allow(clippy::collapsible_if)]
			if !self.inspector_filter.is_empty() {
				if ui.button("Clear").clicked() {
					self.inspector_filter.clear();
				}
			}
		});
		ui.add(TextEdit::singleline(&mut self.inspector_search).hint_text("Search for components"));
		micro::egui::Grid::new("filters")
			.min_col_width(1.0)
			.show(ui, |ui| {
				for (_, &InspectableComponent { name, count, .. }) in &self.inspectable_components {
					if !name
						.to_lowercase()
						.contains(&self.inspector_search.to_lowercase())
					{
						continue;
					}
					let count = count(&mut self.world);
					let mut enabled = self.inspector_filter.contains(name);
					ui.checkbox(&mut enabled, name);
					if enabled {
						self.inspector_filter.insert(name);
					} else {
						self.inspector_filter.swap_remove(name);
					}
					ui.label(count.to_string());
					ui.end_row();
				}
				ui.strong("Total");
				ui.strong(self.world.len().to_string());
			});
	}

	fn show_entities_section(&mut self, ui: &mut Ui) {
		ui.heading("Entities");
		'entity: for entity in self.world.query::<Entity>().iter() {
			let entity_ref = self.world.entity(entity).unwrap();
			for filtered_component_name in &self.inspector_filter {
				let component = self.inspectable_components[filtered_component_name];
				if !(component.exists)(entity_ref) {
					continue 'entity;
				}
			}
			let label = format!("{:?}", entity);
			if ui.button(label).clicked() {
				self.inspecting_entities.insert(entity);
			};
		}
	}

	fn show_entity_window(&self, entity_ref: EntityRef, egui_ctx: &micro::egui::Context) -> Closed {
		let entity = entity_ref.entity();
		let window_title = format!("{:?}", entity);
		let mut open = true;
		micro::egui::Window::new(window_title)
			.open(&mut open)
			.scroll(true)
			.show(egui_ctx, |ui| {
				for (_, InspectableComponent { inspect, .. }) in &self.inspectable_components {
					inspect(ui, entity_ref);
				}
			});
		!open
	}
}

type Closed = bool;
