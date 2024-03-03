mod sprite_params;

use palette::LinSrgba;
pub use sprite_params::SpriteParams;

use generational_arena::{Arena, Index};
use glam::{Mat4, Vec2};
use thiserror::Error;

use crate::{
	graphics::{mesh::Mesh, texture::Texture},
	math::Rect,
	IntoOffsetAndCount, OffsetAndCount,
};

use super::{
	color_constants::ColorConstants, shader::Shader, standard_draw_command_methods, BlendMode,
	NineSlice, Vertex2d,
};

#[derive(Debug)]
pub struct SpriteBatch {
	texture: Texture,
	sprites: Arena<()>,
	mesh: Mesh,
	capacity: usize,
}

impl SpriteBatch {
	pub fn new(texture: &Texture, capacity: usize) -> Self {
		let vertices = vec![
			Vertex2d {
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
			mesh: Mesh::new(&vertices, &indices),
			capacity,
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
			.map(|(position, texture_coords)| Vertex2d {
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

	pub fn add_nine_slice(
		&mut self,

		nine_slice: NineSlice,
		display_rect: Rect,
		params: impl Into<SpriteParams>,
	) -> Result<[SpriteId; 9], SpriteLimitReached> {
		if self.sprites.len() + 9 > self.capacity {
			return Err(SpriteLimitReached);
		}
		let params: SpriteParams = params.into();
		Ok(nine_slice.slices(display_rect).map(|slice| {
			self.add_region(
				slice.texture_region,
				params
					.scaled(slice.display_rect.size / slice.texture_region.size)
					.translated(slice.display_rect.top_left),
			)
			.unwrap()
		}))
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
				Vertex2d {
					position: Vec2::ZERO,
					texture_coords: Vec2::ZERO,
					color: LinSrgba::WHITE,
				},
			);
		}
		Ok(())
	}

	pub fn draw(&self) -> DrawSpriteBatchCommand {
		DrawSpriteBatchCommand {
			sprite_batch: self,
			params: DrawSpriteBatchParams {
				range: (..).into_offset_and_count(self.sprites.len()),
				shader: None,
				transform: Mat4::IDENTITY,
				color: LinSrgba::WHITE,
				blend_mode: BlendMode::default(),
			},
		}
	}

	fn draw_inner(&self, params: &DrawSpriteBatchParams) {
		self.mesh
			.draw()
			.texture(&self.texture)
			.range(OffsetAndCount {
				offset: params.range.offset * 6,
				count: params.range.count * 6,
			})
			.shader(params.shader)
			.transformed(params.transform)
			.color(params.color)
			.blend_mode(params.blend_mode);
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

pub struct DrawSpriteBatchParams<'a> {
	pub range: OffsetAndCount,
	pub shader: Option<&'a Shader>,
	pub transform: Mat4,
	pub color: LinSrgba,
	pub blend_mode: BlendMode,
}

pub struct DrawSpriteBatchCommand<'a> {
	sprite_batch: &'a SpriteBatch,
	params: DrawSpriteBatchParams<'a>,
}

impl<'a> DrawSpriteBatchCommand<'a> {
	pub fn range(mut self, range: impl IntoOffsetAndCount) -> Self {
		self.params.range = range.into_offset_and_count(self.sprite_batch.sprites.len());
		self
	}

	standard_draw_command_methods!();
}

impl<'a> Drop for DrawSpriteBatchCommand<'a> {
	fn drop(&mut self) {
		self.sprite_batch.draw_inner(&self.params);
	}
}
