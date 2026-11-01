use glam::{Affine2, Vec2};
use palette::LinSrgba;

use crate::{
	Micro,
	color::ColorConstants,
	graphics::{IntoScale2d, Vertex2d, mesh::Mesh, texture::Texture},
	math::Rect,
};

pub struct SpriteBatchBuilder {
	texture: Texture,
	vertices: Vec<Vertex2d>,
	indices: Vec<u32>,
	num_sprites: usize,
}

impl SpriteBatchBuilder {
	pub fn new(texture: &Texture) -> Self {
		Self {
			texture: texture.clone(),
			vertices: vec![],
			indices: vec![],
			num_sprites: 0,
		}
	}

	pub fn add(&mut self, params: impl Into<SpriteParams>) {
		let size = self.texture.size().as_vec2();
		self.add_region(Rect::new(Vec2::ZERO, size), params)
	}

	pub fn add_region(&mut self, region: Rect, params: impl Into<SpriteParams>) {
		let params: SpriteParams = params.into();
		let start_index = self.vertices.len();
		let untransformed_display_rect = Rect::new(Vec2::ZERO, region.size);
		let relative_texture_region = self.texture.relative_rect(region);
		let transform = params.transform;
		let corners = untransformed_display_rect.corners();
		self.vertices.extend(
			corners
				.iter()
				.copied()
				.zip(relative_texture_region.corners())
				.map(|(position, texture_coords)| Vertex2d {
					position: transform.transform_point2(position),
					texture_coords,
					color: params.color,
				}),
		);
		self.indices.extend_from_slice(&[
			start_index as u32,
			(start_index + 1) as u32,
			(start_index + 3) as u32,
			(start_index + 1) as u32,
			(start_index + 2) as u32,
			(start_index + 3) as u32,
		]);
		self.num_sprites += 1;
	}

	pub fn build(&self, micro: &Micro) -> Mesh {
		Mesh::triangle(micro, &self.vertices, &self.indices).texture(&self.texture)
	}
}

/// Settings for a sprite in a [`SpriteBatch`](super::SpriteBatch).
#[derive(Debug, Clone, Copy)]
pub struct SpriteParams {
	/// The 2D transform of this sprite.
	pub transform: Affine2,
	/// The blend color of this sprite.
	pub color: LinSrgba,
}

impl SpriteParams {
	/// Creates a new [`SpriteParams`] with the default settings.
	pub fn new() -> Self {
		Self {
			transform: Affine2::IDENTITY,
			color: LinSrgba::WHITE,
		}
	}

	/// Applies the specified `transform` on top of the existing transform.
	pub fn transformed(self, transform: Affine2) -> Self {
		Self {
			transform: transform * self.transform,
			..self
		}
	}

	/// Moves the sprite by the specified `translation` vector.
	pub fn translated(self, translation: impl Into<Vec2>) -> Self {
		Self {
			transform: Affine2::from_translation(translation.into()) * self.transform,
			..self
		}
	}

	/// Scales the sprite by the specified amount along the X and Y axes.
	pub fn scaled(self, scale: impl IntoScale2d) -> Self {
		Self {
			transform: Affine2::from_scale(scale.into_scale_2d()) * self.transform,
			..self
		}
	}

	/// Rotates the sprite by the specified amount (in radians).
	pub fn rotated(self, rotation: f32) -> Self {
		Self {
			transform: Affine2::from_angle(rotation) * self.transform,
			..self
		}
	}

	/// Sets the blend color of the sprite.
	pub fn color(self, color: impl Into<LinSrgba>) -> Self {
		Self {
			color: color.into(),
			..self
		}
	}
}

impl Default for SpriteParams {
	fn default() -> Self {
		Self::new()
	}
}

impl From<Affine2> for SpriteParams {
	fn from(transform: Affine2) -> Self {
		Self::new().transformed(transform)
	}
}

impl From<Vec2> for SpriteParams {
	fn from(translation: Vec2) -> Self {
		Self::new().translated(translation)
	}
}

impl From<LinSrgba> for SpriteParams {
	fn from(color: LinSrgba) -> Self {
		Self::new().color(color)
	}
}
