mod component;
mod context;
mod entity;
mod event;
mod system;

use std::time::Duration;

use hecs::World;
use micro::Event;

use crate::scene_manager::SceneChange;

use self::{context::GameplayContext, system::System};

use super::Scene;

pub struct Gameplay {
	gameplay_ctx: GameplayContext,
	world: World,
	systems: Vec<Box<dyn System>>,
}

impl Gameplay {
	pub fn new() -> anyhow::Result<Self> {
		let gameplay_ctx = GameplayContext::new();
		let systems: Vec<Box<dyn System>> = vec![];
		let mut gameplay = Self {
			gameplay_ctx,
			world: World::new(),
			systems,
		};
		for system in &mut gameplay.systems {
			system.init(&mut gameplay.gameplay_ctx, &mut gameplay.world)?;
		}
		gameplay.dispatch_gameplay_events()?;
		Ok(gameplay)
	}

	fn dispatch_gameplay_events(&mut self) -> anyhow::Result<()> {
		while let Some(event) = self.gameplay_ctx.event_queue.pop_front() {
			for system in &mut self.systems {
				system.gameplay_event(&mut self.gameplay_ctx, &mut self.world, &event)?;
			}
		}
		Ok(())
	}
}

impl Scene for Gameplay {
	fn ui(&mut self, egui_ctx: &egui::Context) -> anyhow::Result<()> {
		for system in &mut self.systems {
			system.ui(egui_ctx, &mut self.gameplay_ctx, &mut self.world)?;
		}
		self.dispatch_gameplay_events()?;
		Ok(())
	}

	fn menu(&mut self, ui: &mut egui::Ui) -> anyhow::Result<()> {
		for system in &mut self.systems {
			system.menu(ui, &mut self.gameplay_ctx, &mut self.world)?;
		}
		self.dispatch_gameplay_events()?;
		Ok(())
	}

	fn stats(&mut self) -> Option<Vec<String>> {
		let mut stats = vec![format!("Number of entities: {}", self.world.len())];
		for system in &mut self.systems {
			if let Some(mut system_stats) = system.stats(&mut self.gameplay_ctx, &mut self.world) {
				stats.append(&mut system_stats);
			}
		}
		Some(stats)
	}

	fn event(&mut self, event: &Event) -> anyhow::Result<()> {
		for system in &mut self.systems {
			system.event(&mut self.gameplay_ctx, &mut self.world, event)?;
		}
		self.dispatch_gameplay_events()?;
		Ok(())
	}

	fn update(&mut self, delta_time: Duration) -> anyhow::Result<()> {
		for system in &mut self.systems {
			system.update(&mut self.gameplay_ctx, &mut self.world, delta_time)?;
		}
		self.dispatch_gameplay_events()?;
		Ok(())
	}

	fn draw(&mut self) -> anyhow::Result<()> {
		for system in &mut self.systems {
			system.draw(&mut self.gameplay_ctx, &mut self.world)?;
		}
		self.gameplay_ctx
			.world_command_buffer
			.run_on(&mut self.world);
		Ok(())
	}

	fn scene_change(&mut self) -> Option<SceneChange> {
		self.gameplay_ctx.scene_change.take()
	}
}
