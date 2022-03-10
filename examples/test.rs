use std::error::Error;

use fontdue::{Font, FontSettings};
use micro::{
	color::Rgba, context::Context, draw_params::DrawParams, image_data::ImageData,
	texture::Texture, Game, State,
};

struct MainState {
	texture: Texture,
}

impl MainState {
	fn new(ctx: &mut Context) -> Result<Self, Box<dyn Error>> {
		let font = Font::from_bytes(
			include_bytes!("Roboto-Regular.ttf") as &[u8],
			FontSettings {
				scale: 40.0,
				..Default::default()
			},
		)?;
		let (metrics, bitmap) = font.rasterize('a', 40.0);
		let image_data = ImageData {
			width: metrics.width.try_into().expect("Font bitmap is too wide"),
			height: metrics.height.try_into().expect("Font bitmap is too tall"),
			pixels: {
				let mut pixels = Vec::with_capacity(bitmap.len() * 4);
				for alpha in bitmap {
					pixels.extend_from_slice(&[255, 255, 255, alpha]);
				}
				pixels
			},
		};
		let texture = Texture::from_image_data(ctx, &image_data)?;
		Ok(Self { texture })
	}
}

impl State<Box<dyn Error>> for MainState {
	fn draw(&mut self, ctx: &mut Context) -> Result<(), Box<dyn Error>> {
		ctx.clear(Rgba::new(0.1, 0.2, 0.3, 1.0));
		self.texture.draw(ctx, DrawParams::new())?;
		Ok(())
	}
}

fn main() -> Result<(), Box<dyn Error>> {
	Game::init()?.run(MainState::new)?;
	Ok(())
}
