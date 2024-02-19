use std::error::Error;

use egui::ComboBox;
use glam::Vec2;
use micro::{
	graphics::{
		mesh::{Mesh, ShapeStyle},
		ColorConstants,
	},
	math::Circle,
	resource::{loader::ShaderLoader, Resources},
	Context, ContextSettings, State,
};
use palette::LinSrgba;

fn main() {
	micro::run(ContextSettings::default(), MainState::new);
}

struct MainState {
	shaders: Resources<ShaderLoader>,
}

impl MainState {
	pub fn new(ctx: &mut Context) -> Result<Self, Box<dyn Error>> {
		Ok(Self {
			shaders: {
				let mut shaders = Resources::autoloaded(ctx, "", ShaderLoader);
				shaders["shader"].send_f32(ctx, "scale", 2.0)?;
				shaders
			},
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

	fn update(
		&mut self,
		ctx: &mut Context,
		delta_time: std::time::Duration,
	) -> Result<(), Box<dyn Error>> {
		self.shaders.update_hot_reload(ctx, delta_time);
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
		.draw(ctx, &self.shaders["shader"]);
		Ok(())
	}
}
