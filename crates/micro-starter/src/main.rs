mod globals;
mod input;
mod resources;
mod scene;
mod scene_manager;

use std::{collections::VecDeque, time::Duration};

use egui::TopBottomPanel;
use glam::UVec2;
use globals::Globals;
use micro::{input::Scancode, Context, ContextSettings, Event, State, WindowMode};
use scene::gameplay::Gameplay;
use scene_manager::SceneManager;

const NUM_FRAME_TIMES_TO_RECORD: usize = 30;

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
	frame_times: VecDeque<Duration>,
}

impl MainState {
	fn new(ctx: &mut Context) -> anyhow::Result<Self> {
		let mut globals = Globals::new(ctx)?;
		let gameplay = Gameplay::new(ctx, &mut globals)?;
		Ok(Self {
			globals,
			scene_manager: SceneManager::new(gameplay),
			dev_tools_enabled: false,
			frame_times: VecDeque::with_capacity(NUM_FRAME_TIMES_TO_RECORD),
		})
	}

	fn record_frame_time(&mut self, delta_time: Duration) {
		if self.frame_times.len() >= NUM_FRAME_TIMES_TO_RECORD {
			self.frame_times.pop_back();
		}
		self.frame_times.push_front(delta_time);
	}
}

impl State<anyhow::Error> for MainState {
	fn ui(&mut self, ctx: &mut Context, egui_ctx: &egui::Context) -> anyhow::Result<()> {
		if !self.dev_tools_enabled {
			return Ok(());
		}
		TopBottomPanel::top("menu").show(egui_ctx, |ui| -> anyhow::Result<()> {
			egui::menu::bar(ui, |ui| -> anyhow::Result<()> {
				self.scene_manager.menu(ctx, ui, &mut self.globals)?;
				ui.separator();
				let average_frame_time =
					self.frame_times.iter().sum::<Duration>() / NUM_FRAME_TIMES_TO_RECORD as u32;
				ui.label(&format!(
					"Average frame time: {:.1}ms ({:.0} FPS)",
					average_frame_time.as_secs_f64() * 1000.0,
					1.0 / average_frame_time.as_secs_f64()
				));
				if let Some(stats) = self.scene_manager.stats(ctx, &mut self.globals) {
					for stat in &stats {
						ui.separator();
						ui.label(stat);
					}
				}
				Ok(())
			})
			.inner
		});
		self.scene_manager.ui(ctx, egui_ctx, &mut self.globals)?;
		Ok(())
	}

	fn event(&mut self, ctx: &mut Context, event: Event) -> anyhow::Result<()> {
		if let Event::KeyPressed {
			key: Scancode::Escape,
			..
		} = event
		{
			ctx.quit();
		}
		if let Event::KeyPressed {
			key: Scancode::F1, ..
		} = event
		{
			self.dev_tools_enabled = !self.dev_tools_enabled;
		}
		self.scene_manager.event(ctx, &mut self.globals, event)?;
		Ok(())
	}

	fn update(&mut self, ctx: &mut Context, delta_time: Duration) -> anyhow::Result<()> {
		self.globals.input.update(ctx);
		self.scene_manager
			.update(ctx, &mut self.globals, delta_time)?;
		self.record_frame_time(delta_time);
		Ok(())
	}

	fn draw(&mut self, ctx: &mut Context) -> anyhow::Result<()> {
		self.scene_manager.draw(ctx, &mut self.globals)?;
		Ok(())
	}
}
