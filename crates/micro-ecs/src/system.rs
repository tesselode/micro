mod system_wrapper;

use std::time::Duration;

use hecs::World;
use indexmap::IndexMap;
use micro::{Event, Micro};

use crate::{Queues, Resources, system::system_wrapper::SystemWrapper};

#[allow(unused_variables)]
pub trait System<Globals, EcsEvent> {
	fn name(&self) -> &'static str;

	fn init(
		&mut self,
		micro: &mut Micro,
		globals: &mut Globals,
		resources: &mut Resources,
		world: &mut World,
		queues: &mut Queues<EcsEvent>,
	) {
	}

	fn debug_ui(
		&mut self,
		micro: &mut Micro,
		egui_ctx: &micro::egui::Context,
		globals: &mut Globals,
		resources: &mut Resources,
		world: &mut World,
		queues: &mut Queues<EcsEvent>,
	) {
	}

	fn event(
		&mut self,
		micro: &mut Micro,
		globals: &mut Globals,
		resources: &mut Resources,
		world: &mut World,
		queues: &mut Queues<EcsEvent>,
		event: &Event,
	) {
	}

	fn ecs_event(
		&mut self,
		micro: &mut Micro,
		globals: &mut Globals,
		resources: &mut Resources,
		world: &mut World,
		queues: &mut Queues<EcsEvent>,
		event: &EcsEvent,
	) {
	}

	fn update(
		&mut self,
		micro: &mut Micro,
		globals: &mut Globals,
		resources: &mut Resources,
		world: &mut World,
		queues: &mut Queues<EcsEvent>,
		delta_time: Duration,
	) {
	}

	fn update_cosmetic(
		&mut self,
		micro: &mut Micro,
		globals: &mut Globals,
		resources: &mut Resources,
		world: &mut World,
		queues: &mut Queues<EcsEvent>,
		delta_time: Duration,
	) {
	}

	fn pause(
		&mut self,
		micro: &mut Micro,
		globals: &mut Globals,
		resources: &mut Resources,
		world: &mut World,
		queues: &mut Queues<EcsEvent>,
	) {
	}

	fn resume(
		&mut self,
		micro: &mut Micro,
		globals: &mut Globals,
		resources: &mut Resources,
		world: &mut World,
		queues: &mut Queues<EcsEvent>,
	) {
	}

	fn leave(
		&mut self,
		micro: &mut Micro,
		globals: &mut Globals,
		resources: &mut Resources,
		world: &mut World,
		queues: &mut Queues<EcsEvent>,
	) {
	}

	fn draw(
		&mut self,
		micro: &mut Micro,
		globals: &mut Globals,
		resources: &mut Resources,
		world: &mut World,
		queues: &mut Queues<EcsEvent>,
	) {
	}

	fn post_draw(
		&mut self,
		micro: &mut Micro,
		globals: &mut Globals,
		resources: &mut Resources,
		world: &mut World,
		queues: &mut Queues<EcsEvent>,
	) {
	}
}

pub(crate) struct Systems<Globals, EcsEvent>(Vec<SystemWrapper<Globals, EcsEvent>>);

impl<Globals, EcsEvent> Systems<Globals, EcsEvent> {
	pub fn new() -> Self {
		Self(vec![])
	}

	pub fn add(&mut self, system: impl System<Globals, EcsEvent> + 'static) {
		self.0.push(SystemWrapper::new(system));
	}

	pub(crate) fn init(
		&mut self,
		micro: &mut Micro,
		globals: &mut Globals,
		resources: &mut Resources,
		world: &mut World,
		queues: &mut Queues<EcsEvent>,
	) {
		for system in &mut self.0 {
			system.init(micro, globals, resources, world, queues);
		}
		self.dispatch_ecs_events(micro, globals, resources, world, queues);
	}

	pub(crate) fn debug_ui(
		&mut self,
		micro: &mut Micro,
		egui_ctx: &micro::egui::Context,
		globals: &mut Globals,
		resources: &mut Resources,
		world: &mut World,
		queues: &mut Queues<EcsEvent>,
	) {
		for system in &mut self.0 {
			system.debug_ui(micro, egui_ctx, globals, resources, world, queues);
		}
		self.dispatch_ecs_events(micro, globals, resources, world, queues);
	}

	pub(crate) fn event(
		&mut self,
		micro: &mut Micro,
		globals: &mut Globals,
		resources: &mut Resources,
		world: &mut World,
		queues: &mut Queues<EcsEvent>,
		event: &Event,
	) {
		for system in &mut self.0 {
			system.event(micro, globals, resources, world, queues, event);
		}
		self.dispatch_ecs_events(micro, globals, resources, world, queues);
	}

	pub(crate) fn ecs_event(
		&mut self,
		micro: &mut Micro,
		globals: &mut Globals,
		resources: &mut Resources,
		world: &mut World,
		queues: &mut Queues<EcsEvent>,
		event: &EcsEvent,
	) {
		for system in &mut self.0 {
			system.ecs_event(micro, globals, resources, world, queues, event);
		}
		self.dispatch_ecs_events(micro, globals, resources, world, queues);
	}

	pub(crate) fn update(
		&mut self,
		micro: &mut Micro,
		globals: &mut Globals,
		resources: &mut Resources,
		world: &mut World,
		queues: &mut Queues<EcsEvent>,
		delta_time: Duration,
	) {
		for system in &mut self.0 {
			system.update(micro, globals, resources, world, queues, delta_time);
		}
		self.dispatch_ecs_events(micro, globals, resources, world, queues);
	}

	pub(crate) fn update_cosmetic(
		&mut self,
		micro: &mut Micro,
		globals: &mut Globals,
		resources: &mut Resources,
		world: &mut World,
		queues: &mut Queues<EcsEvent>,
		delta_time: Duration,
	) {
		for system in &mut self.0 {
			system.update_cosmetic(micro, globals, resources, world, queues, delta_time);
		}
		self.dispatch_ecs_events(micro, globals, resources, world, queues);
	}

	pub(crate) fn pause(
		&mut self,
		micro: &mut Micro,
		globals: &mut Globals,
		resources: &mut Resources,
		world: &mut World,
		queues: &mut Queues<EcsEvent>,
	) {
		for system in &mut self.0 {
			system.pause(micro, globals, resources, world, queues);
		}
		self.dispatch_ecs_events(micro, globals, resources, world, queues);
	}

	pub(crate) fn resume(
		&mut self,
		micro: &mut Micro,
		globals: &mut Globals,
		resources: &mut Resources,
		world: &mut World,
		queues: &mut Queues<EcsEvent>,
	) {
		for system in &mut self.0 {
			system.resume(micro, globals, resources, world, queues);
		}
		self.dispatch_ecs_events(micro, globals, resources, world, queues);
	}

	pub(crate) fn leave(
		&mut self,
		micro: &mut Micro,
		globals: &mut Globals,
		resources: &mut Resources,
		world: &mut World,
		queues: &mut Queues<EcsEvent>,
	) {
		for system in &mut self.0 {
			system.leave(micro, globals, resources, world, queues);
		}
		self.dispatch_ecs_events(micro, globals, resources, world, queues);
	}

	pub(crate) fn draw(
		&mut self,
		micro: &mut Micro,
		globals: &mut Globals,
		resources: &mut Resources,
		world: &mut World,
		queues: &mut Queues<EcsEvent>,
	) {
		for system in &mut self.0 {
			system.draw(micro, globals, resources, world, queues);
		}
		self.dispatch_ecs_events(micro, globals, resources, world, queues);
	}

	pub(crate) fn post_draw(
		&mut self,
		micro: &mut Micro,
		globals: &mut Globals,
		resources: &mut Resources,
		world: &mut World,
		queues: &mut Queues<EcsEvent>,
	) {
		for system in &mut self.0 {
			system.post_draw(micro, globals, resources, world, queues);
		}
		self.dispatch_ecs_events(micro, globals, resources, world, queues);
	}

	pub(crate) fn show_systems_window(
		&mut self,
		open: &mut bool,
		egui_ctx: &micro::egui::Context,
		presets: IndexMap<String, Vec<String>>,
	) {
		micro::egui::Window::new("Systems")
			.scroll([false, true])
			.open(open)
			.show(egui_ctx, |ui| {
				ui.horizontal(|ui| {
					if ui.button("Enable all").clicked() {
						for system in &mut self.0 {
							system.enabled = true;
						}
					}
					if ui.button("Disable all").clicked() {
						for system in &mut self.0 {
							system.enabled = false;
						}
					}
				});
				ui.separator();
				ui.label("Presets");
				for (preset_name, systems) in &presets {
					if ui.button(preset_name).clicked() {
						for system in &mut self.0 {
							system.enabled = systems.iter().any(|name| name == system.name());
						}
					}
				}
				ui.separator();
				for system in &mut self.0 {
					let name = system.name();
					ui.checkbox(&mut system.enabled, name);
				}
			});
	}

	fn dispatch_ecs_events(
		&mut self,
		micro: &mut Micro,
		globals: &mut Globals,
		resources: &mut Resources,
		world: &mut World,
		queues: &mut Queues<EcsEvent>,
	) {
		while let Some(event) = queues.pop_event() {
			for system in &mut self.0 {
				system.ecs_event(micro, globals, resources, world, queues, &event);
			}
		}
	}
}
