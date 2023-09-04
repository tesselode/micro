use glam::Vec2;
use lyon_tessellation::{
	geom::euclid::Point2D,
	path::{
		traits::{Build, PathBuilder},
		Winding,
	},
	BuffersBuilder, FillOptions, FillTessellator, FillVertex, FillVertexConstructor, StrokeOptions,
	StrokeTessellator, StrokeVertex, StrokeVertexConstructor, TessellationError, VertexBuffers,
};
use palette::LinSrgba;

use crate::{
	math::{Circle, Rect},
	Context,
};

use super::{Mesh, Vertex};

pub struct MeshBuilder {
	pub(crate) buffers: VertexBuffers<Vertex, u32>,
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
		color: LinSrgba,
	) -> Result<(), TessellationError> {
		match style {
			ShapeStyle::Fill => FillTessellator::new().tessellate_rectangle(
				&lyon_tessellation::math::Box2D {
					min: lyon_tessellation::math::point(rect.top_left.x, rect.top_left.y),
					max: lyon_tessellation::math::point(
						rect.top_left.x + rect.size.x,
						rect.top_left.y + rect.size.y,
					),
				},
				&FillOptions::default(),
				&mut BuffersBuilder::new(&mut self.buffers, PointWithoutColorToVertex { color }),
			)?,
			ShapeStyle::Stroke(width) => StrokeTessellator::new().tessellate_rectangle(
				&lyon_tessellation::math::Box2D {
					min: lyon_tessellation::math::point(rect.top_left.x, rect.top_left.y),
					max: lyon_tessellation::math::point(
						rect.top_left.x + rect.size.x,
						rect.top_left.y + rect.size.y,
					),
				},
				&StrokeOptions::default().with_line_width(width),
				&mut BuffersBuilder::new(&mut self.buffers, PointWithoutColorToVertex { color }),
			)?,
		};
		Ok(())
	}

	pub fn with_rectangle(
		mut self,
		style: ShapeStyle,
		rect: Rect,
		color: LinSrgba,
	) -> Result<Self, TessellationError> {
		self.add_rectangle(style, rect, color)?;
		Ok(self)
	}

	pub fn add_circle(
		&mut self,
		style: ShapeStyle,
		circle: Circle,
		color: LinSrgba,
	) -> Result<(), TessellationError> {
		match style {
			ShapeStyle::Fill => FillTessellator::new()
				.tessellate_circle(
					lyon_tessellation::math::point(circle.center.x, circle.center.y),
					circle.radius,
					&FillOptions::default(),
					&mut BuffersBuilder::new(
						&mut self.buffers,
						PointWithoutColorToVertex { color },
					),
				)
				.unwrap(),
			ShapeStyle::Stroke(width) => StrokeTessellator::new()
				.tessellate_circle(
					lyon_tessellation::math::point(circle.center.x, circle.center.y),
					circle.radius,
					&StrokeOptions::default().with_line_width(width),
					&mut BuffersBuilder::new(
						&mut self.buffers,
						PointWithoutColorToVertex { color },
					),
				)
				.unwrap(),
		};
		Ok(())
	}

	pub fn with_circle(
		mut self,
		style: ShapeStyle,
		circle: Circle,
		color: LinSrgba,
	) -> Result<Self, TessellationError> {
		self.add_circle(style, circle, color)?;
		Ok(self)
	}

	pub fn add_ellipse(
		&mut self,
		style: ShapeStyle,
		center: Vec2,
		radii: Vec2,
		rotation: f32,
		color: LinSrgba,
	) -> Result<(), TessellationError> {
		match style {
			ShapeStyle::Fill => FillTessellator::new()
				.tessellate_ellipse(
					lyon_tessellation::math::point(center.x, center.y),
					lyon_tessellation::math::vector(radii.x, radii.y),
					lyon_tessellation::math::Angle::radians(rotation),
					Winding::Positive,
					&FillOptions::default(),
					&mut BuffersBuilder::new(
						&mut self.buffers,
						PointWithoutColorToVertex { color },
					),
				)
				.unwrap(),
			ShapeStyle::Stroke(width) => StrokeTessellator::new()
				.tessellate_ellipse(
					lyon_tessellation::math::point(center.x, center.y),
					lyon_tessellation::math::vector(radii.x, radii.y),
					lyon_tessellation::math::Angle::radians(rotation),
					Winding::Positive,
					&StrokeOptions::default().with_line_width(width),
					&mut BuffersBuilder::new(
						&mut self.buffers,
						PointWithoutColorToVertex { color },
					),
				)
				.unwrap(),
		};
		Ok(())
	}

	pub fn with_ellipse(
		mut self,
		style: ShapeStyle,
		center: Vec2,
		radii: Vec2,
		rotation: f32,
		color: LinSrgba,
	) -> Result<Self, TessellationError> {
		self.add_ellipse(style, center, radii, rotation, color)?;
		Ok(self)
	}

	pub fn add_filled_polygon(
		&mut self,
		points: impl IntoIterator<Item = FilledPolygonPoint>,
	) -> Result<(), TessellationError> {
		let mut fill_tessellator = FillTessellator::new();
		let mut buffers_builder = BuffersBuilder::new(&mut self.buffers, PointWithColorToVertex);
		let options = FillOptions::default();
		let mut builder =
			fill_tessellator.builder_with_attributes(4, &options, &mut buffers_builder);
		let mut points = points.into_iter();
		let point = points
			.next()
			.expect("need at least one point to build a polyline");
		builder.begin(
			Point2D::new(point.position.x, point.position.y),
			&[
				point.color.red,
				point.color.green,
				point.color.blue,
				point.color.alpha,
			],
		);
		for point in points {
			builder.line_to(
				Point2D::new(point.position.x, point.position.y),
				&[
					point.color.red,
					point.color.green,
					point.color.blue,
					point.color.alpha,
				],
			);
		}
		builder.end(true);
		builder.build()?;
		Ok(())
	}

	pub fn with_filled_polygon(
		mut self,
		points: impl IntoIterator<Item = FilledPolygonPoint>,
	) -> Result<Self, TessellationError> {
		self.add_filled_polygon(points)?;
		Ok(self)
	}

	pub fn add_polyline(
		&mut self,
		points: impl IntoIterator<Item = StrokePoint>,
		closed: bool,
	) -> Result<(), TessellationError> {
		let mut stroke_tessellator = StrokeTessellator::new();
		let mut buffers_builder = BuffersBuilder::new(&mut self.buffers, PointWithColorToVertex);
		let options = StrokeOptions::default().with_variable_line_width(4);
		let mut builder =
			stroke_tessellator.builder_with_attributes(5, &options, &mut buffers_builder);
		let mut points = points.into_iter();
		let point = points
			.next()
			.expect("need at least one point to build a polyline");
		builder.begin(
			Point2D::new(point.position.x, point.position.y),
			&[
				point.color.red,
				point.color.green,
				point.color.blue,
				point.color.alpha,
				point.stroke_width,
			],
		);
		for point in points {
			builder.line_to(
				Point2D::new(point.position.x, point.position.y),
				&[
					point.color.red,
					point.color.green,
					point.color.blue,
					point.color.alpha,
					point.stroke_width,
				],
			);
		}
		builder.end(closed);
		builder.build()?;
		Ok(())
	}

	pub fn with_polyline(
		mut self,
		points: impl IntoIterator<Item = StrokePoint>,
		closed: bool,
	) -> Result<Self, TessellationError> {
		self.add_polyline(points, closed)?;
		Ok(self)
	}

	pub fn add_simple_polygon(
		&mut self,
		style: ShapeStyle,
		points: impl IntoIterator<Item = Vec2>,
		color: LinSrgba,
	) -> Result<(), TessellationError> {
		match style {
			ShapeStyle::Fill => self.add_filled_polygon(
				points
					.into_iter()
					.map(|position| FilledPolygonPoint { position, color }),
			),
			ShapeStyle::Stroke(stroke_width) => self.add_polyline(
				points.into_iter().map(|position| StrokePoint {
					position,
					color,
					stroke_width,
				}),
				true,
			),
		}
	}

	pub fn with_simple_polygon(
		mut self,
		style: ShapeStyle,
		points: impl IntoIterator<Item = Vec2>,
		color: LinSrgba,
	) -> Result<Self, TessellationError> {
		self.add_simple_polygon(style, points, color)?;
		Ok(self)
	}

	pub fn add_simple_polyline(
		&mut self,
		stroke_width: f32,
		points: impl IntoIterator<Item = Vec2>,
		color: LinSrgba,
	) -> Result<(), TessellationError> {
		self.add_polyline(
			points.into_iter().map(|position| StrokePoint {
				position,
				color,
				stroke_width,
			}),
			false,
		)
	}

	pub fn with_simple_polyline(
		mut self,
		stroke_width: f32,
		points: impl IntoIterator<Item = Vec2>,
		color: LinSrgba,
	) -> Result<Self, TessellationError> {
		self.add_simple_polyline(stroke_width, points, color)?;
		Ok(self)
	}

	pub fn build(self, ctx: &Context) -> Mesh {
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

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FilledPolygonPoint {
	pub position: Vec2,
	pub color: LinSrgba,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StrokePoint {
	pub position: Vec2,
	pub color: LinSrgba,
	pub stroke_width: f32,
}

struct PointWithoutColorToVertex {
	color: LinSrgba,
}

impl FillVertexConstructor<Vertex> for PointWithoutColorToVertex {
	fn new_vertex(&mut self, vertex: FillVertex) -> Vertex {
		Vertex {
			position: Vec2::new(vertex.position().x, vertex.position().y),
			texture_coords: Vec2::ZERO,
			color: self.color,
		}
	}
}

impl StrokeVertexConstructor<Vertex> for PointWithoutColorToVertex {
	fn new_vertex(&mut self, vertex: StrokeVertex) -> Vertex {
		Vertex {
			position: Vec2::new(vertex.position().x, vertex.position().y),
			texture_coords: Vec2::ZERO,
			color: self.color,
		}
	}
}

struct PointWithColorToVertex;

impl FillVertexConstructor<Vertex> for PointWithColorToVertex {
	fn new_vertex(&mut self, mut vertex: FillVertex) -> Vertex {
		let position = Vec2::new(vertex.position().x, vertex.position().y);
		let attributes = vertex.interpolated_attributes();
		Vertex {
			position,
			texture_coords: Vec2::ZERO,
			color: LinSrgba::new(attributes[0], attributes[1], attributes[2], attributes[3]),
		}
	}
}

impl StrokeVertexConstructor<Vertex> for PointWithColorToVertex {
	fn new_vertex(&mut self, mut vertex: StrokeVertex) -> Vertex {
		let position = Vec2::new(vertex.position().x, vertex.position().y);
		let attributes = vertex.interpolated_attributes();
		Vertex {
			position,
			texture_coords: Vec2::ZERO,
			color: LinSrgba::new(attributes[0], attributes[1], attributes[2], attributes[3]),
		}
	}
}
