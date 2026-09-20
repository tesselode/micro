#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod dirs;
mod globals;
mod input;
mod log;
mod scene;

use std::time::Duration;

use backtrace::Backtrace;
use globals::Globals;
use log::setup_logging;
use micro::log_if_err;
use micro::math::UVec2;
use micro::{App, Context, ContextSettings, Event, WindowMode, input::Scancode};
use micro_scene_manager::SceneManager;
use scene::gameplay::Gameplay;

fn main() {
	#[cfg(debug_assertions)]
	setup_logging();
	#[cfg(not(debug_assertions))]
	let _guard = setup_logging();
	std::panic::set_hook(Box::new(|info| {
		tracing::error!("{}\n{:?}", info, Backtrace::new())
	}));

	log_if_err!(micro::run(
		ContextSettings {
			window_title: "Game".to_string(),
			window_mode: WindowMode::Windowed {
				size: UVec2::new(1920, 1080),
			},
			resizable: true,
			..Default::default()
		},
		Game::new,
	));
}

struct Game {
	globals: Globals,
	scene_manager: SceneManager<Globals>,
}

impl Game {
	fn new(micro: &mut Context) -> anyhow::Result<Self> {
		let mut globals = Globals::new(micro);
		let gameplay = Gameplay::new(micro, &mut globals)?;
		Ok(Self {
			globals,
			scene_manager: SceneManager::new(gameplay),
		})
	}
}

impl App for Game {
	fn debug_stats(&mut self, micro: &mut Context) -> Option<Vec<String>> {
		self.scene_manager.debug_stats(micro, &mut self.globals)
	}

	fn debug_ui(
		&mut self,
		micro: &mut Context,
		egui_ctx: &micro::egui::Context,
	) -> anyhow::Result<()> {
		self.scene_manager
			.debug_ui(micro, egui_ctx, &mut self.globals)
	}

	fn event(&mut self, micro: &mut Context, event: Event) -> anyhow::Result<()> {
		if let Event::KeyPressed {
			key: Scancode::Escape,
			..
		} = event
		{
			micro.quit();
		}
		self.scene_manager.event(micro, &mut self.globals, event)
	}

	fn update(&mut self, micro: &mut Context, delta_time: Duration) -> anyhow::Result<()> {
		self.globals.input.update(micro);
		self.scene_manager
			.update(micro, &mut self.globals, delta_time)
	}

	fn draw(&mut self, micro: &mut Context) -> anyhow::Result<()> {
		self.scene_manager.draw(micro, &mut self.globals)
	}
}
