use std::time::Duration;

use hecs::World;
use micro::{Context, Event};

use crate::{Queues, Resources, System};

pub(super) struct SystemWrapper<Globals, WorldEvent, Error> {
	system: Box<dyn System<Globals, WorldEvent, Error>>,
	pub enabled: bool,
}

impl<Globals, WorldEvent, Error> SystemWrapper<Globals, WorldEvent, Error> {
	pub fn new(system: impl System<Globals, WorldEvent, Error> + 'static) -> Self {
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
		resources: &mut Resources,
		world: &mut World,
		queues: &mut Queues<WorldEvent>,
	) -> Result<(), Error> {
		if !self.enabled {
			return Ok(());
		}
		self.system.init(ctx, globals, resources, world, queues)?;
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
		if !self.enabled {
			return Ok(());
		}
		self.system
			.debug_ui(ctx, egui_ctx, globals, resources, world, queues)?;
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
		if !self.enabled {
			return Ok(());
		}
		self.system
			.event(ctx, globals, resources, world, queues, event)?;
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
		if !self.enabled {
			return Ok(());
		}
		self.system
			.world_event(ctx, globals, resources, world, queues, event)?;
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
		if !self.enabled {
			return Ok(());
		}
		self.system
			.update(ctx, globals, resources, world, queues, delta_time)?;
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
		if !self.enabled {
			return Ok(());
		}
		self.system
			.update_cosmetic(ctx, globals, resources, world, queues, delta_time)?;
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
		if !self.enabled {
			return Ok(());
		}
		self.system.pause(ctx, globals, resources, world, queues)?;
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
		if !self.enabled {
			return Ok(());
		}
		self.system.resume(ctx, globals, resources, world, queues)?;
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
		if !self.enabled {
			return Ok(());
		}
		self.system.leave(ctx, globals, resources, world, queues)?;
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
		if !self.enabled {
			return Ok(());
		}
		self.system.draw(ctx, globals, resources, world, queues)?;
		Ok(())
	}
}
