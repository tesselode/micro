use std::time::Duration;

use crate::{Context, Event /* event::Event */};

/// The entrypoint for a Micro application.
#[allow(unused_variables)]
pub trait App {
	/// Returns a series of statistics that should be shown in the top right when the
	/// dev tools are open.
	fn debug_stats(&mut self, ctx: &mut Context) -> Option<Vec<String>> {
		None
	}

	/// A callback for adding items to the top-left of the main menu when the
	/// dev tools are open.
	fn debug_menu(&mut self, ctx: &mut Context, ui: &mut crate::egui::Ui) {}

	/// A callback for rendering egui components in general.
	fn debug_ui(&mut self, ctx: &mut Context, egui_ctx: &crate::egui::Context) {}

	/// Called when various events occur.
	fn event(&mut self, ctx: &mut Context, event: Event) {}

	/// Called on every tick of the game loop. `delta_time` is the amount of time that's elapsed
	/// since the last frame. Business logic should go here.
	fn update(&mut self, ctx: &mut Context, delta_time: Duration) {}

	/// Called on every tick of the game loop. Code related to drawing things on screen
	/// should go here.
	fn draw(&mut self, ctx: &mut Context) {}

	/// Called on every tick of the game loop after drawing operations have been presented
	/// to the screen.
	fn post_draw(&mut self, ctx: &mut Context) {}
}
