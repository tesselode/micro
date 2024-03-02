use std::error::Error;

use egui::ComboBox;
use glam::{Mat4, Vec2, Vec3};
use micro::{
	clear,
	graphics::{
		mesh::{Mesh, ShapeStyle},
		Canvas, CanvasSettings, ColorConstants, DrawParams, StencilAction, StencilTest,
	},
	math::{Circle, Rect},
	resource::{loader::ShaderLoader, Resources},
	use_stencil, window_size, with_canvas, with_transform, write_to_stencil, ContextSettings,
	State,
};
use palette::LinSrgba;

fn main() {
	micro::run(ContextSettings::default(), MainState::new);
}

struct MainState {
	canvas: Canvas,
}

impl MainState {
	pub fn new() -> Result<Self, Box<dyn Error>> {
		Ok(Self {
			canvas: Canvas::new(window_size(), CanvasSettings::default()),
		})
	}
}

impl State<Box<dyn Error>> for MainState {
	fn draw(&mut self) -> Result<(), Box<dyn Error>> {
		with_canvas!(self.canvas, {
			clear(LinSrgba::BLACK);
			write_to_stencil!(StencilAction::Replace(1), {
				with_transform!(Mat4::from_scale(Vec3::splat(2.0)), {
					Mesh::circle(
						ShapeStyle::Fill,
						Circle {
							center: Vec2::splat(50.0),
							radius: 50.0,
						},
						LinSrgba::WHITE,
					)?
					.draw(DrawParams::new());
				});
			});
			use_stencil!(StencilTest::Equal, 1, {
				Mesh::rectangle(Rect::from_xywh(50.0, 50.0, 100.0, 150.0)).draw(DrawParams::new());
			});
		});

		clear(LinSrgba::BLACK);
		self.canvas.draw(DrawParams::new());

		Ok(())
	}
}
