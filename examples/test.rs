use glam::UVec2;
use micro::{window::WindowMode, Context, ContextSettings, State};

struct MainState;

impl State for MainState {
	fn ui(&mut self, _ctx: &mut Context, egui_ctx: &egui::Context) {
		egui::CentralPanel::default().show(egui_ctx, |ui| {
			ui.add(egui::Label::new("Hello World!"));
			ui.label("A shorter and more convenient way to add a label.");
			if ui.button("Click me").clicked() {
				// take some action here
			}
		});
	}
}

fn main() {
	micro::run(
		ContextSettings {
			window_mode: WindowMode::Windowed {
				size: UVec2::new(500, 500),
			},
			resizable: true,
			..Default::default()
		},
		|_| MainState,
	)
}
