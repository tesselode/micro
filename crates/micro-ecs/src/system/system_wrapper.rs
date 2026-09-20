use std::time::Duration;

use hecs::World;
use micro::{Event, Micro};

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
		micro: &mut Micro,
		globals: &mut Globals,
		ecs_ctx: &mut EcsContext,
		world: &mut World,
		queues: &mut Queues<EcsEvent>,
	) {
		if !self.enabled {
			return;
		}
		self.system.init(micro, globals, ecs_ctx, world, queues);
	}

	pub fn debug_ui(
		&mut self,
		micro: &mut Micro,
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
			.debug_ui(micro, egui_ctx, globals, ecs_ctx, world, queues);
	}

	pub fn event(
		&mut self,
		micro: &mut Micro,
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
			.event(micro, globals, ecs_ctx, world, queues, event);
	}

	pub fn ecs_event(
		&mut self,
		micro: &mut Micro,
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
			.ecs_event(micro, globals, ecs_ctx, world, queues, event);
	}

	pub fn update(
		&mut self,
		micro: &mut Micro,
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
			.update(micro, globals, ecs_ctx, world, queues, delta_time);
	}

	pub fn update_cosmetic(
		&mut self,
		micro: &mut Micro,
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
			.update_cosmetic(micro, globals, ecs_ctx, world, queues, delta_time);
	}

	pub fn pause(
		&mut self,
		micro: &mut Micro,
		globals: &mut Globals,
		ecs_ctx: &mut EcsContext,
		world: &mut World,
		queues: &mut Queues<EcsEvent>,
	) {
		if !self.enabled {
			return;
		}
		self.system.pause(micro, globals, ecs_ctx, world, queues);
	}

	pub fn resume(
		&mut self,
		micro: &mut Micro,
		globals: &mut Globals,
		ecs_ctx: &mut EcsContext,
		world: &mut World,
		queues: &mut Queues<EcsEvent>,
	) {
		if !self.enabled {
			return;
		}
		self.system.resume(micro, globals, ecs_ctx, world, queues);
	}

	pub fn leave(
		&mut self,
		micro: &mut Micro,
		globals: &mut Globals,
		ecs_ctx: &mut EcsContext,
		world: &mut World,
		queues: &mut Queues<EcsEvent>,
	) {
		if !self.enabled {
			return;
		}
		self.system.leave(micro, globals, ecs_ctx, world, queues);
	}

	pub fn draw(
		&mut self,
		micro: &mut Micro,
		globals: &mut Globals,
		ecs_ctx: &mut EcsContext,
		world: &mut World,
		queues: &mut Queues<EcsEvent>,
	) {
		if !self.enabled {
			return;
		}
		self.system.draw(micro, globals, ecs_ctx, world, queues);
	}

	pub fn post_draw(
		&mut self,
		micro: &mut Micro,
		globals: &mut Globals,
		ecs_ctx: &mut EcsContext,
		world: &mut World,
		queues: &mut Queues<EcsEvent>,
	) {
		if !self.enabled {
			return;
		}
		self.system
			.post_draw(micro, globals, ecs_ctx, world, queues);
	}
}
