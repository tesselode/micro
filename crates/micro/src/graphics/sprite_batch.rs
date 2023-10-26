mod sprite_params;

use palette::LinSrgba;
pub use sprite_params::SpriteParams;

use generational_arena::{Arena, Index};
use glam::Vec2;
use thiserror::Error;

use crate::{
	context::Context,
	graphics::{
		draw_params::DrawParams,
		mesh::{Mesh, Vertex},
		texture::Texture,
	},
	math::Rect,
	IntoOffsetAndCount, OffsetAndCount,
};

use super::color_constants::ColorConstants;

#[derive(Debug)]
pub struct SpriteBatch {
	texture: Texture,
	sprites: Arena<()>,
	mesh: Mesh,
}

impl SpriteBatch {
	pub fn new(ctx: &Context, texture: &Texture, capacity: usize) -> Self {
		let vertices = vec![
			Vertex {
				position: Vec2::ZERO,
				texture_coords: Vec2::ZERO,
				color: LinSrgba::WHITE,
			};
			capacity * 4
		];
		let mut indices: Vec<u32> = vec![];
		for i in 0..capacity {
			let start_index = i * 4;
			indices.extend_from_slice(&[
				start_index as u32,
				(start_index + 1) as u32,
				(start_index + 3) as u32,
				(start_index + 1) as u32,
				(start_index + 2) as u32,
				(start_index + 3) as u32,
			]);
		}
		Self {
			texture: texture.clone(),
			sprites: Arena::with_capacity(capacity),
			mesh: Mesh::new(ctx, &vertices, &indices),
		}
	}

	pub fn len(&self) -> usize {
		self.sprites.len()
	}

	#[must_use]
	pub fn is_empty(&self) -> bool {
		self.len() == 0
	}

	pub fn add(&mut self, params: impl Into<SpriteParams>) -> Result<SpriteId, SpriteLimitReached> {
		self.add_region(Rect::new(Vec2::ZERO, self.texture.size().as_vec2()), params)
	}

	pub fn add_region(
		&mut self,
		texture_region: Rect,
		params: impl Into<SpriteParams>,
	) -> Result<SpriteId, SpriteLimitReached> {
		let id = self
			.sprites
			.try_insert(())
			.map(SpriteId)
			.map_err(|_| SpriteLimitReached)?;
		let params: SpriteParams = params.into();
		let (sprite_index, _) = id.0.into_raw_parts();
		let start_vertex_index = sprite_index * 4;
		let untransformed_display_rect = Rect::new(Vec2::ZERO, texture_region.size);
		let relative_texture_region = self.texture.relative_rect(texture_region);
		let transform = params.transform;
		let corners = untransformed_display_rect.corners();
		let vertices = corners
			.iter()
			.copied()
			.zip(relative_texture_region.corners())
			.map(|(position, texture_coords)| Vertex {
				position: transform.transform_point2(position),
				texture_coords,
				color: params.color,
			})
			.enumerate();
		for (i, vertex) in vertices {
			self.mesh.set_vertex(start_vertex_index + i, vertex);
		}
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
					color: LinSrgba::WHITE,
				},
			);
		}
		Ok(())
	}

	pub fn draw<'a>(&self, ctx: &Context, params: impl Into<DrawParams<'a>>) {
		self.mesh.draw_textured(ctx, &self.texture, params);
	}

	pub fn draw_range<'a>(
		&self,
		ctx: &Context,
		range: impl IntoOffsetAndCount,
		params: impl Into<DrawParams<'a>>,
	) {
		let offset_and_count = range.into_offset_and_count(self.sprites.len());
		self.mesh.draw_range_textured(
			ctx,
			&self.texture,
			OffsetAndCount {
				offset: offset_and_count.offset * 6,
				count: offset_and_count.count * 6,
			},
			params,
		);
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SpriteId(pub Index);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Error)]
#[error("Cannot add more sprites to the sprite batch")]
pub struct SpriteLimitReached;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Error)]
#[error("No sprite with this ID exists")]
pub struct InvalidSpriteId;
