use std::error::Error;

use glam::Vec2;
use micro::{
	graphics::{
		mesh::{Mesh, ShapeStyle},
		texture::{Texture, TextureSettings},
		ColorConstants, DrawParams,
	},
	input::Scancode,
	math::Circle,
	Context, ContextSettings, Event, State,
};
use palette::LinSrgba;

fn main() {
	micro::run(ContextSettings::default(), MainState::new);
}

struct MainState {
	mesh: Mesh,
	screenshot: Option<Texture>,
}

impl MainState {
	fn new(ctx: &mut Context) -> Result<Self, Box<dyn Error>> {
		Ok(Self {
			mesh: Mesh::circle(
				ctx,
				ShapeStyle::Fill,
				Circle {
					center: Vec2::splat(400.0),
					radius: 50.0,
				},
				LinSrgba::WHITE,
			)?,
			screenshot: None,
		})
	}
}

impl State<Box<dyn Error>> for MainState {
	fn event(&mut self, ctx: &mut Context, event: Event) -> Result<(), Box<dyn Error>> {
		if let Event::KeyPressed {
			key: Scancode::Space,
			..
		} = event
		{
			self.screenshot = Some(Texture::from_image_data(
				ctx,
				&ctx.take_screenshot(),
				TextureSettings::default(),
			));
		}
		Ok(())
	}

	fn draw(&mut self, ctx: &mut Context) -> Result<(), Box<dyn Error>> {
		self.mesh.draw(ctx, DrawParams::new());
		if let Some(screenshot) = &self.screenshot {
			screenshot.draw(ctx, DrawParams::new().scaled(Vec2::splat(0.5)));
		}
		Ok(())
	}
}
