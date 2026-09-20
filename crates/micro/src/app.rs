use std::time::Duration;

use crate::{Micro, event::Event};

/// The entrypoint for a Micro application.
#[allow(unused_variables)]
pub trait App {
	/// Returns a series of statistics that should be shown in the top right when the
	/// dev tools are open.
	fn debug_stats(&mut self, micro: &mut Micro) -> Option<Vec<String>> {
		None
	}

	/// A callback for adding items to the top-left of the main menu when the
	/// dev tools are open.
	fn debug_menu(&mut self, micro: &mut Micro, ui: &mut crate::egui::Ui) {}

	/// A callback for rendering egui components in general.
	fn debug_ui(&mut self, micro: &mut Micro, egui_ctx: &crate::egui::Context) {}

	/// Called when various events occur.
	fn event(&mut self, micro: &mut Micro, event: Event) {}

	/// Called on every tick of the game loop. `delta_time` is the amount of time that's elapsed
	/// since the last frame. Business logic should go here.
	fn update(&mut self, micro: &mut Micro, delta_time: Duration) {}

	/// Called on every tick of the game loop. Code related to drawing things on screen
	/// should go here.
	fn draw(&mut self, micro: &mut Micro) {}

	/// Called on every tick of the game loop after drawing operations have been presented
	/// to the screen.
	fn post_draw(&mut self, micro: &mut Micro) {}
}
