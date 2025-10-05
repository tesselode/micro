mod system_wrapper;

use std::time::Duration;

use hecs::World;
use indexmap::IndexMap;
use micro::{Context, Event};

use crate::{Queues, system::system_wrapper::SystemWrapper};

#[allow(unused_variables)]
pub trait System<Globals, EcsContext, EcsEvent> {
	fn name(&self) -> &'static str;

	fn init(
		&mut self,
		ctx: &mut Context,
		globals: &mut Globals,
		ecs_ctx: &mut EcsContext,
		world: &mut World,
		queues: &mut Queues<EcsEvent>,
	) -> anyhow::Result<()> {
		Ok(())
	}

	fn debug_ui(
		&mut self,
		ctx: &mut Context,
		egui_ctx: &micro::egui::Context,
		globals: &mut Globals,
		ecs_ctx: &mut EcsContext,
		world: &mut World,
		queues: &mut Queues<EcsEvent>,
	) -> anyhow::Result<()> {
		Ok(())
	}

	fn event(
		&mut self,
		ctx: &mut Context,
		globals: &mut Globals,
		ecs_ctx: &mut EcsContext,
		world: &mut World,
		queues: &mut Queues<EcsEvent>,
		event: &Event,
	) -> anyhow::Result<()> {
		Ok(())
	}

	fn ecs_event(
		&mut self,
		ctx: &mut Context,
		globals: &mut Globals,
		ecs_ctx: &mut EcsContext,
		world: &mut World,
		queues: &mut Queues<EcsEvent>,
		event: &EcsEvent,
	) -> anyhow::Result<()> {
		Ok(())
	}

	fn update(
		&mut self,
		ctx: &mut Context,
		globals: &mut Globals,
		ecs_ctx: &mut EcsContext,
		world: &mut World,
		queues: &mut Queues<EcsEvent>,
		delta_time: Duration,
	) -> anyhow::Result<()> {
		Ok(())
	}

	fn update_cosmetic(
		&mut self,
		ctx: &mut Context,
		globals: &mut Globals,
		ecs_ctx: &mut EcsContext,
		world: &mut World,
		queues: &mut Queues<EcsEvent>,
		delta_time: Duration,
	) -> anyhow::Result<()> {
		Ok(())
	}

	fn pause(
		&mut self,
		ctx: &mut Context,
		globals: &mut Globals,
		ecs_ctx: &mut EcsContext,
		world: &mut World,
		queues: &mut Queues<EcsEvent>,
	) -> anyhow::Result<()> {
		Ok(())
	}

	fn resume(
		&mut self,
		ctx: &mut Context,
		globals: &mut Globals,
		ecs_ctx: &mut EcsContext,
		world: &mut World,
		queues: &mut Queues<EcsEvent>,
	) -> anyhow::Result<()> {
		Ok(())
	}

	fn leave(
		&mut self,
		ctx: &mut Context,
		globals: &mut Globals,
		ecs_ctx: &mut EcsContext,
		world: &mut World,
		queues: &mut Queues<EcsEvent>,
	) -> anyhow::Result<()> {
		Ok(())
	}

	fn draw(
		&mut self,
		ctx: &mut Context,
		globals: &mut Globals,
		ecs_ctx: &mut EcsContext,
		world: &mut World,
		queues: &mut Queues<EcsEvent>,
	) -> anyhow::Result<()> {
		Ok(())
	}
}

pub(crate) struct Systems<Globals, EcsContext, EcsEvent>(
	Vec<SystemWrapper<Globals, EcsContext, EcsEvent>>,
);

impl<Globals, EcsContext, EcsEvent> Systems<Globals, EcsContext, EcsEvent> {
	pub fn new() -> Self {
		Self(vec![])
	}

	pub fn add(&mut self, system: impl System<Globals, EcsContext, EcsEvent> + 'static) {
		self.0.push(SystemWrapper::new(system));
	}

	pub fn init(
		&mut self,
		ctx: &mut Context,
		globals: &mut Globals,
		ecs_ctx: &mut EcsContext,
		world: &mut World,
		queues: &mut Queues<EcsEvent>,
	) -> anyhow::Result<()> {
		for system in &mut self.0 {
			system.init(ctx, globals, ecs_ctx, world, queues)?;
		}
		self.dispatch_ecs_events(ctx, globals, ecs_ctx, world, queues)?;
		Ok(())
	}

	pub fn debug_ui(
		&mut self,
		ctx: &mut Context,
		egui_ctx: &micro::egui::Context,
		globals: &mut Globals,
		ecs_ctx: &mut EcsContext,
		world: &mut World,
		queues: &mut Queues<EcsEvent>,
	) -> anyhow::Result<()> {
		for system in &mut self.0 {
			system.debug_ui(ctx, egui_ctx, globals, ecs_ctx, world, queues)?;
		}
		self.dispatch_ecs_events(ctx, globals, ecs_ctx, world, queues)?;
		Ok(())
	}

	pub fn event(
		&mut self,
		ctx: &mut Context,
		globals: &mut Globals,
		ecs_ctx: &mut EcsContext,
		world: &mut World,
		queues: &mut Queues<EcsEvent>,
		event: &Event,
	) -> anyhow::Result<()> {
		for system in &mut self.0 {
			system.event(ctx, globals, ecs_ctx, world, queues, event)?;
		}
		self.dispatch_ecs_events(ctx, globals, ecs_ctx, world, queues)?;
		Ok(())
	}

	pub fn ecs_event(
		&mut self,
		ctx: &mut Context,
		globals: &mut Globals,
		ecs_ctx: &mut EcsContext,
		world: &mut World,
		queues: &mut Queues<EcsEvent>,
		event: &EcsEvent,
	) -> anyhow::Result<()> {
		for system in &mut self.0 {
			system.ecs_event(ctx, globals, ecs_ctx, world, queues, event)?;
		}
		self.dispatch_ecs_events(ctx, globals, ecs_ctx, world, queues)?;
		Ok(())
	}

	pub fn update(
		&mut self,
		ctx: &mut Context,
		globals: &mut Globals,
		ecs_ctx: &mut EcsContext,
		world: &mut World,
		queues: &mut Queues<EcsEvent>,
		delta_time: Duration,
	) -> anyhow::Result<()> {
		for system in &mut self.0 {
			system.update(ctx, globals, ecs_ctx, world, queues, delta_time)?;
		}
		self.dispatch_ecs_events(ctx, globals, ecs_ctx, world, queues)?;
		Ok(())
	}

	pub fn update_cosmetic(
		&mut self,
		ctx: &mut Context,
		globals: &mut Globals,
		ecs_ctx: &mut EcsContext,
		world: &mut World,
		queues: &mut Queues<EcsEvent>,
		delta_time: Duration,
	) -> anyhow::Result<()> {
		for system in &mut self.0 {
			system.update_cosmetic(ctx, globals, ecs_ctx, world, queues, delta_time)?;
		}
		self.dispatch_ecs_events(ctx, globals, ecs_ctx, world, queues)?;
		Ok(())
	}

	pub fn pause(
		&mut self,
		ctx: &mut Context,
		globals: &mut Globals,
		ecs_ctx: &mut EcsContext,
		world: &mut World,
		queues: &mut Queues<EcsEvent>,
	) -> anyhow::Result<()> {
		for system in &mut self.0 {
			system.pause(ctx, globals, ecs_ctx, world, queues)?;
		}
		self.dispatch_ecs_events(ctx, globals, ecs_ctx, world, queues)?;
		Ok(())
	}

	pub fn resume(
		&mut self,
		ctx: &mut Context,
		globals: &mut Globals,
		ecs_ctx: &mut EcsContext,
		world: &mut World,
		queues: &mut Queues<EcsEvent>,
	) -> anyhow::Result<()> {
		for system in &mut self.0 {
			system.resume(ctx, globals, ecs_ctx, world, queues)?;
		}
		self.dispatch_ecs_events(ctx, globals, ecs_ctx, world, queues)?;
		Ok(())
	}

	pub fn leave(
		&mut self,
		ctx: &mut Context,
		globals: &mut Globals,
		ecs_ctx: &mut EcsContext,
		world: &mut World,
		queues: &mut Queues<EcsEvent>,
	) -> anyhow::Result<()> {
		for system in &mut self.0 {
			system.leave(ctx, globals, ecs_ctx, world, queues)?;
		}
		self.dispatch_ecs_events(ctx, globals, ecs_ctx, world, queues)?;
		Ok(())
	}

	pub fn draw(
		&mut self,
		ctx: &mut Context,
		globals: &mut Globals,
		ecs_ctx: &mut EcsContext,
		world: &mut World,
		queues: &mut Queues<EcsEvent>,
	) -> anyhow::Result<()> {
		for system in &mut self.0 {
			system.draw(ctx, globals, ecs_ctx, world, queues)?;
		}
		self.dispatch_ecs_events(ctx, globals, ecs_ctx, world, queues)?;
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

	fn dispatch_ecs_events(
		&mut self,
		ctx: &mut Context,
		globals: &mut Globals,
		ecs_ctx: &mut EcsContext,
		world: &mut World,
		queues: &mut Queues<EcsEvent>,
	) -> anyhow::Result<()> {
		while let Some(event) = queues.pop_event() {
			for system in &mut self.0 {
				system.ecs_event(ctx, globals, ecs_ctx, world, queues, &event)?;
			}
		}
		Ok(())
	}
}
