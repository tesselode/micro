pub mod gameplay;

use std::time::Duration;

use micro::Event;

use crate::{globals::Globals, scene_manager::SceneChange};

#[allow(unused_variables)]
pub trait Scene {
	fn transparent(&self) -> bool {
		false
	}

	fn scene_change(&mut self) -> Option<SceneChange> {
		None
	}

	fn ui(&mut self, egui_ctx: &egui::Context, globals: &mut Globals) -> anyhow::Result<()> {
		Ok(())
	}

	fn menu(&mut self, ui: &mut egui::Ui, globals: &mut Globals) -> anyhow::Result<()> {
		Ok(())
	}

	fn stats(&mut self, globals: &mut Globals) -> Option<Vec<String>> {
		None
	}

	fn event(&mut self, globals: &mut Globals, event: &Event) -> anyhow::Result<()> {
		Ok(())
	}

	fn update(&mut self, globals: &mut Globals, delta_time: Duration) -> anyhow::Result<()> {
		Ok(())
	}

	fn draw(&mut self, globals: &mut Globals) -> anyhow::Result<()> {
		Ok(())
	}

	fn pause(&mut self, globals: &mut Globals) -> anyhow::Result<()> {
		Ok(())
	}

	fn resume(&mut self, globals: &mut Globals) -> anyhow::Result<()> {
		Ok(())
	}
}
