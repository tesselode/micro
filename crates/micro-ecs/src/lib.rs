pub mod prelude;
mod queues;
mod resources;
mod system;

pub use queues::*;
pub use resources::*;
pub use system::*;

pub use hecs::*;

use std::time::Duration;

use indexmap::IndexMap;
use micro::{Context, Event};

pub struct Ecs<Globals, WorldEvent, Error> {
	world: World,
	queues: Queues<WorldEvent>,
	resources: Resources,
	systems: Systems<Globals, WorldEvent, Error>,
}

impl<Globals, WorldEvent, Error> Ecs<Globals, WorldEvent, Error> {
	pub fn world(&self) -> &World {
		&self.world
	}

	pub fn world_mut(&mut self) -> &mut World {
		&mut self.world
	}

	pub fn queues(&self) -> &Queues<WorldEvent> {
		&self.queues
	}

	pub fn queues_mut(&mut self) -> &mut Queues<WorldEvent> {
		&mut self.queues
	}

	pub fn resources(&self) -> &Resources {
		&self.resources
	}

	pub fn debug_ui(
		&mut self,
		ctx: &mut Context,
		egui_ctx: &micro::egui::Context,
		globals: &mut Globals,
	) -> Result<(), Error> {
		self.systems.debug_ui(
			ctx,
			egui_ctx,
			globals,
			&mut self.resources,
			&mut self.world,
			&mut self.queues,
		)
	}

	pub fn event(
		&mut self,
		ctx: &mut Context,
		globals: &mut Globals,
		event: &Event,
	) -> Result<(), Error> {
		self.systems.event(
			ctx,
			globals,
			&mut self.resources,
			&mut self.world,
			&mut self.queues,
			event,
		)
	}

	pub fn dispatch_world_event(
		&mut self,
		ctx: &mut Context,
		globals: &mut Globals,
		event: &WorldEvent,
	) -> Result<(), Error> {
		self.systems.world_event(
			ctx,
			globals,
			&mut self.resources,
			&mut self.world,
			&mut self.queues,
			event,
		)
	}

	pub fn update(
		&mut self,
		ctx: &mut Context,
		globals: &mut Globals,
		delta_time: Duration,
	) -> Result<(), Error> {
		self.systems.update(
			ctx,
			globals,
			&mut self.resources,
			&mut self.world,
			&mut self.queues,
			delta_time,
		)
	}

	pub fn update_cosmetic(
		&mut self,
		ctx: &mut Context,
		globals: &mut Globals,
		delta_time: Duration,
	) -> Result<(), Error> {
		self.systems.update_cosmetic(
			ctx,
			globals,
			&mut self.resources,
			&mut self.world,
			&mut self.queues,
			delta_time,
		)
	}

	pub fn pause(&mut self, ctx: &mut Context, globals: &mut Globals) -> Result<(), Error> {
		self.systems.pause(
			ctx,
			globals,
			&mut self.resources,
			&mut self.world,
			&mut self.queues,
		)
	}

	pub fn resume(&mut self, ctx: &mut Context, globals: &mut Globals) -> Result<(), Error> {
		self.systems.resume(
			ctx,
			globals,
			&mut self.resources,
			&mut self.world,
			&mut self.queues,
		)
	}

	pub fn leave(&mut self, ctx: &mut Context, globals: &mut Globals) -> Result<(), Error> {
		self.systems.leave(
			ctx,
			globals,
			&mut self.resources,
			&mut self.world,
			&mut self.queues,
		)
	}

	pub fn draw(&mut self, ctx: &mut Context, globals: &mut Globals) -> Result<(), Error> {
		self.systems.draw(
			ctx,
			globals,
			&mut self.resources,
			&mut self.world,
			&mut self.queues,
		)?;
		self.queues.flush_world_queue(&mut self.world);
		Ok(())
	}

	pub fn show_systems_window(
		&mut self,
		open: &mut bool,
		egui_ctx: &micro::egui::Context,
		presets: IndexMap<String, Vec<String>>,
	) {
		self.systems.show_systems_window(open, egui_ctx, presets);
	}

	fn init(&mut self, ctx: &mut Context, globals: &mut Globals) -> Result<(), Error> {
		self.systems.init(
			ctx,
			globals,
			&mut self.resources,
			&mut self.world,
			&mut self.queues,
		)?;
		self.queues.flush_world_queue(&mut self.world);
		Ok(())
	}
}

pub struct EcsBuilder<Globals, WorldEvent, Error> {
	resources: Resources,
	systems: Systems<Globals, WorldEvent, Error>,
}

impl<Globals, WorldEvent, Error> EcsBuilder<Globals, WorldEvent, Error> {
	pub fn new() -> Self {
		Self {
			resources: Resources::new(),
			systems: Systems::new(),
		}
	}

	pub fn resource<T: 'static>(mut self, resource: T) -> Self {
		self.resources.insert(resource);
		self
	}

	pub fn system(mut self, system: impl System<Globals, WorldEvent, Error> + 'static) -> Self {
		self.systems.add(system);
		self
	}

	pub fn build(
		self,
		ctx: &mut Context,
		globals: &mut Globals,
	) -> Result<Ecs<Globals, WorldEvent, Error>, Error> {
		let mut ecs = Ecs {
			world: World::new(),
			queues: Queues::new(),
			resources: self.resources,
			systems: self.systems,
		};
		ecs.init(ctx, globals)?;
		Ok(ecs)
	}
}

impl<Globals, WorldEvent, Error> Default for EcsBuilder<Globals, WorldEvent, Error> {
	fn default() -> Self {
		Self::new()
	}
}
