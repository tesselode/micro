use std::error::Error;

use egui::ComboBox;
use glam::Vec2;
use micro::{
	graphics::{
		mesh::{Mesh, ShapeStyle},
		shader::Shader,
		ColorConstants,
	},
	math::Circle,
	Context, ContextSettings, State,
};
use palette::LinSrgba;

fn main() {
	micro::run(ContextSettings::default(), MainState::new);
}

struct MainState {
	shader: Shader,
}

impl MainState {
	pub fn new(ctx: &mut Context) -> Result<Self, Box<dyn Error>> {
		Ok(Self {
			shader: Shader::from_combined_str(ctx, include_str!("shader.glsl"))?,
		})
	}
}

impl State<Box<dyn Error>> for MainState {
	fn ui(&mut self, _ctx: &mut Context, egui_ctx: &egui::Context) -> Result<(), Box<dyn Error>> {
		egui::Window::new("test").show(egui_ctx, |ui| {
			ComboBox::new("test_box", "Test combo box")
				.show_index(ui, &mut 0, 100, |i| i.to_string())
		});
		Ok(())
	}

	fn draw(&mut self, ctx: &mut Context) -> Result<(), Box<dyn Error>> {
		ctx.clear(LinSrgba::BLACK);
		Mesh::circle(
			ctx,
			ShapeStyle::Fill,
			Circle {
				center: Vec2::new(50.0, 50.0),
				radius: 20.0,
			},
			LinSrgba::WHITE,
		)?
		.draw(ctx, &self.shader);
		Ok(())
	}
}
