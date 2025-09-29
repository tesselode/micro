pub mod prelude;
mod queues;
mod system;

pub use queues::*;
pub use system::*;

pub use hecs::*;

use std::time::Duration;

use indexmap::IndexMap;
use micro::{Context, Event};

pub struct Ecs<Globals, EcsContext, EcsEvent, Error> {
	world: World,
	queues: Queues<EcsEvent>,
	systems: Systems<Globals, EcsContext, EcsEvent, Error>,
}

impl<Globals, EcsContext, EcsEvent, Error> Ecs<Globals, EcsContext, EcsEvent, Error> {
	pub fn world(&self) -> &World {
		&self.world
	}

	pub fn world_mut(&mut self) -> &mut World {
		&mut self.world
	}

	pub fn queues(&self) -> &Queues<EcsEvent> {
		&self.queues
	}

	pub fn queues_mut(&mut self) -> &mut Queues<EcsEvent> {
		&mut self.queues
	}

	pub fn debug_ui(
		&mut self,
		ctx: &mut Context,
		egui_ctx: &micro::egui::Context,
		ecs_ctx: &mut EcsContext,
		globals: &mut Globals,
	) -> Result<(), Error> {
		self.systems.debug_ui(
			ctx,
			egui_ctx,
			globals,
			ecs_ctx,
			&mut self.world,
			&mut self.queues,
		)
	}

	pub fn event(
		&mut self,
		ctx: &mut Context,
		globals: &mut Globals,
		ecs_ctx: &mut EcsContext,
		event: &Event,
	) -> Result<(), Error> {
		self.systems.event(
			ctx,
			globals,
			ecs_ctx,
			&mut self.world,
			&mut self.queues,
			event,
		)
	}

	pub fn dispatch_ecs_event(
		&mut self,
		ctx: &mut Context,
		globals: &mut Globals,
		ecs_ctx: &mut EcsContext,
		event: &EcsEvent,
	) -> Result<(), Error> {
		self.systems.ecs_event(
			ctx,
			globals,
			ecs_ctx,
			&mut self.world,
			&mut self.queues,
			event,
		)
	}

	pub fn update(
		&mut self,
		ctx: &mut Context,
		globals: &mut Globals,
		ecs_ctx: &mut EcsContext,
		delta_time: Duration,
	) -> Result<(), Error> {
		self.systems.update(
			ctx,
			globals,
			ecs_ctx,
			&mut self.world,
			&mut self.queues,
			delta_time,
		)
	}

	pub fn update_cosmetic(
		&mut self,
		ctx: &mut Context,
		globals: &mut Globals,
		ecs_ctx: &mut EcsContext,
		delta_time: Duration,
	) -> Result<(), Error> {
		self.systems.update_cosmetic(
			ctx,
			globals,
			ecs_ctx,
			&mut self.world,
			&mut self.queues,
			delta_time,
		)
	}

	pub fn pause(
		&mut self,
		ctx: &mut Context,
		globals: &mut Globals,
		ecs_ctx: &mut EcsContext,
	) -> Result<(), Error> {
		self.systems
			.pause(ctx, globals, ecs_ctx, &mut self.world, &mut self.queues)
	}

	pub fn resume(
		&mut self,
		ctx: &mut Context,
		globals: &mut Globals,
		ecs_ctx: &mut EcsContext,
	) -> Result<(), Error> {
		self.systems
			.resume(ctx, globals, ecs_ctx, &mut self.world, &mut self.queues)
	}

	pub fn leave(
		&mut self,
		ctx: &mut Context,
		globals: &mut Globals,
		ecs_ctx: &mut EcsContext,
	) -> Result<(), Error> {
		self.systems
			.leave(ctx, globals, ecs_ctx, &mut self.world, &mut self.queues)
	}

	pub fn draw(
		&mut self,
		ctx: &mut Context,
		globals: &mut Globals,
		ecs_ctx: &mut EcsContext,
	) -> Result<(), Error> {
		self.systems
			.draw(ctx, globals, ecs_ctx, &mut self.world, &mut self.queues)?;
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

	fn init(
		&mut self,
		ctx: &mut Context,
		globals: &mut Globals,
		ecs_ctx: &mut EcsContext,
	) -> Result<(), Error> {
		self.systems
			.init(ctx, globals, ecs_ctx, &mut self.world, &mut self.queues)?;
		self.queues.flush_world_queue(&mut self.world);
		Ok(())
	}
}

pub struct EcsBuilder<Globals, EcsContext, EcsEvent, Error> {
	systems: Systems<Globals, EcsContext, EcsEvent, Error>,
}

impl<Globals, EcsContext, EcsEvent, Error> EcsBuilder<Globals, EcsContext, EcsEvent, Error> {
	pub fn new() -> Self {
		Self {
			systems: Systems::new(),
		}
	}

	pub fn system(
		mut self,
		system: impl System<Globals, EcsContext, EcsEvent, Error> + 'static,
	) -> Self {
		self.systems.add(system);
		self
	}

	pub fn build(
		self,
		ctx: &mut Context,
		globals: &mut Globals,
		ecs_ctx: &mut EcsContext,
	) -> Result<Ecs<Globals, EcsContext, EcsEvent, Error>, Error> {
		let mut ecs = Ecs {
			world: World::new(),
			queues: Queues::new(),
			systems: self.systems,
		};
		ecs.init(ctx, globals, ecs_ctx)?;
		Ok(ecs)
	}
}

impl<Globals, EcsContext, EcsEvent, Error> Default
	for EcsBuilder<Globals, EcsContext, EcsEvent, Error>
{
	fn default() -> Self {
		Self::new()
	}
}
