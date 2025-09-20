#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod dirs;
mod globals;
mod input;
mod scene;

use std::time::Duration;

use globals::Globals;
use micro::math::UVec2;
use micro::{App, Context, ContextSettings, Event, WindowMode, input::Scancode};
use micro_scene_manager::SceneManager;
use scene::gameplay::Gameplay;

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
		Game::new,
	);
}

struct Game {
	globals: Globals,
	scene_manager: SceneManager<Globals>,
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
	fn debug_stats(&mut self, ctx: &mut Context) -> Option<Vec<String>> {
		self.scene_manager.debug_stats(ctx, &mut self.globals)
	}

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
