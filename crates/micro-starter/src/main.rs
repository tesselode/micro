mod globals;
mod input;
mod scene;
mod scene_manager;

use std::time::Duration;

use egui::TopBottomPanel;
use glam::UVec2;
use globals::Globals;
use micro::{
	average_frame_time, clear, fps, graphics::ColorConstants, input::Scancode, quit,
	ContextSettings, Event, State, WindowMode,
};
use palette::LinSrgba;
use scene::gameplay::Gameplay;
use scene_manager::SceneManager;

fn main() {
	micro::run(
		ContextSettings {
			window_title: "Game".to_string(),
			window_mode: WindowMode::Windowed {
				size: UVec2::new(1920, 1080),
			},
			resizable: true,
			..Default::default()
		},
		MainState::new,
	)
}

struct MainState {
	globals: Globals,
	scene_manager: SceneManager,
	dev_tools_enabled: bool,
}

impl MainState {
	fn new() -> anyhow::Result<Self> {
		let mut globals = Globals::new()?;
		let gameplay = Gameplay::new(&mut globals)?;
		Ok(Self {
			globals,
			scene_manager: SceneManager::new(gameplay),
			dev_tools_enabled: false,
		})
	}
}

impl State<anyhow::Error> for MainState {
	fn ui(&mut self, egui_ctx: &egui::Context) -> anyhow::Result<()> {
		if !self.dev_tools_enabled {
			return Ok(());
		}
		TopBottomPanel::top("menu").show(egui_ctx, |ui| -> anyhow::Result<()> {
			egui::menu::bar(ui, |ui| -> anyhow::Result<()> {
				self.scene_manager.menu(ui, &mut self.globals)?;
				ui.separator();
				ui.label(&format!(
					"Average frame time: {:.1}ms ({:.0} FPS)",
					average_frame_time().as_secs_f64() * 1000.0,
					fps()
				));
				if let Some(stats) = self.scene_manager.stats(&mut self.globals) {
					for stat in &stats {
						ui.separator();
						ui.label(stat);
					}
				}
				Ok(())
			})
			.inner
		});
		self.scene_manager.ui(egui_ctx, &mut self.globals)?;
		Ok(())
	}

	fn event(&mut self, event: Event) -> anyhow::Result<()> {
		if let Event::KeyPressed {
			key: Scancode::Escape,
			..
		} = event
		{
			quit();
		}
		if let Event::KeyPressed {
			key: Scancode::F1, ..
		} = event
		{
			self.dev_tools_enabled = !self.dev_tools_enabled;
		}
		self.scene_manager.event(&mut self.globals, event)?;
		Ok(())
	}

	fn update(&mut self, delta_time: Duration) -> anyhow::Result<()> {
		self.globals.input.update();
		self.scene_manager.update(&mut self.globals, delta_time)?;
		Ok(())
	}

	fn draw(&mut self) -> anyhow::Result<()> {
		clear(LinSrgba::BLACK);
		self.scene_manager.draw(&mut self.globals)?;
		Ok(())
	}
}
