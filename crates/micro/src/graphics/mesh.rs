mod builder;

pub use builder::*;

use std::marker::PhantomData;

use glam::{Mat4, Vec2};
use palette::LinSrgba;
use wgpu::{
	Buffer, BufferUsages,
	util::{BufferInitDescriptor, DeviceExt},
};

use crate::{
	Context,
	color::ColorConstants,
	context::graphics::QueueDrawCommandSettings,
	graphics::{IntoRange, texture::Texture},
	math::{Circle, Rect},
	standard_draw_param_methods,
};

use super::{Vertex, Vertex2d};

#[derive(Debug, Clone, PartialEq)]
pub struct Mesh<V: Vertex = Vertex2d> {
	pub(crate) vertex_buffer: Buffer,
	pub(crate) index_buffer: Buffer,
	pub(crate) num_indices: u32,
	_phantom_data: PhantomData<V>,
	// draw params
	pub texture: Option<Texture>,
	pub transform: Mat4,
	pub color: LinSrgba,
	pub range: Option<(u32, u32)>,
}

impl<V: Vertex> Mesh<V> {
	pub fn new(vertices: &[V], indices: &[u32]) -> Self {
		let vertex_buffer = Context::with(|ctx| {
			ctx.graphics
				.device
				.create_buffer_init(&BufferInitDescriptor {
					label: Some("Mesh Vertex Buffer"),
					contents: bytemuck::cast_slice(vertices),
					usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
				})
		});
		let index_buffer = Context::with(|ctx| {
			ctx.graphics
				.device
				.create_buffer_init(&BufferInitDescriptor {
					label: Some("Mesh Index Buffer"),
					contents: bytemuck::cast_slice(indices),
					usage: BufferUsages::INDEX,
				})
		});
		let num_indices = indices.len() as u32;
		Self {
			vertex_buffer,
			index_buffer,
			num_indices,
			_phantom_data: PhantomData,
			texture: None,
			transform: Mat4::IDENTITY,
			color: LinSrgba::WHITE,
			range: None,
		}
	}

	standard_draw_param_methods!();

	pub fn texture<'a>(&self, texture: impl Into<Option<&'a Texture>>) -> Self {
		Self {
			texture: texture.into().cloned(),
			..self.clone()
		}
	}

	pub fn range(&self, range: impl IntoRange) -> Self {
		let mut new = self.clone();
		new.range = range.into_range(self.num_indices);
		new
	}

	pub fn set_vertices(&self, index: usize, vertices: &[V]) {
		Context::with(|ctx| {
			ctx.graphics.queue.write_buffer(
				&self.vertex_buffer,
				(index * std::mem::size_of::<V>()) as u64,
				bytemuck::cast_slice(vertices),
			)
		});
	}

	pub fn draw(&self) {
		Context::with_mut(|ctx| {
			ctx.graphics
				.queue_draw_command::<V>(QueueDrawCommandSettings {
					vertex_buffer: self.vertex_buffer.clone(),
					index_buffer: self.index_buffer.clone(),
					range: self.range.unwrap_or((0, self.num_indices)),
					texture: self
						.texture
						.as_ref()
						.unwrap_or(&ctx.graphics.default_texture)
						.clone(),
					transform: self.transform,
					color: self.color,
				})
		});
	}
}

impl Mesh<Vertex2d> {
	pub fn rectangle(rect: Rect) -> Self {
		let _span = tracy_client::span!();
		Self::rectangle_with_texture_region(rect, Rect::new((0.0, 0.0), (1.0, 1.0)))
	}

	pub fn rectangle_with_texture_region(display_rect: Rect, texture_region: Rect) -> Self {
		let _span = tracy_client::span!();
		let vertices = display_rect
			.corners()
			.iter()
			.copied()
			.zip(texture_region.corners())
			.map(|(position, texture_coords)| Vertex2d {
				position,
				texture_coords,
				color: LinSrgba::WHITE,
			})
			.collect::<Vec<_>>();
		Self::new(&vertices, &[0, 1, 3, 1, 2, 3])
	}

	pub fn outlined_rectangle(stroke_width: f32, rect: Rect) -> Self {
		let _span = tracy_client::span!();
		MeshBuilder::new()
			.with_rectangle(ShapeStyle::Stroke(stroke_width), rect, LinSrgba::WHITE)
			.build()
	}

	pub fn circle(style: ShapeStyle, circle: Circle) -> Self {
		let _span = tracy_client::span!();
		MeshBuilder::new()
			.with_circle(style, circle, LinSrgba::WHITE)
			.build()
	}

	pub fn ellipse(
		style: ShapeStyle,
		center: impl Into<Vec2>,
		radii: impl Into<Vec2>,
		rotation: f32,
	) -> Self {
		let _span = tracy_client::span!();
		MeshBuilder::new()
			.with_ellipse(style, center, radii, rotation, LinSrgba::WHITE)
			.build()
	}

	pub fn filled_polygon(points: impl IntoIterator<Item = impl Into<FilledPolygonPoint>>) -> Self {
		let _span = tracy_client::span!();
		MeshBuilder::new().with_filled_polygon(points).build()
	}

	pub fn polyline(
		points: impl IntoIterator<Item = impl Into<StrokePoint>>,
		closed: bool,
	) -> Self {
		let _span = tracy_client::span!();
		MeshBuilder::new().with_polyline(points, closed).build()
	}

	pub fn simple_polygon(
		style: ShapeStyle,
		points: impl IntoIterator<Item = impl Into<Vec2>>,
	) -> Self {
		let _span = tracy_client::span!();
		MeshBuilder::new()
			.with_simple_polygon(style, points, LinSrgba::WHITE)
			.build()
	}

	pub fn simple_polyline(
		stroke_width: f32,
		points: impl IntoIterator<Item = impl Into<Vec2>>,
	) -> Self {
		let _span = tracy_client::span!();
		MeshBuilder::new()
			.with_simple_polyline(stroke_width, points, LinSrgba::WHITE)
			.build()
	}
}
