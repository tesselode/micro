use std::error::Error;

use glam::{Mat4, Vec2, Vec3};
use micro::{
	graphics::{
		color::Rgba,
		mesh::{Mesh, Vertex},
		text::{Font, FontSettings, Text},
		texture::{Texture, TextureFilter, TextureSettings, TextureWrapping},
		DrawParams,
	},
	Context, Game, State,
};

struct MainState {
	text: Text,
}

impl MainState {
	fn new(ctx: &mut Context) -> Result<Self, Box<dyn Error>> {
		let font = Font::from_file(
			ctx,
			"examples/Roboto-Regular.ttf",
			FontSettings {
				scale: 128.0,
				texture_settings: TextureSettings {
					minifying_filter: TextureFilter::Linear,
					..Default::default()
				},
				..Default::default()
			},
		)?;
		let text = Text::new(ctx, &font, "hello world!")?;
		Ok(Self { text })
	}
}

impl State<Box<dyn Error>> for MainState {
	fn draw(&mut self, ctx: &mut Context) -> Result<(), Box<dyn Error>> {
		ctx.clear(Rgba::BLACK);
		self.text.draw(
			ctx,
			DrawParams::new().transform(Mat4::from_scale(Vec3::new(0.5, 0.5, 0.5))),
		);
		Ok(())
	}
}

fn main() -> Result<(), Box<dyn Error>> {
	Game::init()?.run(MainState::new)?;
	Ok(())
}
