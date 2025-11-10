//! Types for grouping multiple drawing operations involving the same texture.

mod sprite_params;

pub use sprite_params::SpriteParams;

use std::sync::{Arc, Mutex};

use derive_more::derive::{Display, Error};
use generational_arena::{Arena, Index};
use glam::{Mat4, Vec2};
use palette::LinSrgba;

use crate::{
	Context,
	color::ColorConstants,
	graphics::{BlendMode, mesh::Mesh, texture::Texture},
	math::Rect,
	standard_draw_param_methods,
};

use super::{IntoIndexRange, Vertex2d};

/// Groups multiple drawing operations using the same texture
/// for better performance.
///
/// Can be modified as needed or reused as-is if the sprites don't change.
#[derive(Debug, Clone)]
pub struct SpriteBatch {
	inner: Arc<Mutex<SpriteBatchInner>>,
	texture: Texture,
	mesh: Mesh,
	/// The transform to use when drawing this sprite batch.
	pub transform: Mat4,
	/// The blend color to use when drawing this sprite batch.
	pub color: LinSrgba,
	/// The blend mode to use when drawing this sprite batch.
	pub blend_mode: BlendMode,
	/// The min and max sprite index to draw.
	///
	/// Setting this results in only some of the sprites being drawn.
	/// When `None`, all the sprites are drawn.
	pub range: Option<(u32, u32)>,
}

impl SpriteBatch {
	/// Creates a new [`SpriteBatch`] for the given `texture` that can hold
	/// a maximum of `capacity` sprites.
	pub fn new(ctx: &Context, texture: &Texture, capacity: usize) -> Self {
		let _span = tracy_client::span!();
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
			inner: Arc::new(Mutex::new(SpriteBatchInner {
				sprites: Arena::with_capacity(capacity),
			})),
			texture: texture.clone(),
			mesh: Mesh::new(ctx, &vertices, &indices),
			transform: Mat4::IDENTITY,
			color: LinSrgba::WHITE,
			blend_mode: BlendMode::default(),
			range: None,
		}
	}

	standard_draw_param_methods!();

	/// Sets the range of sprites to draw.
	///
	/// Setting this results in only some of the sprites being drawn.
	/// When `None`, all the sprites are drawn.
	pub fn range(&self, range: impl IntoIndexRange) -> Self {
		let mut new = self.clone();
		new.range = range.into_index_range(self.len() as u32);
		new
	}

	/// Returns the number of sprites in this [`SpriteBatch`].
	pub fn len(&self) -> usize {
		self.inner
			.try_lock()
			.expect("sprite batch mutex locked")
			.sprites
			.len()
	}

	/// Returns `true` if there are no sprites in this [`SpriteBatch`].
	#[must_use]
	pub fn is_empty(&self) -> bool {
		self.len() == 0
	}

	/// Adds a sprite to the [`SpriteBatch`] containing the entire texture.
	///
	/// Returns a [`SpriteId`] which can be used to remove the sprite later.
	pub fn add(
		&mut self,
		ctx: &Context,
		params: impl Into<SpriteParams>,
	) -> Result<SpriteId, SpriteLimitReached> {
		let _span = tracy_client::span!();
		let size = self.texture.size().as_vec2();
		self.add_region(ctx, Rect::new(Vec2::ZERO, size), params)
	}

	/// Adds a sprite to the [`SpriteBatch`] containing a portion of the texture.
	///
	/// Returns a [`SpriteId`] which can be used to remove the sprite later.
	pub fn add_region(
		&mut self,
		ctx: &Context,
		texture_region: Rect,
		params: impl Into<SpriteParams>,
	) -> Result<SpriteId, SpriteLimitReached> {
		let _span = tracy_client::span!();
		let id = self
			.inner
			.try_lock()
			.expect("sprite batch mutex locked")
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
			.collect::<Vec<_>>();
		self.mesh.set_vertices(ctx, start_vertex_index, &vertices);
		Ok(id)
	}

	/// Removes the sprite with the given `id` from the [`SpriteBatch`].
	pub fn remove(&mut self, ctx: &Context, id: SpriteId) -> Result<(), InvalidSpriteId> {
		let _span = tracy_client::span!();
		if self
			.inner
			.try_lock()
			.expect("sprite batch mutex locked")
			.sprites
			.remove(id.0)
			.is_none()
		{
			return Err(InvalidSpriteId);
		}
		let (sprite_index, _) = id.0.into_raw_parts();
		let start_vertex_index = sprite_index * 4;
		let vertices = [Vertex2d {
			position: Vec2::ZERO,
			texture_coords: Vec2::ZERO,
			color: LinSrgba::WHITE,
		}; 4];
		self.mesh.set_vertices(ctx, start_vertex_index, &vertices);
		Ok(())
	}

	/// Draws the [`SpriteBatch`].
	pub fn draw(&self, ctx: &mut Context) {
		self.mesh
			.texture(&self.texture)
			.transformed(self.transform)
			.color(self.color)
			.blend_mode(self.blend_mode)
			.range(self.range.map(|(start, end)| (start * 6, end * 6)))
			.draw(ctx)
	}
}

/// Uniquely identifies a sprite within a [`SpriteBatch`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SpriteId(pub Index);

/// An error that occurs when trying to add a sprite to a [`SpriteBatch`]
/// that's full.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Error, Display)]
#[display("Cannot add more sprites to the sprite batch")]
pub struct SpriteLimitReached;

/// An error that occurs when trying to remove a sprite that doesn't exist
/// in a [`SpriteBatch`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Error, Display)]
#[display("No sprite with this ID exists")]
pub struct InvalidSpriteId;

#[derive(Debug)]
struct SpriteBatchInner {
	sprites: Arena<()>,
}
