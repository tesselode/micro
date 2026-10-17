#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod dirs;
mod event;
mod globals;
mod input;
mod log;
mod scene;

use std::time::Duration;

use backtrace::Backtrace;
use globals::Globals;
use log::setup_logging;
use micro::math::UVec2;
use micro::{App, Event, WindowMode, input::Scancode};
use micro::{Micro, MicroSettings};
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

	micro::run(
		MicroSettings {
			window_title: "Game".to_string(),
			window_mode: WindowMode::Windowed {
				size: UVec2::new(1920, 1080),
			},
			resizable: true,
			..Default::default()
		},
		Game::new,
	);
}

struct Game {
	globals: Globals,
	scene_manager: SceneManager<Globals>,
}

impl Game {
	fn new(micro: &mut Micro) -> Self {
		let mut globals = Globals::new(micro);
		let gameplay = Gameplay::new(micro, &mut globals);
		Self {
			globals,
			scene_manager: SceneManager::new(gameplay),
		}
	}
}

impl App for Game {
	fn debug_stats(&mut self, micro: &mut Micro) -> Option<Vec<String>> {
		self.scene_manager.debug_stats(micro, &mut self.globals)
	}

	fn debug_menu(&mut self, micro: &mut Micro, ui: &mut micro::egui::Ui) {
		self.scene_manager.debug_menu(micro, ui, &mut self.globals)
	}

	fn debug_ui(&mut self, micro: &mut Micro, egui_ctx: &micro::egui::Context) {
		self.scene_manager
			.debug_ui(micro, egui_ctx, &mut self.globals)
	}

	fn event(&mut self, micro: &mut Micro, event: Event) {
		if let Event::KeyPressed {
			key: Scancode::Escape,
			..
		} = event
		{
			micro.quit();
		}
		self.scene_manager.event(micro, &mut self.globals, event)
	}

	fn update(&mut self, micro: &mut Micro, delta_time: Duration) {
		self.globals.input.update(micro);
		self.scene_manager
			.update(micro, &mut self.globals, delta_time)
	}

	fn draw(&mut self, micro: &mut Micro) {
		self.scene_manager.draw(micro, &mut self.globals)
	}

	fn post_draw(&mut self, micro: &mut Micro) {
		self.scene_manager.post_draw(micro, &mut self.globals)
	}
}
