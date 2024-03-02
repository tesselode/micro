use std::error::Error;

use egui::ComboBox;
use glam::Vec2;
use micro::{
	clear,
	graphics::{
		mesh::{Mesh, ShapeStyle},
		ColorConstants,
	},
	math::Circle,
	resource::{loader::ShaderLoader, Resources},
	ContextSettings, State,
};
use palette::LinSrgba;

fn main() {
	micro::run(ContextSettings::default(), MainState::new);
}

struct MainState {
	shaders: Resources<ShaderLoader>,
}

impl MainState {
	pub fn new() -> Result<Self, Box<dyn Error>> {
		Ok(Self {
			shaders: {
				let shaders = Resources::autoloaded("", ShaderLoader);
				shaders["shader"].send_f32("scale", 2.0)?;
				shaders
			},
		})
	}
}

impl State<Box<dyn Error>> for MainState {
	fn ui(&mut self, egui_ctx: &egui::Context) -> Result<(), Box<dyn Error>> {
		egui::Window::new("test").show(egui_ctx, |ui| {
			ComboBox::new("test_box", "Test combo box")
				.show_index(ui, &mut 0, 100, |i| i.to_string())
		});
		Ok(())
	}

	fn update(&mut self, delta_time: std::time::Duration) -> Result<(), Box<dyn Error>> {
		self.shaders.update_hot_reload(delta_time);
		Ok(())
	}

	fn draw(&mut self) -> Result<(), Box<dyn Error>> {
		clear(LinSrgba::BLACK);
		Mesh::circle(
			ShapeStyle::Fill,
			Circle {
				center: Vec2::new(50.0, 50.0),
				radius: 20.0,
			},
			LinSrgba::WHITE,
		)?
		.draw(&self.shaders["shader"]);
		Ok(())
	}
}
