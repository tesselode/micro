pub mod prelude;
mod queues;
mod system;

pub use queues::*;
pub use system::*;

pub use hecs::*;

use std::time::Duration;

use indexmap::IndexMap;
use micro::{Context, Event};

pub struct Ecs<Globals, EcsContext, EcsEvent> {
	world: World,
	queues: Queues<EcsEvent>,
	systems: Systems<Globals, EcsContext, EcsEvent>,
}

impl<Globals, EcsContext, EcsEvent> Ecs<Globals, EcsContext, EcsEvent> {
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
		globals: &mut Globals,
		ecs_ctx: &mut EcsContext,
	) {
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
	) {
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
	) {
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
	) {
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
	) {
		self.systems.update_cosmetic(
			ctx,
			globals,
			ecs_ctx,
			&mut self.world,
			&mut self.queues,
			delta_time,
		)
	}

	pub fn pause(&mut self, ctx: &mut Context, globals: &mut Globals, ecs_ctx: &mut EcsContext) {
		self.systems
			.pause(ctx, globals, ecs_ctx, &mut self.world, &mut self.queues)
	}

	pub fn resume(&mut self, ctx: &mut Context, globals: &mut Globals, ecs_ctx: &mut EcsContext) {
		self.systems
			.resume(ctx, globals, ecs_ctx, &mut self.world, &mut self.queues)
	}

	pub fn leave(&mut self, ctx: &mut Context, globals: &mut Globals, ecs_ctx: &mut EcsContext) {
		self.systems
			.leave(ctx, globals, ecs_ctx, &mut self.world, &mut self.queues)
	}

	pub fn draw(&mut self, ctx: &mut Context, globals: &mut Globals, ecs_ctx: &mut EcsContext) {
		self.systems
			.draw(ctx, globals, ecs_ctx, &mut self.world, &mut self.queues)
	}

	pub fn post_draw(
		&mut self,
		ctx: &mut Context,
		globals: &mut Globals,
		ecs_ctx: &mut EcsContext,
	) {
		self.systems
			.post_draw(ctx, globals, ecs_ctx, &mut self.world, &mut self.queues);
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

	fn init(&mut self, ctx: &mut Context, globals: &mut Globals, ecs_ctx: &mut EcsContext) {
		self.systems
			.init(ctx, globals, ecs_ctx, &mut self.world, &mut self.queues);
		self.queues.flush_world_queue(&mut self.world);
	}
}

pub struct EcsBuilder<Globals, EcsContext, EcsEvent> {
	systems: Systems<Globals, EcsContext, EcsEvent>,
}

impl<Globals, EcsContext, EcsEvent> EcsBuilder<Globals, EcsContext, EcsEvent> {
	pub fn new() -> Self {
		Self {
			systems: Systems::new(),
		}
	}

	pub fn system(mut self, system: impl System<Globals, EcsContext, EcsEvent> + 'static) -> Self {
		self.systems.add(system);
		self
	}

	pub fn build(
		self,
		ctx: &mut Context,
		globals: &mut Globals,
		ecs_ctx: &mut EcsContext,
	) -> Ecs<Globals, EcsContext, EcsEvent> {
		let mut ecs = Ecs {
			world: World::new(),
			queues: Queues::new(),
			systems: self.systems,
		};
		ecs.init(ctx, globals, ecs_ctx);
		ecs
	}
}

impl<Globals, EcsContext, EcsEvent> Default for EcsBuilder<Globals, EcsContext, EcsEvent> {
	fn default() -> Self {
		Self::new()
	}
}
