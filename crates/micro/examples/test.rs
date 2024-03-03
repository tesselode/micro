use std::error::Error;

use glam::{Mat4, Vec2, Vec3};
use micro::{
	clear,
	graphics::{
		mesh::{Mesh, ShapeStyle},
		text::{Font, FontSettings, LayoutSettings, Text},
		texture::{Texture, TextureSettings},
		Canvas, CanvasSettings, ColorConstants, StencilAction, StencilTest,
	},
	math::{Circle, Rect},
	use_stencil, window_size, with_canvas, with_transform, write_to_stencil, ContextSettings,
	State,
};
use palette::LinSrgba;

fn main() {
	micro::run(ContextSettings::default(), MainState::new);
}

struct MainState {
	texture: Texture,
	font: Font,
	canvas: Canvas,
}

impl MainState {
	pub fn new() -> Result<Self, Box<dyn Error>> {
		Ok(Self {
			texture: Texture::from_file(
				"resources/spritesheet_default.png",
				TextureSettings::default(),
			)?,
			font: Font::from_file("resources/Abaddon Bold.ttf", FontSettings::default())?,
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
					.draw();
				});
			});
			use_stencil!(StencilTest::Equal, 1, {
				self.texture
					.draw()
					.region(Rect::new(Vec2::ZERO, Vec2::splat(50.0)));
			});
			Text::new(&self.font, "Hello, world!", LayoutSettings::default())
				.draw()
				.color(LinSrgba::RED)
				.scaled_2d(Vec2::splat(5.0));
		});

		clear(LinSrgba::BLACK);
		self.canvas.draw();

		Ok(())
	}
}
