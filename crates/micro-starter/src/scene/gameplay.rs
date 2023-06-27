mod context;
mod entity;
mod event;
mod system;

use std::time::Duration;

use hecs::World;
use micro::{Context, Event};

use crate::{globals::Globals, scene_manager::SceneChange};

use self::{context::GameplayContext, system::System};

use super::Scene;

pub struct Gameplay {
	gameplay_ctx: GameplayContext,
	world: World,
	systems: Vec<Box<dyn System>>,
}

impl Gameplay {
	pub fn new(ctx: &mut Context, globals: &mut Globals) -> anyhow::Result<Self> {
		let gameplay_ctx = GameplayContext::new();
		let systems = vec![];
		let mut gameplay = Self {
			gameplay_ctx,
			world: World::new(),
			systems,
		};
		for system in &mut gameplay.systems {
			system.init(
				ctx,
				globals,
				&mut gameplay.gameplay_ctx,
				&mut gameplay.world,
			)?;
		}
		gameplay.dispatch_gameplay_events(ctx, globals)?;
		Ok(gameplay)
	}

	fn dispatch_gameplay_events(
		&mut self,
		ctx: &mut Context,
		globals: &mut Globals,
	) -> anyhow::Result<()> {
		while let Some(event) = self.gameplay_ctx.event_queue.pop_front() {
			for system in &mut self.systems {
				system.gameplay_event(
					ctx,
					globals,
					&mut self.gameplay_ctx,
					&mut self.world,
					&event,
				)?;
			}
		}
		Ok(())
	}
}

impl Scene for Gameplay {
	fn ui(
		&mut self,
		ctx: &mut Context,
		egui_ctx: &egui::Context,
		globals: &mut Globals,
	) -> anyhow::Result<()> {
		for system in &mut self.systems {
			system.ui(
				ctx,
				egui_ctx,
				globals,
				&mut self.gameplay_ctx,
				&mut self.world,
			)?;
		}
		self.dispatch_gameplay_events(ctx, globals)?;
		Ok(())
	}

	fn menu(
		&mut self,
		ctx: &mut Context,
		ui: &mut egui::Ui,
		globals: &mut Globals,
	) -> anyhow::Result<()> {
		for system in &mut self.systems {
			system.menu(ctx, ui, globals, &mut self.gameplay_ctx, &mut self.world)?;
		}
		self.dispatch_gameplay_events(ctx, globals)?;
		Ok(())
	}

	fn stats(&mut self, ctx: &mut Context, globals: &mut Globals) -> Option<Vec<String>> {
		let mut stats = vec![format!("Number of entities: {}", self.world.len())];
		for system in &mut self.systems {
			if let Some(mut system_stats) =
				system.stats(ctx, globals, &mut self.gameplay_ctx, &mut self.world)
			{
				stats.append(&mut system_stats);
			}
		}
		Some(stats)
	}

	fn event(
		&mut self,
		ctx: &mut Context,
		globals: &mut Globals,
		event: &Event,
	) -> anyhow::Result<()> {
		for system in &mut self.systems {
			system.event(ctx, globals, &mut self.gameplay_ctx, &mut self.world, event)?;
		}
		self.dispatch_gameplay_events(ctx, globals)?;
		Ok(())
	}

	fn update(
		&mut self,
		ctx: &mut Context,
		globals: &mut Globals,
		delta_time: Duration,
	) -> anyhow::Result<()> {
		for system in &mut self.systems {
			system.update(
				ctx,
				globals,
				&mut self.gameplay_ctx,
				&mut self.world,
				delta_time,
			)?;
		}
		self.dispatch_gameplay_events(ctx, globals)?;
		self.gameplay_ctx
			.world_command_buffer
			.run_on(&mut self.world);
		Ok(())
	}

	fn draw(&mut self, ctx: &mut Context, globals: &mut Globals) -> anyhow::Result<()> {
		for system in &mut self.systems {
			system.draw(ctx, globals, &mut self.gameplay_ctx, &mut self.world)?;
		}
		Ok(())
	}

	fn scene_change(&mut self) -> Option<SceneChange> {
		self.gameplay_ctx.scene_change.take()
	}
}
