mod dirs;
mod input;
mod log;
mod scene;
mod scene_manager;

use std::time::Duration;

use backtrace::Backtrace;
use input::{default_input_config, Input};
use log::setup_logging;
use micro::color::LinSrgba;
use micro::math::UVec2;
use micro::resource::loader::{FontLoader, TextureLoader};
use micro::resource::Resources;
use micro::ui::TopBottomPanel;
use micro::Globals;
use micro::{
	color::ColorConstants, input::Scancode, Context, ContextSettings, Event, State, WindowMode,
};
use scene::gameplay::Gameplay;
use scene_manager::SceneManager;

fn main() {
	#[cfg(debug_assertions)]
	setup_logging();
	#[cfg(not(debug_assertions))]
	let _guard = setup_logging(&settings);
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
		MainState::new,
	)
}

struct MainState {
	scene_manager: SceneManager,
	dev_tools_enabled: bool,
}

impl MainState {
	fn new(ctx: &mut Context, globals: &mut Globals) -> anyhow::Result<Self> {
		globals.add(Input::new(default_input_config(), ctx.gamepad(0)));
		globals.add(Resources::autoloaded(
			ctx,
			"texture",
			TextureLoader::default(),
		));
		globals.add(Resources::autoloaded(ctx, "font", FontLoader::default()));
		let gameplay = Gameplay::new(ctx, globals)?;
		Ok(Self {
			scene_manager: SceneManager::new(gameplay),
			dev_tools_enabled: false,
		})
	}
}

impl State<anyhow::Error> for MainState {
	fn ui(
		&mut self,
		ctx: &mut Context,
		globals: &mut Globals,
		egui_ctx: &micro::ui::Context,
	) -> anyhow::Result<()> {
		if !self.dev_tools_enabled {
			return Ok(());
		}
		TopBottomPanel::top("menu").show(egui_ctx, |ui| -> anyhow::Result<()> {
			micro::ui::menu::bar(ui, |ui| -> anyhow::Result<()> {
				self.scene_manager.menu(ctx, ui, globals)?;
				ui.separator();
				ui.label(&format!(
					"Average frame time: {:.1}ms ({:.0} FPS)",
					ctx.average_frame_time().as_secs_f64() * 1000.0,
					ctx.fps()
				));
				if let Some(stats) = self.scene_manager.stats(ctx, globals) {
					for stat in &stats {
						ui.separator();
						ui.label(stat);
					}
				}
				Ok(())
			})
			.inner
		});
		self.scene_manager.ui(ctx, egui_ctx, globals)?;
		Ok(())
	}

	fn event(
		&mut self,
		ctx: &mut Context,
		globals: &mut Globals,
		event: Event,
	) -> anyhow::Result<()> {
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
		self.scene_manager.event(ctx, globals, event)?;
		Ok(())
	}

	fn update(
		&mut self,
		ctx: &mut Context,
		globals: &mut Globals,
		delta_time: Duration,
	) -> anyhow::Result<()> {
		globals.get_mut::<Input>().update(ctx);
		self.scene_manager.update(ctx, globals, delta_time)?;
		Ok(())
	}

	fn draw(&mut self, ctx: &mut Context, globals: &mut Globals) -> anyhow::Result<()> {
		ctx.clear(LinSrgba::BLACK);
		self.scene_manager.draw(ctx, globals)?;
		Ok(())
	}
}
