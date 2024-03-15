mod sprite_params;

use std::{cell::RefCell, rc::Rc};

use palette::LinSrgba;
pub use sprite_params::SpriteParams;

use generational_arena::{Arena, Index};
use glam::{Mat4, Vec2, Vec3};
use thiserror::Error;

use crate::{
	graphics::{mesh::Mesh, texture::Texture},
	math::Rect,
	IntoOffsetAndCount, OffsetAndCount,
};

use super::{color_constants::ColorConstants, shader::Shader, BlendMode, NineSlice, Vertex2d};

#[derive(Debug, Clone)]
pub struct SpriteBatch {
	inner: Rc<RefCell<SpriteBatchInner>>,
	pub range: OffsetAndCount,
	pub shader: Option<Shader>,
	pub transform: Mat4,
	pub color: LinSrgba,
	pub blend_mode: BlendMode,
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
			inner: Rc::new(RefCell::new(SpriteBatchInner {
				texture: texture.clone(),
				sprites: Arena::with_capacity(capacity),
				mesh: Mesh::new(&vertices, &indices),
				capacity,
			})),
			shader: None,
			transform: Mat4::IDENTITY,
			color: LinSrgba::WHITE,
			blend_mode: BlendMode::default(),
			range: (..).into_offset_and_count(0),
		}
	}

	pub fn shader<'a>(&self, shader: impl Into<Option<&'a Shader>>) -> Self {
		let mut new = self.clone();
		new.shader = shader.into().cloned();
		new
	}

	pub fn transformed(&self, transform: impl Into<Mat4>) -> Self {
		let mut new = self.clone();
		new.transform = transform.into() * self.transform;
		new
	}

	pub fn translated_2d(&self, translation: impl Into<Vec2>) -> Self {
		self.transformed(Mat4::from_translation(translation.into().extend(0.0)))
	}

	pub fn translated_3d(&self, translation: impl Into<Vec3>) -> Self {
		self.transformed(Mat4::from_translation(translation.into()))
	}

	pub fn scaled_2d(&self, scale: impl Into<Vec2>) -> Self {
		self.transformed(Mat4::from_scale(scale.into().extend(1.0)))
	}

	pub fn scaled_3d(&self, scale: impl Into<Vec3>) -> Self {
		self.transformed(Mat4::from_scale(scale.into()))
	}

	pub fn rotated_x(&self, rotation: f32) -> Self {
		self.transformed(Mat4::from_rotation_x(rotation))
	}

	pub fn rotated_y(&self, rotation: f32) -> Self {
		self.transformed(Mat4::from_rotation_y(rotation))
	}

	pub fn rotated_z(&self, rotation: f32) -> Self {
		self.transformed(Mat4::from_rotation_z(rotation))
	}

	pub fn color(&self, color: impl Into<LinSrgba>) -> Self {
		let mut new = self.clone();
		new.color = color.into();
		new
	}

	pub fn blend_mode(&self, blend_mode: BlendMode) -> Self {
		let mut new = self.clone();
		new.blend_mode = blend_mode;
		new
	}

	pub fn range(&self, range: impl IntoOffsetAndCount) -> Self {
		let mut new = self.clone();
		new.range = range.into_offset_and_count(self.inner.borrow().sprites.len());
		new
	}

	pub fn len(&self) -> usize {
		self.inner.borrow().sprites.len()
	}

	#[must_use]
	pub fn is_empty(&self) -> bool {
		self.len() == 0
	}

	pub fn add(&mut self, params: impl Into<SpriteParams>) -> Result<SpriteId, SpriteLimitReached> {
		let size = self.inner.borrow_mut().texture.size().as_vec2();
		self.add_region(Rect::new(Vec2::ZERO, size), params)
	}

	pub fn add_region(
		&mut self,
		texture_region: Rect,
		params: impl Into<SpriteParams>,
	) -> Result<SpriteId, SpriteLimitReached> {
		let id = self
			.inner
			.borrow_mut()
			.sprites
			.try_insert(())
			.map(SpriteId)
			.map_err(|_| SpriteLimitReached)?;
		let params: SpriteParams = params.into();
		let (sprite_index, _) = id.0.into_raw_parts();
		let start_vertex_index = sprite_index * 4;
		let untransformed_display_rect = Rect::new(Vec2::ZERO, texture_region.size);
		let relative_texture_region = self.inner.borrow().texture.relative_rect(texture_region);
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
			self.inner
				.borrow()
				.mesh
				.set_vertex(start_vertex_index + i, vertex);
		}
		Ok(id)
	}

	pub fn add_nine_slice(
		&mut self,

		nine_slice: NineSlice,
		display_rect: Rect,
		params: impl Into<SpriteParams>,
	) -> Result<[SpriteId; 9], SpriteLimitReached> {
		if self.inner.borrow().sprites.len() + 9 > self.inner.borrow().capacity {
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
		if self.inner.borrow_mut().sprites.remove(id.0).is_none() {
			return Err(InvalidSpriteId);
		}
		let (sprite_index, _) = id.0.into_raw_parts();
		let start_vertex_index = sprite_index * 4;
		for i in 0..4 {
			self.inner.borrow().mesh.set_vertex(
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

	pub fn draw(&self) {
		self.inner
			.borrow()
			.mesh
			.texture(&self.inner.borrow().texture)
			.range(OffsetAndCount {
				offset: self.range.offset * 6,
				count: self.range.count * 6,
			})
			.shader(&self.shader)
			.transformed(self.transform)
			.color(self.color)
			.blend_mode(self.blend_mode)
			.draw();
	}
}

#[derive(Debug)]
struct SpriteBatchInner {
	texture: Texture,
	sprites: Arena<()>,
	mesh: Mesh,
	capacity: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SpriteId(pub Index);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Error)]
#[error("Cannot add more sprites to the sprite batch")]
pub struct SpriteLimitReached;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Error)]
#[error("No sprite with this ID exists")]
pub struct InvalidSpriteId;
