use std::error::Error;

use glam::Vec2;
use micro::{
	graphics::{
		mesh::{Mesh, ShapeStyle},
		text::{Font, FontSettings, LayoutSettings, Text},
		texture::{Texture, TextureSettings},
		DrawParams,
	},
	Context, ContextSettings, State,
};
use palette::LinSrgba;

struct MainState {
	text: Text,
	mesh: Mesh,
	texture: Texture,
}

impl MainState {
	fn new(ctx: &mut Context) -> Result<Self, Box<dyn Error>> {
		let font = Font::from_file(
			ctx,
			"crates/micro/examples/Roboto-Regular.ttf",
			FontSettings::default(),
		)?;
		let text = Text::new(
			ctx,
			&font,
			"The quick brown fox jumps over the lazy dog.",
			LayoutSettings::default(),
		)?;
		let rect = text.character_bounds(5..10).unwrap();
		let mesh = Mesh::styled_rectangle(
			ctx,
			ShapeStyle::Stroke(2.0),
			rect,
			LinSrgba::new(1.0, 0.0, 0.0, 1.0),
		)?;
		Ok(Self {
			text,
			mesh,
			texture: Texture::from_file(
				ctx,
				"crates/micro/examples/tree.png",
				TextureSettings::default(),
			)?,
		})
	}
}

impl State<Box<dyn Error>> for MainState {
	fn draw(&mut self, ctx: &mut Context) -> Result<(), Box<dyn Error>> {
		self.text.draw(ctx, DrawParams::new());
		self.mesh.draw(ctx, DrawParams::new());
		self.texture.draw(ctx, Vec2::splat(100.0).extend(0.0));
		Ok(())
	}
}

fn main() {
	micro::run(ContextSettings::default(), MainState::new)
}
