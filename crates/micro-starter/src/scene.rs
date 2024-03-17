pub mod gameplay;

use std::time::Duration;

use micro::Event;

use crate::scene_manager::SceneChange;

#[allow(unused_variables)]
pub trait Scene {
	fn transparent(&self) -> bool {
		false
	}

	fn scene_change(&mut self) -> Option<SceneChange> {
		None
	}

	fn ui(&mut self, egui_ctx: &egui::Context) -> anyhow::Result<()> {
		Ok(())
	}

	fn menu(&mut self, ui: &mut egui::Ui) -> anyhow::Result<()> {
		Ok(())
	}

	fn stats(&mut self) -> Option<Vec<String>> {
		None
	}

	fn event(&mut self, event: &Event) -> anyhow::Result<()> {
		Ok(())
	}

	fn update(&mut self, delta_time: Duration) -> anyhow::Result<()> {
		Ok(())
	}

	fn draw(&mut self) -> anyhow::Result<()> {
		Ok(())
	}

	fn pause(&mut self) -> anyhow::Result<()> {
		Ok(())
	}

	fn resume(&mut self) -> anyhow::Result<()> {
		Ok(())
	}
}
