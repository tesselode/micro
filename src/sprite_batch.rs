use generational_arena::{Arena, Index};
use glam::Vec2;
use thiserror::Error;

use crate::{
	context::Context,
	draw_params::DrawParams,
	error::GlError,
	mesh::{Mesh, Vertex},
	rect::Rect,
	texture::Texture,
};

pub struct SpriteBatch {
	sprites: Arena<()>,
	mesh: Mesh,
}

impl SpriteBatch {
	pub fn new(ctx: &mut Context, capacity: usize) -> Result<Self, GlError> {
		let vertices = vec![
			Vertex {
				position: Vec2::ZERO,
				texture_coords: Vec2::ZERO,
			};
			capacity * 4
		];
		let mut indices: Vec<u32> = vec![];
		for i in 0..capacity {
			let start_index = i * 4;
			indices.extend_from_slice(&[
				start_index.try_into().expect("Too many vertices"),
				(start_index + 1).try_into().expect("Too many vertices"),
				(start_index + 3).try_into().expect("Too many vertices"),
				(start_index + 1).try_into().expect("Too many vertices"),
				(start_index + 2).try_into().expect("Too many vertices"),
				(start_index + 3).try_into().expect("Too many vertices"),
			]);
		}
		Ok(Self {
			sprites: Arena::with_capacity(capacity),
			mesh: Mesh::new(ctx, &vertices, &indices)?,
		})
	}

	pub fn add(&mut self, sprite: Sprite) -> Result<SpriteId, SpriteLimitReached> {
		let id = self
			.sprites
			.try_insert(())
			.map(SpriteId)
			.map_err(|_| SpriteLimitReached)?;
		let (sprite_index, _) = id.0.into_raw_parts();
		let start_vertex_index = sprite_index * 4;
		self.mesh.set_vertex(
			start_vertex_index,
			Vertex {
				position: sprite.display_rect.bottom_right(),
				texture_coords: sprite.texture_rect.bottom_right(),
			},
		);
		self.mesh.set_vertex(
			start_vertex_index + 1,
			Vertex {
				position: sprite.display_rect.top_right(),
				texture_coords: sprite.texture_rect.top_right(),
			},
		);
		self.mesh.set_vertex(
			start_vertex_index + 2,
			Vertex {
				position: sprite.display_rect.top_left,
				texture_coords: sprite.texture_rect.top_left,
			},
		);
		self.mesh.set_vertex(
			start_vertex_index + 3,
			Vertex {
				position: sprite.display_rect.bottom_left(),
				texture_coords: sprite.texture_rect.bottom_left(),
			},
		);
		Ok(id)
	}

	pub fn remove(&mut self, id: SpriteId) -> Result<(), InvalidSpriteId> {
		if self.sprites.remove(id.0).is_none() {
			return Err(InvalidSpriteId);
		}
		let (sprite_index, _) = id.0.into_raw_parts();
		let start_vertex_index = sprite_index * 4;
		for i in 0..4 {
			self.mesh.set_vertex(
				start_vertex_index + i,
				Vertex {
					position: Vec2::ZERO,
					texture_coords: Vec2::ZERO,
				},
			);
		}
		Ok(())
	}

	pub fn draw(&self, ctx: &mut Context, texture: &Texture, params: impl Into<DrawParams>) {
		self.mesh.draw_textured(ctx, texture, params);
	}
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Sprite {
	pub display_rect: Rect,
	pub texture_rect: Rect,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SpriteId(pub Index);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Error)]
#[error("Cannot add more sprites to the sprite batch")]
pub struct SpriteLimitReached;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Error)]
#[error("No sprite with this ID exists")]
pub struct InvalidSpriteId;
