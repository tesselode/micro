use std::time::Duration;

use crate::{Context, event::Event};

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
	fn debug_menu(&mut self, ctx: &mut Context, ui: &mut crate::egui::Ui) -> anyhow::Result<()> {
		Ok(())
	}

	/// A callback for rendering egui components in general.
	fn debug_ui(
		&mut self,
		ctx: &mut Context,
		egui_ctx: &crate::egui::Context,
	) -> anyhow::Result<()> {
		Ok(())
	}

	/// Called when various events occur.
	fn event(&mut self, ctx: &mut Context, event: Event) -> anyhow::Result<()> {
		Ok(())
	}

	/// Called on every tick of the game loop. `delta_time` is the amount of time that's elapsed
	/// since the last frame. Business logic should go here.
	fn update(&mut self, ctx: &mut Context, delta_time: Duration) -> anyhow::Result<()> {
		Ok(())
	}

	/// Called on every tick of the game loop. Code related to drawing things on screen
	/// should go here.
	fn draw(&mut self, ctx: &mut Context) -> anyhow::Result<()> {
		Ok(())
	}
}
