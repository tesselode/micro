use std::time::Duration;

use hecs::World;
use micro::{Context, Event};

use crate::{Queues, System};

pub(super) struct SystemWrapper<Globals, EcsContext, EcsEvent> {
	system: Box<dyn System<Globals, EcsContext, EcsEvent>>,
	pub enabled: bool,
}

impl<Globals, EcsContext, EcsEvent> SystemWrapper<Globals, EcsContext, EcsEvent> {
	pub fn new(system: impl System<Globals, EcsContext, EcsEvent> + 'static) -> Self {
		Self {
			system: Box::new(system),
			enabled: true,
		}
	}

	pub fn name(&self) -> &'static str {
		self.system.name()
	}

	pub fn init(
		&mut self,
		ctx: &mut Context,
		globals: &mut Globals,
		ecs_ctx: &mut EcsContext,
		world: &mut World,
		queues: &mut Queues<EcsEvent>,
	) {
		if !self.enabled {
			return;
		}
		self.system.init(ctx, globals, ecs_ctx, world, queues);
	}

	pub fn debug_ui(
		&mut self,
		ctx: &mut Context,
		egui_ctx: &micro::egui::Context,
		globals: &mut Globals,
		ecs_ctx: &mut EcsContext,
		world: &mut World,
		queues: &mut Queues<EcsEvent>,
	) {
		if !self.enabled {
			return;
		}
		self.system
			.debug_ui(ctx, egui_ctx, globals, ecs_ctx, world, queues);
	}

	pub fn event(
		&mut self,
		ctx: &mut Context,
		globals: &mut Globals,
		ecs_ctx: &mut EcsContext,
		world: &mut World,
		queues: &mut Queues<EcsEvent>,
		event: &Event,
	) {
		if !self.enabled {
			return;
		}
		self.system
			.event(ctx, globals, ecs_ctx, world, queues, event);
	}

	pub fn ecs_event(
		&mut self,
		ctx: &mut Context,
		globals: &mut Globals,
		ecs_ctx: &mut EcsContext,
		world: &mut World,
		queues: &mut Queues<EcsEvent>,
		event: &EcsEvent,
	) {
		if !self.enabled {
			return;
		}
		self.system
			.ecs_event(ctx, globals, ecs_ctx, world, queues, event);
	}

	pub fn update(
		&mut self,
		ctx: &mut Context,
		globals: &mut Globals,
		ecs_ctx: &mut EcsContext,
		world: &mut World,
		queues: &mut Queues<EcsEvent>,
		delta_time: Duration,
	) {
		if !self.enabled {
			return;
		}
		self.system
			.update(ctx, globals, ecs_ctx, world, queues, delta_time);
	}

	pub fn update_cosmetic(
		&mut self,
		ctx: &mut Context,
		globals: &mut Globals,
		ecs_ctx: &mut EcsContext,
		world: &mut World,
		queues: &mut Queues<EcsEvent>,
		delta_time: Duration,
	) {
		if !self.enabled {
			return;
		}
		self.system
			.update_cosmetic(ctx, globals, ecs_ctx, world, queues, delta_time);
	}

	pub fn pause(
		&mut self,
		ctx: &mut Context,
		globals: &mut Globals,
		ecs_ctx: &mut EcsContext,
		world: &mut World,
		queues: &mut Queues<EcsEvent>,
	) {
		if !self.enabled {
			return;
		}
		self.system.pause(ctx, globals, ecs_ctx, world, queues);
	}

	pub fn resume(
		&mut self,
		ctx: &mut Context,
		globals: &mut Globals,
		ecs_ctx: &mut EcsContext,
		world: &mut World,
		queues: &mut Queues<EcsEvent>,
	) {
		if !self.enabled {
			return;
		}
		self.system.resume(ctx, globals, ecs_ctx, world, queues);
	}

	pub fn leave(
		&mut self,
		ctx: &mut Context,
		globals: &mut Globals,
		ecs_ctx: &mut EcsContext,
		world: &mut World,
		queues: &mut Queues<EcsEvent>,
	) {
		if !self.enabled {
			return;
		}
		self.system.leave(ctx, globals, ecs_ctx, world, queues);
	}

	pub fn draw(
		&mut self,
		ctx: &mut Context,
		globals: &mut Globals,
		ecs_ctx: &mut EcsContext,
		world: &mut World,
		queues: &mut Queues<EcsEvent>,
	) {
		if !self.enabled {
			return;
		}
		self.system.draw(ctx, globals, ecs_ctx, world, queues);
	}

	pub fn post_draw(
		&mut self,
		ctx: &mut Context,
		globals: &mut Globals,
		ecs_ctx: &mut EcsContext,
		world: &mut World,
		queues: &mut Queues<EcsEvent>,
	) {
		if !self.enabled {
			return;
		}
		self.system.post_draw(ctx, globals, ecs_ctx, world, queues);
	}
}
