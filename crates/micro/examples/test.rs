use glam::{UVec2, Vec2};
use micro::{
	graphics::{
		color::Rgba,
		mesh::{Mesh, ShapeStyle},
		DrawParams,
	},
	input::Scancode,
	window::WindowMode,
	Context, ContextSettings, Event, State,
};

struct MainState {
	mesh: Mesh,
}

impl MainState {
	fn new(ctx: &mut Context) -> Self {
		Self {
			mesh: Mesh::circle(
				ctx,
				ShapeStyle::Fill,
				Vec2::splat(1280.0 / 2.0),
				1280.0 / 2.0,
			),
		}
	}
}

impl State for MainState {
	fn ui(&mut self, ctx: &mut Context, egui_ctx: &egui::Context) {
		egui::Window::new("Test window").show(egui_ctx, |ui| {
			ui.label("hello, world!");
		});
	}

	fn event(&mut self, ctx: &mut Context, event: Event) {
		if let Event::KeyPressed(Scancode::Space) = event {
			ctx.set_background_color(Rgba::new(0.25, 0.25, 0.25, 1.0));
			ctx.set_window_mode(WindowMode::Windowed {
				size: UVec2::new(1280, 720),
			});
		}
	}

	fn draw(&mut self, ctx: &mut Context) {
		self.mesh.draw(ctx, DrawParams::new());
	}
}

fn main() {
	micro::run(ContextSettings::default(), MainState::new)
}
