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
		egui_ctx: &micro::debug_ui::Context,
		globals: &mut Globals,
	) -> anyhow::Result<()> {
		Ok(())
	}

	fn debug_menu(
		&mut self,
		ctx: &mut Context,
		ui: &mut micro::debug_ui::Ui,
		globals: &mut Globals,
	) -> anyhow::Result<()> {
		Ok(())
	}

	fn debug_stats(&mut self, ctx: &mut Context, globals: &mut Globals) -> Option<Vec<String>> {
		None
	}

	fn event(
		&mut self,
		ctx: &mut Context,
		globals: &mut Globals,
		event: &Event,
	) -> anyhow::Result<()> {
		Ok(())
	}

	fn update(
		&mut self,
		ctx: &mut Context,
		globals: &mut Globals,
		delta_time: Duration,
	) -> anyhow::Result<()> {
		Ok(())
	}

	fn draw(&mut self, ctx: &mut Context, globals: &mut Globals) -> anyhow::Result<()> {
		Ok(())
	}

	fn pause(&mut self, ctx: &mut Context, globals: &mut Globals) -> anyhow::Result<()> {
		Ok(())
	}

	fn resume(&mut self, ctx: &mut Context, globals: &mut Globals) -> anyhow::Result<()> {
		Ok(())
	}

	fn leave(&mut self, ctx: &mut Context, globals: &mut Globals) -> anyhow::Result<()> {
		Ok(())
	}
}
