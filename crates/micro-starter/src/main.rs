#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod dirs;
mod globals;
mod input;
mod log;
mod scene;
mod scene_manager;

use std::time::Duration;

use backtrace::Backtrace;
use globals::Globals;
use log::setup_logging;
use micro::math::UVec2;
use micro::{App, Context, ContextSettings, Event, WindowMode, input::Scancode};
use scene::gameplay::Gameplay;
use scene_manager::SceneManager;

fn main() {
	#[cfg(debug_assertions)]
	setup_logging();
	#[cfg(not(debug_assertions))]
	let _guard = setup_logging();
	std::panic::set_hook(Box::new(|info| {
		tracing::error!("{}\n{:?}", info, Backtrace::new())
	}));
	micro::run(
		ContextSettings {
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
	scene_manager: SceneManager,
}

impl Game {
	fn new(ctx: &mut Context) -> Self {
		let mut globals = Globals::new(ctx);
		let gameplay = Gameplay::new(ctx, &mut globals);
		Self {
			globals,
			scene_manager: SceneManager::new(gameplay),
		}
	}
}

impl App for Game {
	fn debug_ui(&mut self, ctx: &mut Context, egui_ctx: &micro::egui::Context) {
		self.scene_manager
			.debug_ui(ctx, egui_ctx, &mut self.globals);
	}

	fn event(&mut self, ctx: &mut Context, event: Event) {
		if let Event::KeyPressed {
			key: Scancode::Escape,
			..
		} = event
		{
			ctx.quit();
		}
		self.scene_manager.event(ctx, &mut self.globals, event);
	}

	fn update(&mut self, ctx: &mut Context, delta_time: Duration) {
		self.globals.input.update(ctx);
		self.scene_manager
			.update(ctx, &mut self.globals, delta_time);
	}

	fn draw(&mut self, ctx: &mut Context) {
		self.scene_manager.draw(ctx, &mut self.globals);
	}
}
