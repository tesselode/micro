use glam::Vec2;

use crate::{
	graphics::{color::Rgba, texture::Texture, DrawParams},
	Context,
};

use super::{BuiltWidget, Constraints, Widget};

pub struct Image {
	pub texture: Texture,
	pub color: Rgba,
}

impl Image {
	pub fn new(texture: &Texture) -> Self {
		Self {
			texture: texture.clone(),
			color: Rgba::WHITE,
		}
	}

	pub fn with_color(self, color: Rgba) -> Self {
		Self { color, ..self }
	}
}

impl Widget for Image {
	fn build(&self, _ctx: &mut Context, constraints: Constraints) -> Box<dyn BuiltWidget> {
		Box::new(BuiltImage {
			texture: self.texture.clone(),
			size: self.texture.size().as_vec2().min(constraints.max_size),
			color: self.color,
		})
	}
}

struct BuiltImage {
	texture: Texture,
	size: Vec2,
	color: Rgba,
}

impl BuiltWidget for BuiltImage {
	fn size(&self) -> Vec2 {
		self.size
	}

	fn draw(&self, ctx: &mut Context) {
		self.texture.draw(
			ctx,
			DrawParams::new()
				.scale(self.size / self.texture.size().as_vec2())
				.color(self.color),
		)
	}
}
