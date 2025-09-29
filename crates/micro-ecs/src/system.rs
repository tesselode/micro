mod system_wrapper;

use std::time::Duration;

use hecs::World;
use indexmap::IndexMap;
use micro::{Context, Event};

use crate::{Queues, Resources, system::system_wrapper::SystemWrapper};

#[allow(unused_variables)]
pub trait System<Globals, WorldEvent, Error> {
	fn name(&self) -> &'static str;

	fn init(
		&mut self,
		ctx: &mut Context,
		globals: &mut Globals,
		resources: &mut Resources,
		world: &mut World,
		queues: &mut Queues<WorldEvent>,
	) -> Result<(), Error> {
		Ok(())
	}

	fn debug_ui(
		&mut self,
		ctx: &mut Context,
		egui_ctx: &micro::egui::Context,
		globals: &mut Globals,
		resources: &mut Resources,
		world: &mut World,
		queues: &mut Queues<WorldEvent>,
	) -> Result<(), Error> {
		Ok(())
	}

	fn event(
		&mut self,
		ctx: &mut Context,
		globals: &mut Globals,
		resources: &mut Resources,
		world: &mut World,
		queues: &mut Queues<WorldEvent>,
		event: &Event,
	) -> Result<(), Error> {
		Ok(())
	}

	fn world_event(
		&mut self,
		ctx: &mut Context,
		globals: &mut Globals,
		resources: &mut Resources,
		world: &mut World,
		queues: &mut Queues<WorldEvent>,
		event: &WorldEvent,
	) -> Result<(), Error> {
		Ok(())
	}

	fn update(
		&mut self,
		ctx: &mut Context,
		globals: &mut Globals,
		resources: &mut Resources,
		world: &mut World,
		queues: &mut Queues<WorldEvent>,
		delta_time: Duration,
	) -> Result<(), Error> {
		Ok(())
	}

	fn update_cosmetic(
		&mut self,
		ctx: &mut Context,
		globals: &mut Globals,
		resources: &mut Resources,
		world: &mut World,
		queues: &mut Queues<WorldEvent>,
		delta_time: Duration,
	) -> Result<(), Error> {
		Ok(())
	}

	fn pause(
		&mut self,
		ctx: &mut Context,
		globals: &mut Globals,
		resources: &mut Resources,
		world: &mut World,
		queues: &mut Queues<WorldEvent>,
	) -> Result<(), Error> {
		Ok(())
	}

	fn resume(
		&mut self,
		ctx: &mut Context,
		globals: &mut Globals,
		resources: &mut Resources,
		world: &mut World,
		queues: &mut Queues<WorldEvent>,
	) -> Result<(), Error> {
		Ok(())
	}

	fn leave(
		&mut self,
		ctx: &mut Context,
		globals: &mut Globals,
		resources: &mut Resources,
		world: &mut World,
		queues: &mut Queues<WorldEvent>,
	) -> Result<(), Error> {
		Ok(())
	}

	fn draw(
		&mut self,
		ctx: &mut Context,
		globals: &mut Globals,
		resources: &mut Resources,
		world: &mut World,
		queues: &mut Queues<WorldEvent>,
	) -> Result<(), Error> {
		Ok(())
	}
}

pub(crate) struct Systems<Globals, WorldEvent, Error>(
	Vec<SystemWrapper<Globals, WorldEvent, Error>>,
);

impl<Globals, WorldEvent, Error> Systems<Globals, WorldEvent, Error> {
	pub fn new() -> Self {
		Self(vec![])
	}

	pub fn add(&mut self, system: impl System<Globals, WorldEvent, Error> + 'static) {
		self.0.push(SystemWrapper::new(system));
	}

	pub fn init(
		&mut self,
		ctx: &mut Context,
		globals: &mut Globals,
		resources: &mut Resources,
		world: &mut World,
		queues: &mut Queues<WorldEvent>,
	) -> Result<(), Error> {
		for system in &mut self.0 {
			system.init(ctx, globals, resources, world, queues)?;
		}
		self.dispatch_world_events(ctx, globals, resources, world, queues)?;
		Ok(())
	}

	pub fn debug_ui(
		&mut self,
		ctx: &mut Context,
		egui_ctx: &micro::egui::Context,
		globals: &mut Globals,
		resources: &mut Resources,
		world: &mut World,
		queues: &mut Queues<WorldEvent>,
	) -> Result<(), Error> {
		for system in &mut self.0 {
			system.debug_ui(ctx, egui_ctx, globals, resources, world, queues)?;
		}
		self.dispatch_world_events(ctx, globals, resources, world, queues)?;
		Ok(())
	}

	pub fn event(
		&mut self,
		ctx: &mut Context,
		globals: &mut Globals,
		resources: &mut Resources,
		world: &mut World,
		queues: &mut Queues<WorldEvent>,
		event: &Event,
	) -> Result<(), Error> {
		for system in &mut self.0 {
			system.event(ctx, globals, resources, world, queues, event)?;
		}
		self.dispatch_world_events(ctx, globals, resources, world, queues)?;
		Ok(())
	}

	pub fn world_event(
		&mut self,
		ctx: &mut Context,
		globals: &mut Globals,
		resources: &mut Resources,
		world: &mut World,
		queues: &mut Queues<WorldEvent>,
		event: &WorldEvent,
	) -> Result<(), Error> {
		for system in &mut self.0 {
			system.world_event(ctx, globals, resources, world, queues, event)?;
		}
		self.dispatch_world_events(ctx, globals, resources, world, queues)?;
		Ok(())
	}

	pub fn update(
		&mut self,
		ctx: &mut Context,
		globals: &mut Globals,
		resources: &mut Resources,
		world: &mut World,
		queues: &mut Queues<WorldEvent>,
		delta_time: Duration,
	) -> Result<(), Error> {
		for system in &mut self.0 {
			system.update(ctx, globals, resources, world, queues, delta_time)?;
		}
		self.dispatch_world_events(ctx, globals, resources, world, queues)?;
		Ok(())
	}

	pub fn update_cosmetic(
		&mut self,
		ctx: &mut Context,
		globals: &mut Globals,
		resources: &mut Resources,
		world: &mut World,
		queues: &mut Queues<WorldEvent>,
		delta_time: Duration,
	) -> Result<(), Error> {
		for system in &mut self.0 {
			system.update_cosmetic(ctx, globals, resources, world, queues, delta_time)?;
		}
		self.dispatch_world_events(ctx, globals, resources, world, queues)?;
		Ok(())
	}

	pub fn pause(
		&mut self,
		ctx: &mut Context,
		globals: &mut Globals,
		resources: &mut Resources,
		world: &mut World,
		queues: &mut Queues<WorldEvent>,
	) -> Result<(), Error> {
		for system in &mut self.0 {
			system.pause(ctx, globals, resources, world, queues)?;
		}
		self.dispatch_world_events(ctx, globals, resources, world, queues)?;
		Ok(())
	}

	pub fn resume(
		&mut self,
		ctx: &mut Context,
		globals: &mut Globals,
		resources: &mut Resources,
		world: &mut World,
		queues: &mut Queues<WorldEvent>,
	) -> Result<(), Error> {
		for system in &mut self.0 {
			system.resume(ctx, globals, resources, world, queues)?;
		}
		self.dispatch_world_events(ctx, globals, resources, world, queues)?;
		Ok(())
	}

	pub fn leave(
		&mut self,
		ctx: &mut Context,
		globals: &mut Globals,
		resources: &mut Resources,
		world: &mut World,
		queues: &mut Queues<WorldEvent>,
	) -> Result<(), Error> {
		for system in &mut self.0 {
			system.leave(ctx, globals, resources, world, queues)?;
		}
		self.dispatch_world_events(ctx, globals, resources, world, queues)?;
		Ok(())
	}

	pub fn draw(
		&mut self,
		ctx: &mut Context,
		globals: &mut Globals,
		resources: &mut Resources,
		world: &mut World,
		queues: &mut Queues<WorldEvent>,
	) -> Result<(), Error> {
		for system in &mut self.0 {
			system.draw(ctx, globals, resources, world, queues)?;
		}
		self.dispatch_world_events(ctx, globals, resources, world, queues)?;
		Ok(())
	}

	pub fn show_systems_window(
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

	fn dispatch_world_events(
		&mut self,
		ctx: &mut Context,
		globals: &mut Globals,
		resources: &mut Resources,
		world: &mut World,
		queues: &mut Queues<WorldEvent>,
	) -> Result<(), Error> {
		while let Some(event) = queues.pop_event() {
			for system in &mut self.0 {
				system.world_event(ctx, globals, resources, world, queues, &event)?;
			}
		}
		Ok(())
	}
}
