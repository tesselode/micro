pub mod builder;

use std::marker::PhantomData;

use builder::{FilledPolygonPoint, MeshBuilder, ShapeStyle, StrokePoint};
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
	context::graphics::{DrawCommand, DrawParams},
	graphics::texture::Texture,
	math::{Circle, Rect},
	standard_draw_param_methods,
};

use super::{Vertex, Vertex2d, graphics_pipeline::GraphicsPipeline};

#[derive(Debug, Clone, PartialEq)]
pub struct Mesh<V: Vertex = Vertex2d> {
	vertex_buffer: Buffer,
	index_buffer: Buffer,
	num_indices: u32,
	_phantom_data: PhantomData<V>,

	// draw params
	pub texture: Option<Texture>,
	pub graphics_pipeline: Option<GraphicsPipeline<V>>,
	pub transform: Mat4,
	pub color: LinSrgba,
}

impl<V: Vertex> Mesh<V> {
	pub fn new(ctx: &Context, vertices: &[V], indices: &[u32]) -> Self {
		let vertex_buffer = ctx
			.graphics
			.device
			.create_buffer_init(&BufferInitDescriptor {
				label: None,
				contents: bytemuck::cast_slice(vertices),
				usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
			});
		let index_buffer = ctx
			.graphics
			.device
			.create_buffer_init(&BufferInitDescriptor {
				label: None,
				contents: bytemuck::cast_slice(indices),
				usage: BufferUsages::INDEX,
			});
		Self {
			vertex_buffer,
			index_buffer,
			num_indices: indices.len() as u32,
			_phantom_data: PhantomData,
			texture: None,
			graphics_pipeline: None,
			transform: Mat4::IDENTITY,
			color: LinSrgba::WHITE,
		}
	}

	pub fn texture<'a>(&self, texture: impl Into<Option<&'a Texture>>) -> Self {
		Self {
			texture: texture.into().cloned(),
			..self.clone()
		}
	}

	pub fn graphics_pipeline<'a>(
		&self,
		graphics_pipeline: impl Into<Option<&'a GraphicsPipeline<V>>>,
	) -> Self {
		Self {
			graphics_pipeline: graphics_pipeline.into().cloned(),
			..self.clone()
		}
	}

	standard_draw_param_methods!();

	pub fn set_vertices(&self, ctx: &Context, index: usize, vertices: &[V]) {
		ctx.graphics.queue.write_buffer(
			&self.vertex_buffer,
			(index * std::mem::size_of::<V>()) as u64,
			bytemuck::cast_slice(vertices),
		);
	}

	pub fn draw(&self, ctx: &mut Context) {
		ctx.graphics.queue_draw_command(DrawCommand {
			vertex_buffer: self.vertex_buffer.clone(),
			index_buffer: self.index_buffer.clone(),
			num_indices: self.num_indices,
			render_pipeline: self
				.graphics_pipeline
				.as_ref()
				.map(|graphics_pipeline| graphics_pipeline.render_pipeline.clone()),
			draw_params: DrawParams {
				transform: self.transform,
				color: self.color,
			},
			texture: self.texture.clone(),
		});
	}
}

impl Mesh<Vertex2d> {
	pub fn rectangle(ctx: &mut Context, rect: Rect) -> Self {
		let _span = tracy_client::span!();
		Self::rectangle_with_texture_region(ctx, rect, Rect::new((0.0, 0.0), (1.0, 1.0)))
	}

	pub fn rectangle_with_texture_region(
		ctx: &mut Context,
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
		ctx: &mut Context,
		stroke_width: f32,
		rect: Rect,
	) -> Result<Self, TessellationError> {
		let _span = tracy_client::span!();
		Ok(MeshBuilder::new()
			.with_rectangle(ShapeStyle::Stroke(stroke_width), rect, LinSrgba::WHITE)?
			.build(ctx))
	}

	pub fn circle(
		ctx: &mut Context,
		style: ShapeStyle,
		circle: Circle,
	) -> Result<Self, TessellationError> {
		let _span = tracy_client::span!();
		Ok(MeshBuilder::new()
			.with_circle(style, circle, LinSrgba::WHITE)?
			.build(ctx))
	}

	pub fn ellipse(
		ctx: &mut Context,
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
		ctx: &mut Context,
		points: impl IntoIterator<Item = impl Into<FilledPolygonPoint>>,
	) -> Result<Self, TessellationError> {
		let _span = tracy_client::span!();
		Ok(MeshBuilder::new().with_filled_polygon(points)?.build(ctx))
	}

	pub fn polyline(
		ctx: &mut Context,
		points: impl IntoIterator<Item = impl Into<StrokePoint>>,
		closed: bool,
	) -> Result<Self, TessellationError> {
		let _span = tracy_client::span!();
		Ok(MeshBuilder::new().with_polyline(points, closed)?.build(ctx))
	}

	pub fn simple_polygon(
		ctx: &mut Context,
		style: ShapeStyle,
		points: impl IntoIterator<Item = impl Into<Vec2>>,
	) -> Result<Self, TessellationError> {
		let _span = tracy_client::span!();
		Ok(MeshBuilder::new()
			.with_simple_polygon(style, points, LinSrgba::WHITE)?
			.build(ctx))
	}

	pub fn simple_polyline(
		ctx: &mut Context,
		stroke_width: f32,
		points: impl IntoIterator<Item = impl Into<Vec2>>,
	) -> Result<Self, TessellationError> {
		let _span = tracy_client::span!();
		Ok(MeshBuilder::new()
			.with_simple_polyline(stroke_width, points, LinSrgba::WHITE)?
			.build(ctx))
	}
}
