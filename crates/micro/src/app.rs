use std::time::Duration;

use crate::{Context, event::Event};

#[allow(unused_variables)]
pub trait App {
	fn debug_stats(&mut self, ctx: &mut Context) -> Option<Vec<String>> {
		None
	}

	fn debug_menu(&mut self, ctx: &mut Context, ui: &mut crate::egui::Ui) -> anyhow::Result<()> {
		Ok(())
	}

	fn debug_ui(
		&mut self,
		ctx: &mut Context,
		egui_ctx: &crate::egui::Context,
	) -> anyhow::Result<()> {
		Ok(())
	}

	fn event(&mut self, ctx: &mut Context, event: Event) -> anyhow::Result<()> {
		Ok(())
	}

	fn update(&mut self, ctx: &mut Context, delta_time: Duration) -> anyhow::Result<()> {
		Ok(())
	}

	fn draw(&mut self, ctx: &mut Context) -> anyhow::Result<()> {
		Ok(())
	}
}
