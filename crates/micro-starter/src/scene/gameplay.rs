mod component;
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
	pub fn new(ctx: &mut Context, globals: &mut Globals) -> Self {
		let gameplay_ctx = GameplayContext::new();
		macro_rules! systems {
			($($system:expr),*$(,)?) => {
				vec![
					$(Box::new($system)),*
				]
			};
		}
		let systems: Vec<Box<dyn System>> = systems![];
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
			);
		}
		gameplay.dispatch_gameplay_events(ctx, globals);
		gameplay
	}

	fn dispatch_gameplay_events(&mut self, ctx: &mut Context, globals: &mut Globals) {
		while let Some(event) = self.gameplay_ctx.event_queue.pop_front() {
			for system in &mut self.systems {
				system.gameplay_event(
					ctx,
					globals,
					&mut self.gameplay_ctx,
					&mut self.world,
					&event,
				);
			}
		}
	}
}

impl Scene for Gameplay {
	fn debug_ui(
		&mut self,
		ctx: &mut Context,
		egui_ctx: &micro::egui::Context,
		globals: &mut Globals,
	) {
		for system in &mut self.systems {
			system.debug_ui(
				ctx,
				egui_ctx,
				globals,
				&mut self.gameplay_ctx,
				&mut self.world,
			);
		}
		self.dispatch_gameplay_events(ctx, globals);
	}

	fn debug_menu(&mut self, ctx: &mut Context, ui: &mut micro::egui::Ui, globals: &mut Globals) {
		for system in &mut self.systems {
			system.debug_menu(ctx, ui, globals, &mut self.gameplay_ctx, &mut self.world);
		}
		self.dispatch_gameplay_events(ctx, globals);
	}

	fn debug_stats(&mut self, ctx: &mut Context, globals: &mut Globals) -> Option<Vec<String>> {
		let mut stats = vec![format!("Number of entities: {}", self.world.len())];
		for system in &mut self.systems {
			if let Some(mut system_stats) =
				system.debug_stats(ctx, globals, &mut self.gameplay_ctx, &mut self.world)
			{
				stats.append(&mut system_stats);
			}
		}
		Some(stats)
	}

	fn event(&mut self, ctx: &mut Context, globals: &mut Globals, event: &Event) {
		for system in &mut self.systems {
			system.event(ctx, globals, &mut self.gameplay_ctx, &mut self.world, event);
		}
		self.dispatch_gameplay_events(ctx, globals);
	}

	fn update(&mut self, ctx: &mut Context, globals: &mut Globals, delta_time: Duration) {
		for system in &mut self.systems {
			system.update(
				ctx,
				globals,
				&mut self.gameplay_ctx,
				&mut self.world,
				delta_time,
			);
		}
		self.dispatch_gameplay_events(ctx, globals);
	}

	fn draw(&mut self, ctx: &mut Context, globals: &mut Globals) {
		for system in &mut self.systems {
			system.draw(ctx, globals, &mut self.gameplay_ctx, &mut self.world);
		}
		self.gameplay_ctx
			.world_command_buffer
			.run_on(&mut self.world);
	}

	fn scene_change(&mut self) -> Option<SceneChange> {
		self.gameplay_ctx.scene_change.take()
	}
}
