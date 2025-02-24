mod sprite_params;

use std::sync::{Arc, Mutex};

use derive_more::derive::{Display, Error};
use palette::LinSrgba;
pub use sprite_params::SpriteParams;

use generational_arena::{Arena, Index};
use glam::{Mat4, Vec2};

use crate::{
	Context, IntoOffsetAndCount, OffsetAndCount,
	color::ColorConstants,
	graphics::{mesh::Mesh, texture::Texture},
	math::Rect,
};

use super::{BlendMode, NineSlice, Vertex2d, shader::Shader, standard_draw_param_methods};

#[derive(Debug, Clone)]
pub struct SpriteBatch {
	inner: Arc<Mutex<SpriteBatchInner>>,
	texture: Texture,
	mesh: Mesh,
	pub range: Option<OffsetAndCount>,
	pub shader: Option<Shader>,
	pub transform: Mat4,
	pub color: LinSrgba,
	pub blend_mode: BlendMode,
}

impl SpriteBatch {
	pub fn new(ctx: &mut Context, texture: &Texture, capacity: usize) -> Self {
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
				capacity,
			})),
			texture: texture.clone(),
			mesh: Mesh::new(ctx, &vertices, &indices),
			shader: None,
			transform: Mat4::IDENTITY,
			color: LinSrgba::WHITE,
			blend_mode: BlendMode::default(),
			range: None,
		}
	}

	standard_draw_param_methods!();

	pub fn range(&self, range: impl IntoOffsetAndCount) -> Self {
		let mut new = self.clone();
		new.range = range.into_offset_and_count(self.inner.try_lock().unwrap().sprites.len());
		new
	}

	pub fn len(&self) -> usize {
		self.inner.try_lock().unwrap().sprites.len()
	}

	#[must_use]
	pub fn is_empty(&self) -> bool {
		self.len() == 0
	}

	pub fn add(
		&mut self,
		ctx: &Context,
		params: impl Into<SpriteParams>,
	) -> Result<SpriteId, SpriteLimitReached> {
		let _span = tracy_client::span!();
		let size = self.texture.size().as_vec2();
		self.add_region(ctx, Rect::new(Vec2::ZERO, size), params)
	}

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
			.unwrap()
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
			self.mesh.set_vertex(ctx, start_vertex_index + i, vertex);
		}
		Ok(id)
	}

	pub fn add_nine_slice(
		&mut self,
		ctx: &Context,
		nine_slice: NineSlice,
		display_rect: Rect,
		params: impl Into<SpriteParams>,
	) -> Result<[SpriteId; 9], SpriteLimitReached> {
		let _span = tracy_client::span!();
		if self.inner.try_lock().unwrap().sprites.len() + 9
			> self.inner.try_lock().unwrap().capacity
		{
			return Err(SpriteLimitReached);
		}
		let params: SpriteParams = params.into();
		Ok(nine_slice.slices(display_rect).map(|slice| {
			self.add_region(
				ctx,
				slice.texture_region,
				params
					.scaled(slice.display_rect.size / slice.texture_region.size)
					.translated(slice.display_rect.top_left),
			)
			.unwrap()
		}))
	}

	pub fn remove(&mut self, ctx: &Context, id: SpriteId) -> Result<(), InvalidSpriteId> {
		let _span = tracy_client::span!();
		if self
			.inner
			.try_lock()
			.unwrap()
			.sprites
			.remove(id.0)
			.is_none()
		{
			return Err(InvalidSpriteId);
		}
		let (sprite_index, _) = id.0.into_raw_parts();
		let start_vertex_index = sprite_index * 4;
		for i in 0..4 {
			self.mesh.set_vertex(
				ctx,
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

	pub fn draw(&self, ctx: &mut Context) {
		let _span = tracy_client::span!();
		self.mesh
			.texture(&self.texture)
			.range(self.range.map(|range| OffsetAndCount {
				offset: range.offset * 6,
				count: range.count * 6,
			}))
			.shader(&self.shader)
			.transformed(self.transform)
			.color(self.color)
			.blend_mode(self.blend_mode)
			.draw(ctx);
	}
}

#[derive(Debug)]
struct SpriteBatchInner {
	sprites: Arena<()>,
	capacity: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SpriteId(pub Index);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Error, Display)]
#[display("Cannot add more sprites to the sprite batch")]
pub struct SpriteLimitReached;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Error, Display)]
#[display("No sprite with this ID exists")]
pub struct InvalidSpriteId;
