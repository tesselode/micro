use hecs::{Entity, EntityRef};
use indexmap::IndexSet;
use micro::egui::Ui;

use crate::Ecs;

#[derive(Debug, Clone, Copy)]
pub struct InspectableComponent {
	pub name: &'static str,
	pub inspect: fn(&mut Ui, EntityRef<'_>),
}

inventory::collect!(InspectableComponent);

pub(super) fn inspectable_components() -> Vec<InspectableComponent> {
	let mut inspectable_components = inventory::iter::<InspectableComponent>
		.into_iter()
		.copied()
		.collect::<Vec<_>>();
	inspectable_components.sort_by_key(|component| component.name);
	inspectable_components
}

impl<Globals, EcsContext, EcsEvent> Ecs<Globals, EcsContext, EcsEvent> {
	pub fn show_entity_inspector(&mut self, egui_ctx: &micro::egui::Context, open: &mut bool) {
		micro::egui::Window::new("Entities")
			.open(open)
			.scroll(true)
			.show(egui_ctx, |ui| {
				ui.heading("Entities");
				for entity in self.world.query_mut::<Entity>() {
					let label = format!("{:?}", entity);
					if ui.small_button(label).clicked() {
						self.inspecting_entities.insert(entity);
					};
				}
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

	fn show_entity_window(&self, entity_ref: EntityRef, egui_ctx: &micro::egui::Context) -> Closed {
		let entity = entity_ref.entity();
		let window_title = format!("{:?}", entity);
		let mut open = true;
		micro::egui::Window::new(window_title)
			.open(&mut open)
			.scroll(true)
			.show(egui_ctx, |ui| {
				for InspectableComponent { inspect, .. } in &self.inspectable_components {
					inspect(ui, entity_ref);
				}
			});
		!open
	}
}

type Closed = bool;
