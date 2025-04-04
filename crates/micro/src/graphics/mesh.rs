mod builder;

pub use builder::*;

use std::marker::PhantomData;

use glam::{Mat4, Vec2};
use lyon_tessellation::TessellationError;
use palette::LinSrgba;
use wgpu::{
	Buffer, BufferUsages,
	util::{BufferInitDescriptor, DeviceExt},
};

use crate::{
	Context,
	color::ColorConstants,
	context::graphics::QueueDrawCommandSettings,
	graphics::texture::Texture,
	math::{Circle, Rect, URect},
	standard_draw_param_methods,
};

use super::{IntoRange, Vertex, Vertex2d, drawable::Drawable};

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
	pub scissor_rect: Option<URect>,
	pub range: Option<(u32, u32)>,
}

impl<V: Vertex> Mesh<V> {
	pub fn new(ctx: &Context, vertices: &[V], indices: &[u32]) -> Self {
		let vertex_buffer = ctx
			.graphics
			.device
			.create_buffer_init(&BufferInitDescriptor {
				label: Some("Mesh Vertex Buffer"),
				contents: bytemuck::cast_slice(vertices),
				usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
			});
		let index_buffer = ctx
			.graphics
			.device
			.create_buffer_init(&BufferInitDescriptor {
				label: Some("Mesh Index Buffer"),
				contents: bytemuck::cast_slice(indices),
				usage: BufferUsages::INDEX,
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
			scissor_rect: None,
			range: None,
		}
	}

	pub fn texture<'a>(&self, texture: impl Into<Option<&'a Texture>>) -> Self {
		Self {
			texture: texture.into().cloned(),
			..self.clone()
		}
	}

	standard_draw_param_methods!();

	pub fn range(&self, range: impl IntoRange) -> Self {
		let mut new = self.clone();
		new.range = range.into_range(self.num_indices);
		new
	}

	pub fn set_vertices(&self, ctx: &Context, index: usize, vertices: &[V]) {
		ctx.graphics.queue.write_buffer(
			&self.vertex_buffer,
			(index * std::mem::size_of::<V>()) as u64,
			bytemuck::cast_slice(vertices),
		);
	}
}

impl<V: Vertex> Drawable for Mesh<V> {
	type Vertex = V;

	#[allow(private_interfaces)]
	fn draw_instructions(&self, _ctx: &mut Context) -> Vec<QueueDrawCommandSettings> {
		let _span = tracy_client::span!();
		vec![QueueDrawCommandSettings {
			vertex_buffer: self.vertex_buffer.clone(),
			index_buffer: self.index_buffer.clone(),
			range: self.range.unwrap_or((0, self.num_indices)),
			local_transform: self.transform,
			color: self.color,
			scissor_rect: self.scissor_rect,
			texture: self.texture.clone(),
		}]
	}
}

impl Mesh<Vertex2d> {
	pub fn rectangle(ctx: &Context, rect: Rect) -> Self {
		let _span = tracy_client::span!();
		Self::rectangle_with_texture_region(ctx, rect, Rect::new((0.0, 0.0), (1.0, 1.0)))
	}

	pub fn rectangle_with_texture_region(
		ctx: &Context,
		display_rect: Rect,
		texture_region: Rect,
	) -> Self {
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
		Self::new(ctx, &vertices, &[0, 1, 3, 1, 2, 3])
	}

	pub fn outlined_rectangle(
		ctx: &Context,
		stroke_width: f32,
		rect: Rect,
	) -> Result<Self, TessellationError> {
		let _span = tracy_client::span!();
		Ok(MeshBuilder::new()
			.with_rectangle(ShapeStyle::Stroke(stroke_width), rect, LinSrgba::WHITE)?
			.build(ctx))
	}

	pub fn circle(
		ctx: &Context,
		style: ShapeStyle,
		circle: Circle,
	) -> Result<Self, TessellationError> {
		let _span = tracy_client::span!();
		Ok(MeshBuilder::new()
			.with_circle(style, circle, LinSrgba::WHITE)?
			.build(ctx))
	}

	pub fn ellipse(
		ctx: &Context,
		style: ShapeStyle,
		center: impl Into<Vec2>,
		radii: impl Into<Vec2>,
		rotation: f32,
	) -> Result<Self, TessellationError> {
		let _span = tracy_client::span!();
		Ok(MeshBuilder::new()
			.with_ellipse(style, center, radii, rotation, LinSrgba::WHITE)?
			.build(ctx))
	}

	pub fn filled_polygon(
		ctx: &Context,
		points: impl IntoIterator<Item = impl Into<FilledPolygonPoint>>,
	) -> Result<Self, TessellationError> {
		let _span = tracy_client::span!();
		Ok(MeshBuilder::new().with_filled_polygon(points)?.build(ctx))
	}

	pub fn polyline(
		ctx: &Context,
		points: impl IntoIterator<Item = impl Into<StrokePoint>>,
		closed: bool,
	) -> Result<Self, TessellationError> {
		let _span = tracy_client::span!();
		Ok(MeshBuilder::new().with_polyline(points, closed)?.build(ctx))
	}

	pub fn simple_polygon(
		ctx: &Context,
		style: ShapeStyle,
		points: impl IntoIterator<Item = impl Into<Vec2>>,
	) -> Result<Self, TessellationError> {
		let _span = tracy_client::span!();
		Ok(MeshBuilder::new()
			.with_simple_polygon(style, points, LinSrgba::WHITE)?
			.build(ctx))
	}

	pub fn simple_polyline(
		ctx: &Context,
		stroke_width: f32,
		points: impl IntoIterator<Item = impl Into<Vec2>>,
	) -> Result<Self, TessellationError> {
		let _span = tracy_client::span!();
		Ok(MeshBuilder::new()
			.with_simple_polyline(stroke_width, points, LinSrgba::WHITE)?
			.build(ctx))
	}

	pub fn draw(&self, ctx: &mut Context) {
		ctx.default_graphics_pipeline().draw(ctx, self);
	}
}
