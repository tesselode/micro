use std::time::Duration;

use hecs::World;
use micro::{Context, Event};

use crate::{Queues, System};

pub(super) struct SystemWrapper<Globals, EcsContext, EcsEvent, Error> {
	system: Box<dyn System<Globals, EcsContext, EcsEvent, Error>>,
	pub enabled: bool,
}

impl<Globals, EcsContext, EcsEvent, Error> SystemWrapper<Globals, EcsContext, EcsEvent, Error> {
	pub fn new(system: impl System<Globals, EcsContext, EcsEvent, Error> + 'static) -> Self {
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
	) -> Result<(), Error> {
		if !self.enabled {
			return Ok(());
		}
		self.system.init(ctx, globals, ecs_ctx, world, queues)?;
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
	) -> Result<(), Error> {
		if !self.enabled {
			return Ok(());
		}
		self.system
			.debug_ui(ctx, egui_ctx, globals, ecs_ctx, world, queues)?;
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
	) -> Result<(), Error> {
		if !self.enabled {
			return Ok(());
		}
		self.system
			.event(ctx, globals, ecs_ctx, world, queues, event)?;
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
	) -> Result<(), Error> {
		if !self.enabled {
			return Ok(());
		}
		self.system
			.ecs_event(ctx, globals, ecs_ctx, world, queues, event)?;
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
	) -> Result<(), Error> {
		if !self.enabled {
			return Ok(());
		}
		self.system
			.update(ctx, globals, ecs_ctx, world, queues, delta_time)?;
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
	) -> Result<(), Error> {
		if !self.enabled {
			return Ok(());
		}
		self.system
			.update_cosmetic(ctx, globals, ecs_ctx, world, queues, delta_time)?;
		Ok(())
	}

	pub fn pause(
		&mut self,
		ctx: &mut Context,
		globals: &mut Globals,
		ecs_ctx: &mut EcsContext,
		world: &mut World,
		queues: &mut Queues<EcsEvent>,
	) -> Result<(), Error> {
		if !self.enabled {
			return Ok(());
		}
		self.system.pause(ctx, globals, ecs_ctx, world, queues)?;
		Ok(())
	}

	pub fn resume(
		&mut self,
		ctx: &mut Context,
		globals: &mut Globals,
		ecs_ctx: &mut EcsContext,
		world: &mut World,
		queues: &mut Queues<EcsEvent>,
	) -> Result<(), Error> {
		if !self.enabled {
			return Ok(());
		}
		self.system.resume(ctx, globals, ecs_ctx, world, queues)?;
		Ok(())
	}

	pub fn leave(
		&mut self,
		ctx: &mut Context,
		globals: &mut Globals,
		ecs_ctx: &mut EcsContext,
		world: &mut World,
		queues: &mut Queues<EcsEvent>,
	) -> Result<(), Error> {
		if !self.enabled {
			return Ok(());
		}
		self.system.leave(ctx, globals, ecs_ctx, world, queues)?;
		Ok(())
	}

	pub fn draw(
		&mut self,
		ctx: &mut Context,
		globals: &mut Globals,
		ecs_ctx: &mut EcsContext,
		world: &mut World,
		queues: &mut Queues<EcsEvent>,
	) -> Result<(), Error> {
		if !self.enabled {
			return Ok(());
		}
		self.system.draw(ctx, globals, ecs_ctx, world, queues)?;
		Ok(())
	}
}
