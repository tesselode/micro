use lyon::{
	lyon_tessellation::{
		BuffersBuilder, FillOptions, FillTessellator, FillVertex, FillVertexConstructor,
		StrokeOptions, StrokeTessellator, StrokeVertex, StrokeVertexConstructor, VertexBuffers,
	},
	path::{traits::PathBuilder, Winding},
};
use thiserror::Error;
use vek::Vec2;

use crate::{error::GlError, math::Rect, Context};

use super::{Mesh, Vertex};

pub struct MeshBuilder {
	buffers: VertexBuffers<Vertex, u32>,
}

impl MeshBuilder {
	pub fn new() -> Self {
		Self {
			buffers: VertexBuffers::new(),
		}
	}

	pub fn add_rectangle(
		&mut self,
		style: ShapeStyle,
		rect: Rect,
	) -> Result<(), TessellationError> {
		match style {
			ShapeStyle::Fill => FillTessellator::new().tessellate_rectangle(
				&lyon::math::Rect {
					origin: lyon::math::point(rect.top_left.x, rect.top_left.y),
					size: lyon::math::size(rect.size().x, rect.size().y),
				},
				&FillOptions::default(),
				&mut BuffersBuilder::new(&mut self.buffers, FillVertexToVertex),
			)?,
			ShapeStyle::Stroke(width) => StrokeTessellator::new().tessellate_rectangle(
				&lyon::math::Rect {
					origin: lyon::math::point(rect.top_left.x, rect.top_left.y),
					size: lyon::math::size(rect.size().x, rect.size().y),
				},
				&StrokeOptions::default().with_line_width(width),
				&mut BuffersBuilder::new(&mut self.buffers, StrokeVertexToVertex),
			)?,
		};
		Ok(())
	}

	pub fn with_rectangle(
		mut self,
		style: ShapeStyle,
		rect: Rect,
	) -> Result<Self, TessellationError> {
		self.add_rectangle(style, rect)?;
		Ok(self)
	}

	pub fn add_circle(
		&mut self,
		style: ShapeStyle,
		center: Vec2<f32>,
		radius: f32,
	) -> Result<(), TessellationError> {
		match style {
			ShapeStyle::Fill => FillTessellator::new().tessellate_circle(
				lyon::math::point(center.x, center.y),
				radius,
				&FillOptions::default(),
				&mut BuffersBuilder::new(&mut self.buffers, FillVertexToVertex),
			)?,
			ShapeStyle::Stroke(width) => StrokeTessellator::new().tessellate_circle(
				lyon::math::point(center.x, center.y),
				radius,
				&StrokeOptions::default().with_line_width(width),
				&mut BuffersBuilder::new(&mut self.buffers, StrokeVertexToVertex),
			)?,
		};
		Ok(())
	}

	pub fn with_circle(
		mut self,
		style: ShapeStyle,
		center: Vec2<f32>,
		radius: f32,
	) -> Result<Self, TessellationError> {
		self.add_circle(style, center, radius)?;
		Ok(self)
	}

	pub fn add_ellipse(
		&mut self,
		style: ShapeStyle,
		center: Vec2<f32>,
		radii: Vec2<f32>,
		rotation: f32,
	) -> Result<(), TessellationError> {
		match style {
			ShapeStyle::Fill => FillTessellator::new().tessellate_ellipse(
				lyon::math::point(center.x, center.y),
				lyon::math::vector(radii.x, radii.y),
				lyon::math::Angle::radians(rotation),
				Winding::Positive,
				&FillOptions::default(),
				&mut BuffersBuilder::new(&mut self.buffers, FillVertexToVertex),
			)?,
			ShapeStyle::Stroke(width) => StrokeTessellator::new().tessellate_ellipse(
				lyon::math::point(center.x, center.y),
				lyon::math::vector(radii.x, radii.y),
				lyon::math::Angle::radians(rotation),
				Winding::Positive,
				&StrokeOptions::default().with_line_width(width),
				&mut BuffersBuilder::new(&mut self.buffers, StrokeVertexToVertex),
			)?,
		};
		Ok(())
	}

	pub fn with_ellipse(
		mut self,
		style: ShapeStyle,
		center: Vec2<f32>,
		radii: Vec2<f32>,
		rotation: f32,
	) -> Result<Self, TessellationError> {
		self.add_ellipse(style, center, radii, rotation)?;
		Ok(self)
	}

	pub fn add_polygon(
		&mut self,
		style: ShapeStyle,
		points: &[Vec2<f32>],
	) -> Result<(), TessellationError> {
		let polygon = lyon::path::Polygon {
			points: &points
				.iter()
				.map(|point| lyon::math::point(point.x, point.y))
				.collect::<Vec<_>>(),
			closed: true,
		};
		match style {
			ShapeStyle::Fill => FillTessellator::new().tessellate_polygon(
				polygon,
				&FillOptions::default(),
				&mut BuffersBuilder::new(&mut self.buffers, FillVertexToVertex),
			)?,
			ShapeStyle::Stroke(width) => StrokeTessellator::new().tessellate_polygon(
				polygon,
				&StrokeOptions::default().with_line_width(width),
				&mut BuffersBuilder::new(&mut self.buffers, StrokeVertexToVertex),
			)?,
		};
		Ok(())
	}

	pub fn with_polygon(
		mut self,
		style: ShapeStyle,
		points: &[Vec2<f32>],
	) -> Result<Self, TessellationError> {
		self.add_polygon(style, points)?;
		Ok(self)
	}

	pub fn add_polyline(
		&mut self,
		line_width: f32,
		points: &[Vec2<f32>],
	) -> Result<(), TessellationError> {
		if points.is_empty() {
			panic!("Need at least one point to build a polyline");
		}
		let mut stroke_tessellator = StrokeTessellator::new();
		let options = StrokeOptions::default().with_line_width(line_width);
		let mut buffers_builder = BuffersBuilder::new(&mut self.buffers, StrokeVertexToVertex);
		let mut builder = stroke_tessellator.builder(&options, &mut buffers_builder);
		builder.begin(lyon::math::point(points[0].x, points[0].y));
		for point in &points[1..] {
			builder.line_to(lyon::math::point(point.x, point.y));
		}
		builder.end(false);
		Ok(())
	}

	pub fn with_polyline(
		mut self,
		line_width: f32,
		points: &[Vec2<f32>],
	) -> Result<Self, TessellationError> {
		self.add_polyline(line_width, points)?;
		Ok(self)
	}

	pub fn build(self, ctx: &mut Context) -> Result<Mesh, GlError> {
		Mesh::new(ctx, &self.buffers.vertices, &self.buffers.indices)
	}
}

impl Default for MeshBuilder {
	fn default() -> Self {
		Self::new()
	}
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ShapeStyle {
	Fill,
	Stroke(f32),
}

struct FillVertexToVertex;

impl FillVertexConstructor<Vertex> for FillVertexToVertex {
	fn new_vertex(&mut self, vertex: FillVertex) -> Vertex {
		Vertex {
			position: Vec2::new(vertex.position().x, vertex.position().y),
			texture_coords: Vec2::zero(),
		}
	}
}

struct StrokeVertexToVertex;

impl StrokeVertexConstructor<Vertex> for StrokeVertexToVertex {
	fn new_vertex(&mut self, vertex: StrokeVertex) -> Vertex {
		Vertex {
			position: Vec2::new(vertex.position().x, vertex.position().y),
			texture_coords: Vec2::zero(),
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Error)]
pub enum TessellationError {
	#[error("Too many vertices")]
	TooManyVertices,
	#[error("An internal error occurred while building the mesh")]
	Internal,
}

impl From<lyon::tessellation::TessellationError> for TessellationError {
	fn from(error: lyon::tessellation::TessellationError) -> Self {
		match error {
			lyon::lyon_tessellation::TessellationError::UnsupportedParamater => {
				panic!("unsupported parameter")
			}
			lyon::lyon_tessellation::TessellationError::InvalidVertex => panic!("invalid vertex"),
			lyon::lyon_tessellation::TessellationError::TooManyVertices => Self::TooManyVertices,
			lyon::lyon_tessellation::TessellationError::Internal(_) => Self::Internal,
		}
	}
}
