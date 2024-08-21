use glam::Vec2;
use palette::LinSrgba;

use crate::{color::ColorConstants, graphics::texture::Texture, Context};

use super::Widget;

#[derive(Debug)]
pub struct Image {
	texture: Texture,
	color: LinSrgba,
}

impl Image {
	pub fn new(texture: &Texture) -> Self {
		Self {
			texture: texture.clone(),
			color: LinSrgba::WHITE,
		}
	}

	pub fn with_color(self, color: impl Into<LinSrgba>) -> Self {
		Self {
			color: color.into(),
			..self
		}
	}
}

impl Widget for Image {
	fn size(&mut self, _ctx: &mut Context, _max_size: Vec2) -> Vec2 {
		self.texture.size().as_vec2()
	}

	fn draw(&self, ctx: &mut Context) -> anyhow::Result<()> {
		self.texture.color(self.color).draw(ctx);
		Ok(())
	}
}
