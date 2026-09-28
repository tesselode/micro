pub mod prelude;
mod queues;
mod resources;
mod system;

pub use queues::*;
pub use resources::*;
pub use system::*;

pub use hecs::*;

use std::{any::Any, time::Duration};

use indexmap::IndexMap;
use micro::{Event, Micro};

pub struct Ecs<Globals, EcsEvent> {
	pub world: World,
	pub queues: Queues<EcsEvent>,
	pub resources: Resources,
	systems: Systems<Globals, EcsEvent>,
}

impl<Globals, EcsEvent> Ecs<Globals, EcsEvent> {
	pub fn debug_ui(
		&mut self,
		micro: &mut Micro,
		egui_ctx: &micro::egui::Context,
		globals: &mut Globals,
	) {
		self.systems.debug_ui(
			micro,
			egui_ctx,
			globals,
			&mut self.resources,
			&mut self.world,
			&mut self.queues,
		)
	}

	pub fn event(&mut self, micro: &mut Micro, globals: &mut Globals, event: &Event) {
		self.systems.event(
			micro,
			globals,
			&mut self.resources,
			&mut self.world,
			&mut self.queues,
			event,
		)
	}

	pub fn dispatch_ecs_event(
		&mut self,
		micro: &mut Micro,
		globals: &mut Globals,

		event: &EcsEvent,
	) {
		self.systems.ecs_event(
			micro,
			globals,
			&mut self.resources,
			&mut self.world,
			&mut self.queues,
			event,
		)
	}

	pub fn update(&mut self, micro: &mut Micro, globals: &mut Globals, delta_time: Duration) {
		self.systems.update(
			micro,
			globals,
			&mut self.resources,
			&mut self.world,
			&mut self.queues,
			delta_time,
		)
	}

	pub fn update_cosmetic(
		&mut self,
		micro: &mut Micro,
		globals: &mut Globals,

		delta_time: Duration,
	) {
		self.systems.update_cosmetic(
			micro,
			globals,
			&mut self.resources,
			&mut self.world,
			&mut self.queues,
			delta_time,
		)
	}

	pub fn pause(&mut self, micro: &mut Micro, globals: &mut Globals) {
		self.systems.pause(
			micro,
			globals,
			&mut self.resources,
			&mut self.world,
			&mut self.queues,
		)
	}

	pub fn resume(&mut self, micro: &mut Micro, globals: &mut Globals) {
		self.systems.resume(
			micro,
			globals,
			&mut self.resources,
			&mut self.world,
			&mut self.queues,
		)
	}

	pub fn leave(&mut self, micro: &mut Micro, globals: &mut Globals) {
		self.systems.leave(
			micro,
			globals,
			&mut self.resources,
			&mut self.world,
			&mut self.queues,
		)
	}

	pub fn draw(&mut self, micro: &mut Micro, globals: &mut Globals) {
		self.systems.draw(
			micro,
			globals,
			&mut self.resources,
			&mut self.world,
			&mut self.queues,
		)
	}

	pub fn post_draw(&mut self, micro: &mut Micro, globals: &mut Globals) {
		self.systems.post_draw(
			micro,
			globals,
			&mut self.resources,
			&mut self.world,
			&mut self.queues,
		);
		self.queues.flush_world_queue(&mut self.world);
	}

	pub fn show_systems_window(
		&mut self,
		open: &mut bool,
		egui_ctx: &micro::egui::Context,
		presets: IndexMap<String, Vec<String>>,
	) {
		self.systems.show_systems_window(open, egui_ctx, presets);
	}

	fn init(&mut self, micro: &mut Micro, globals: &mut Globals) {
		self.systems.init(
			micro,
			globals,
			&mut self.resources,
			&mut self.world,
			&mut self.queues,
		);
		self.queues.flush_world_queue(&mut self.world);
	}
}

pub struct EcsBuilder<Globals, EcsEvent> {
	systems: Systems<Globals, EcsEvent>,
	resources: Resources,
}

impl<Globals, EcsEvent> EcsBuilder<Globals, EcsEvent> {
	pub fn new() -> Self {
		Self {
			systems: Systems::new(),
			resources: Resources::new(),
		}
	}

	pub fn system(mut self, system: impl System<Globals, EcsEvent> + 'static) -> Self {
		self.systems.add(system);
		self
	}

	pub fn resource(mut self, resource: impl Any + 'static) -> Self {
		self.resources.insert(resource);
		self
	}

	pub fn build(self, micro: &mut Micro, globals: &mut Globals) -> Ecs<Globals, EcsEvent> {
		let mut ecs = Ecs {
			world: World::new(),
			queues: Queues::new(),
			resources: self.resources,
			systems: self.systems,
		};
		ecs.init(micro, globals);
		ecs
	}
}

impl<Globals, EcsEvent> Default for EcsBuilder<Globals, EcsEvent> {
	fn default() -> Self {
		Self::new()
	}
}
