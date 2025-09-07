pub mod gameplay;

use std::time::Duration;

use micro::{Context, Event};

use crate::{globals::Globals, scene_manager::SceneChange};

#[allow(unused_variables)]
pub trait Scene {
	fn transparent(&self) -> bool {
		false
	}

	fn scene_change(&mut self) -> Option<SceneChange> {
		None
	}

	fn debug_ui(
		&mut self,
		ctx: &mut Context,
		egui_ctx: &micro::egui::Context,
		globals: &mut Globals,
	) {
	}

	fn debug_menu(&mut self, ctx: &mut Context, ui: &mut micro::egui::Ui, globals: &mut Globals) {}

	fn debug_stats(&mut self, ctx: &mut Context, globals: &mut Globals) -> Option<Vec<String>> {
		None
	}

	fn event(&mut self, ctx: &mut Context, globals: &mut Globals, event: &Event) {}

	fn update(&mut self, ctx: &mut Context, globals: &mut Globals, delta_time: Duration) {}

	fn draw(&mut self, ctx: &mut Context, globals: &mut Globals) {}

	fn pause(&mut self, ctx: &mut Context, globals: &mut Globals) {}

	fn resume(&mut self, ctx: &mut Context, globals: &mut Globals) {}

	fn leave(&mut self, ctx: &mut Context, globals: &mut Globals) {}
}
