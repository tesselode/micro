mod context;

use std::time::Duration;

use micro::{Event, Micro};
use micro_ecs::{Ecs, EcsBuilder};
use micro_scene_manager::{Scene, SceneChange};

use crate::{event::GameEvent, globals::Globals};

use self::context::GameplayContext;

pub struct Gameplay {
	gameplay_ctx: GameplayContext,
	ecs: Ecs<Globals, GameplayContext, GameEvent>,
}

impl Gameplay {
	pub fn new(micro: &mut Micro, globals: &mut Globals) -> Self {
		let mut gameplay_ctx = GameplayContext::new();
		let ecs = EcsBuilder::new().build(micro, globals, &mut gameplay_ctx);
		Self { gameplay_ctx, ecs }
	}
}

impl Scene<Globals> for Gameplay {
	fn name(&self) -> &'static str {
		"Gameplay"
	}

	fn debug_stats(&mut self, _micro: &mut Micro, _globals: &mut Globals) -> Option<Vec<String>> {
		Some(vec![format!(
			"Number of entities: {}",
			self.ecs.world.len()
		)])
	}

	fn debug_ui(
		&mut self,
		micro: &mut Micro,
		egui_ctx: &micro::egui::Context,
		globals: &mut Globals,
	) {
		self.ecs
			.debug_ui(micro, egui_ctx, globals, &mut self.gameplay_ctx);
	}

	fn event(&mut self, micro: &mut Micro, globals: &mut Globals, event: &Event) {
		self.ecs
			.event(micro, globals, &mut self.gameplay_ctx, event);
	}

	fn update(&mut self, micro: &mut Micro, globals: &mut Globals, delta_time: Duration) {
		self.ecs
			.update(micro, globals, &mut self.gameplay_ctx, delta_time);
		self.ecs
			.update_cosmetic(micro, globals, &mut self.gameplay_ctx, delta_time);
	}

	fn draw(&mut self, micro: &mut Micro, globals: &mut Globals) {
		self.ecs.draw(micro, globals, &mut self.gameplay_ctx);
	}

	fn post_draw(&mut self, micro: &mut Micro, globals: &mut Globals) {
		self.ecs.post_draw(micro, globals, &mut self.gameplay_ctx);
	}

	fn scene_change(&mut self) -> Option<SceneChange<Globals>> {
		self.gameplay_ctx.scene_change.take()
	}
}
