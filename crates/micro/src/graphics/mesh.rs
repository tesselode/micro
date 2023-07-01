mod builder;

pub use builder::*;
use bytemuck::{Pod, Zeroable};
use glam::Vec2;
use lyon_tessellation::TessellationError;
use palette::LinSrgba;

use std::rc::Rc;

use wgpu::{
	util::{BufferInitDescriptor, DeviceExt},
	Buffer, BufferAddress, BufferUsages, Device, VertexAttribute, VertexBufferLayout,
	VertexStepMode,
};

use crate::{math::Rect, Context, IntoOffsetAndCount};

use super::{canvas::Canvas, shader::Shader, texture::Texture, DrawParams};

#[derive(Clone)]
pub struct Mesh(pub(crate) Rc<MeshInner>);

impl Mesh {
	pub fn new(ctx: &Context, vertices: &[Vertex], indices: &[u32]) -> Self {
		Self::new_internal(vertices, indices, &ctx.graphics_ctx.device)
	}

	pub fn rectangle(ctx: &Context, rect: Rect) -> Self {
		Self::rectangle_with_texture_region(ctx, rect, Rect::xywh(0.0, 0.0, 1.0, 1.0))
	}

	pub fn rectangle_with_texture_region(
		ctx: &Context,
		display_rect: Rect,
		texture_rect: Rect,
	) -> Self {
		let vertices = display_rect
			.corners()
			.iter()
			.copied()
			.zip(texture_rect.corners())
			.map(|(position, texture_coords)| Vertex {
				position,
				texture_coords,
				color: LinSrgba::new(1.0, 1.0, 1.0, 1.0),
			})
			.collect::<Vec<_>>();
		Self::new(ctx, &vertices, &[0, 1, 3, 1, 2, 3])
	}

	pub fn styled_rectangle(
		ctx: &Context,
		style: ShapeStyle,
		rect: Rect,
		color: LinSrgba,
	) -> Result<Self, TessellationError> {
		Ok(MeshBuilder::new()
			.with_rectangle(style, rect, color)?
			.build(ctx))
	}

	pub fn circle(
		ctx: &Context,
		style: ShapeStyle,
		center: Vec2,
		radius: f32,
		color: LinSrgba,
	) -> Result<Self, TessellationError> {
		Ok(MeshBuilder::new()
			.with_circle(style, center, radius, color)?
			.build(ctx))
	}

	pub fn ellipse(
		ctx: &Context,
		style: ShapeStyle,
		center: Vec2,
		radii: Vec2,
		rotation: f32,
		color: LinSrgba,
	) -> Result<Self, TessellationError> {
		Ok(MeshBuilder::new()
			.with_ellipse(style, center, radii, rotation, color)?
			.build(ctx))
	}

	pub fn filled_polygon(
		ctx: &Context,
		points: impl IntoIterator<Item = FilledPolygonPoint>,
	) -> Result<Self, TessellationError> {
		Ok(MeshBuilder::new().with_filled_polygon(points)?.build(ctx))
	}

	pub fn polyline(
		ctx: &Context,
		points: impl IntoIterator<Item = StrokePoint>,
		closed: bool,
	) -> Result<Self, TessellationError> {
		Ok(MeshBuilder::new().with_polyline(points, closed)?.build(ctx))
	}

	pub fn simple_polygon(
		ctx: &Context,
		style: ShapeStyle,
		points: impl IntoIterator<Item = Vec2>,
		color: LinSrgba,
	) -> Result<Self, TessellationError> {
		Ok(MeshBuilder::new()
			.with_simple_polygon(style, points, color)?
			.build(ctx))
	}

	pub fn simple_polyline(
		ctx: &Context,
		stroke_width: f32,
		points: impl IntoIterator<Item = Vec2>,
		color: LinSrgba,
	) -> Result<Self, TessellationError> {
		Ok(MeshBuilder::new()
			.with_simple_polyline(stroke_width, points, color)?
			.build(ctx))
	}

	pub fn set_vertex(&self, ctx: &Context, index: usize, vertex: Vertex) {
		ctx.graphics_ctx.queue.write_buffer(
			&self.0.vertex_buffer,
			(index * std::mem::size_of::<Vertex>()) as u64,
			bytemuck::cast_slice(&[vertex]),
		);
	}

	pub fn draw<S: Shader>(&self, ctx: &mut Context, params: impl Into<DrawParams<S>>) {
		self.draw_range(ctx, .., params);
	}

	pub fn draw_textured<S: Shader>(
		&self,
		ctx: &mut Context,
		texture: impl Into<MeshTexture>,
		params: impl Into<DrawParams<S>>,
	) {
		self.draw_range_textured(ctx, texture, .., params);
	}

	pub fn draw_range<S: Shader>(
		&self,
		ctx: &mut Context,
		range: impl IntoOffsetAndCount,
		params: impl Into<DrawParams<S>>,
	) {
		self.draw_range_textured(
			ctx,
			&ctx.graphics_ctx.default_texture.clone(),
			range,
			params,
		);
	}

	pub fn draw_range_textured<S: Shader>(
		&self,
		ctx: &mut Context,
		texture: impl Into<MeshTexture>,
		range: impl IntoOffsetAndCount,
		params: impl Into<DrawParams<S>>,
	) {
		ctx.graphics_ctx.push_instruction(
			self.clone(),
			texture.into(),
			range.into_offset_and_count(self.0.num_indices),
			params.into(),
		);
	}

	pub(crate) fn new_internal(vertices: &[Vertex], indices: &[u32], device: &Device) -> Self {
		let vertex_buffer = device.create_buffer_init(&BufferInitDescriptor {
			label: Some("Vertex Buffer"),
			contents: bytemuck::cast_slice(vertices),
			usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
		});
		let index_buffer = device.create_buffer_init(&BufferInitDescriptor {
			label: Some("Index Buffer"),
			contents: bytemuck::cast_slice(indices),
			usage: BufferUsages::INDEX,
		});
		Self(Rc::new(MeshInner {
			vertex_buffer,
			index_buffer,
			num_indices: indices.len() as u32,
		}))
	}
}

#[derive(Clone)]
pub enum MeshTexture {
	Texture(Texture),
	Canvas(Canvas),
}

impl From<Texture> for MeshTexture {
	fn from(v: Texture) -> Self {
		Self::Texture(v)
	}
}

impl From<Canvas> for MeshTexture {
	fn from(v: Canvas) -> Self {
		Self::Canvas(v)
	}
}

impl From<&Texture> for MeshTexture {
	fn from(v: &Texture) -> Self {
		Self::Texture(v.clone())
	}
}

impl From<&Canvas> for MeshTexture {
	fn from(v: &Canvas) -> Self {
		Self::Canvas(v.clone())
	}
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Vertex {
	pub position: Vec2,
	pub texture_coords: Vec2,
	pub color: LinSrgba,
}

impl Vertex {
	const ATTRIBUTES: [VertexAttribute; 3] =
		wgpu::vertex_attr_array![0 => Float32x2, 1 => Float32x2, 2 => Float32x4];

	pub(crate) fn buffer_layout<'a>() -> VertexBufferLayout<'a> {
		use std::mem;

		VertexBufferLayout {
			array_stride: mem::size_of::<Self>() as BufferAddress,
			step_mode: VertexStepMode::Vertex,
			attributes: &Self::ATTRIBUTES,
		}
	}
}

pub(crate) struct MeshInner {
	pub(crate) vertex_buffer: Buffer,
	pub(crate) index_buffer: Buffer,
	pub(crate) num_indices: u32,
}
