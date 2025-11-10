mod vertex_constructors;

use std::fmt::Debug;

use glam::Vec2;
use lyon_tessellation::{
	BuffersBuilder, FillOptions, FillTessellator, StrokeOptions, StrokeTessellator, VertexBuffers,
	geom::euclid::Point2D,
	path::{
		Winding,
		traits::{Build, PathBuilder},
	},
};
use palette::LinSrgba;

use crate::{
	Context,
	graphics::Vertex2d,
	math::{Circle, Rect},
};

use super::Mesh;

/// Creates a [`Mesh`] out of various shapes.
#[derive(Debug, Clone)]
pub struct MeshBuilder {
	pub(crate) buffers: VertexBuffers<Vertex2d, u32>,
}

impl MeshBuilder {
	/// Creates a new [`MeshBuilder`].
	pub fn new() -> Self {
		Self {
			buffers: VertexBuffers::new(),
		}
	}

	/// Creates a new [`MeshBuilder`] and adds a rectangle to the mesh.
	pub fn rectangle(style: ShapeStyle, rect: Rect, color: impl Into<LinSrgba>) -> Self {
		let _span = tracy_client::span!();
		Self::new().with_rectangle(style, rect, color)
	}

	/// Creates a new [`MeshBuilder`] and adds a circle to the mesh.
	pub fn circle(style: ShapeStyle, circle: Circle, color: impl Into<LinSrgba>) -> Self {
		let _span = tracy_client::span!();
		Self::new().with_circle(style, circle, color)
	}

	/// Creates a new [`MeshBuilder`] and adds an ellipse to the mesh.
	pub fn ellipse(
		style: ShapeStyle,
		center: impl Into<Vec2>,
		radii: impl Into<Vec2>,
		rotation: f32,
		color: impl Into<LinSrgba>,
	) -> Self {
		let _span = tracy_client::span!();
		Self::new().with_ellipse(style, center, radii, rotation, color)
	}

	/// Creates a new [`MeshBuilder`] and adds a filled polygon to the mesh.
	pub fn filled_polygon(points: impl IntoIterator<Item = impl Into<FilledPolygonPoint>>) -> Self {
		let _span = tracy_client::span!();
		Self::new().with_filled_polygon(points)
	}

	/// Creates a new [`MeshBuilder`] and adds a polyline to the mesh.
	pub fn polyline(
		points: impl IntoIterator<Item = impl Into<StrokePoint>>,
		closed: bool,
	) -> Self {
		let _span = tracy_client::span!();
		Self::new().with_polyline(points, closed)
	}

	/// Creates a new [`MeshBuilder`] and adds a polygon to the mesh where
	/// all of the vertices have the same color and stroke width (if applicable).
	pub fn simple_polygon(
		style: ShapeStyle,
		points: impl IntoIterator<Item = impl Into<Vec2>>,
		color: impl Into<LinSrgba>,
	) -> Self {
		let _span = tracy_client::span!();
		Self::new().with_simple_polygon(style, points, color)
	}

	/// Creates a new [`MeshBuilder`] and adds a polyline to the mesh where all of the
	/// vertices have the same stroke width and color.
	pub fn simple_polyline(
		stroke_width: f32,
		points: impl IntoIterator<Item = impl Into<Vec2>>,
		color: impl Into<LinSrgba>,
	) -> Self {
		let _span = tracy_client::span!();
		Self::new().with_simple_polyline(stroke_width, points, color)
	}

	/// Adds a rectangle to the mesh.
	pub fn add_rectangle(&mut self, style: ShapeStyle, rect: Rect, color: impl Into<LinSrgba>) {
		self.add_rectangle_inner(style, rect, color.into())
	}

	/// Adds a rectangle to the mesh and returns the [`MeshBuilder`].
	pub fn with_rectangle(
		mut self,
		style: ShapeStyle,
		rect: Rect,
		color: impl Into<LinSrgba>,
	) -> Self {
		let _span = tracy_client::span!();
		self.add_rectangle(style, rect, color);
		self
	}

	/// Adds a circle to the mesh.
	pub fn add_circle(&mut self, style: ShapeStyle, circle: Circle, color: impl Into<LinSrgba>) {
		self.add_circle_inner(style, circle, color.into())
	}

	/// Adds a circle to the mesh and returns the [`MeshBuilder`].
	pub fn with_circle(
		mut self,
		style: ShapeStyle,
		circle: Circle,
		color: impl Into<LinSrgba>,
	) -> Self {
		let _span = tracy_client::span!();
		self.add_circle(style, circle, color);
		self
	}

	/// Adds an ellipse to the mesh.
	pub fn add_ellipse(
		&mut self,
		style: ShapeStyle,
		center: impl Into<Vec2>,
		radii: impl Into<Vec2>,
		rotation: f32,
		color: impl Into<LinSrgba>,
	) {
		self.add_ellipse_inner(style, center, radii, rotation, color.into())
	}

	/// Adds an ellipse to the mesh and returns the [`MeshBuilder`].
	pub fn with_ellipse(
		mut self,
		style: ShapeStyle,
		center: impl Into<Vec2>,
		radii: impl Into<Vec2>,
		rotation: f32,
		color: impl Into<LinSrgba>,
	) -> Self {
		let _span = tracy_client::span!();
		self.add_ellipse(style, center, radii, rotation, color);
		self
	}

	/// Adds a filled polygon to the mesh.
	pub fn add_filled_polygon(
		&mut self,
		points: impl IntoIterator<Item = impl Into<FilledPolygonPoint>>,
	) {
		let _span = tracy_client::span!();
		let mut fill_tessellator = FillTessellator::new();
		let mut buffers_builder = BuffersBuilder::new(
			&mut self.buffers,
			vertex_constructors::PointWithColorToVertex,
		);
		let options = FillOptions::default();
		let mut builder =
			fill_tessellator.builder_with_attributes(4, &options, &mut buffers_builder);
		let mut points = points.into_iter();
		let point: FilledPolygonPoint = points
			.next()
			.expect("cannot build a polygon with no points")
			.into();
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
			let point: FilledPolygonPoint = point.into();
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
		builder.build().unwrap();
	}

	/// Adds a filled polygon to the mesh and returns the [`MeshBuilder`].
	pub fn with_filled_polygon(
		mut self,
		points: impl IntoIterator<Item = impl Into<FilledPolygonPoint>>,
	) -> Self {
		let _span = tracy_client::span!();
		self.add_filled_polygon(points);
		self
	}

	/// Adds a polyline to the mesh.
	pub fn add_polyline(
		&mut self,
		points: impl IntoIterator<Item = impl Into<StrokePoint>>,
		closed: bool,
	) {
		let _span = tracy_client::span!();
		let mut stroke_tessellator = StrokeTessellator::new();
		let mut buffers_builder = BuffersBuilder::new(
			&mut self.buffers,
			vertex_constructors::PointWithColorToVertex,
		);
		let options = StrokeOptions::default().with_variable_line_width(4);
		let mut builder =
			stroke_tessellator.builder_with_attributes(5, &options, &mut buffers_builder);
		let mut points = points.into_iter();
		let point: StrokePoint = points
			.next()
			.expect("cannot build a polyline with no points")
			.into();
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
			let point: StrokePoint = point.into();
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
		builder.build().unwrap();
	}

	/// Adds a polyline to the mesh and returns the [`MeshBuilder`].
	pub fn with_polyline(
		mut self,
		points: impl IntoIterator<Item = impl Into<StrokePoint>>,
		closed: bool,
	) -> Self {
		let _span = tracy_client::span!();
		self.add_polyline(points, closed);
		self
	}

	/// Adds a polygon to the mesh where all of the vertices have
	/// the same color and stroke width (if applicable).
	pub fn add_simple_polygon(
		&mut self,
		style: ShapeStyle,
		points: impl IntoIterator<Item = impl Into<Vec2>>,
		color: impl Into<LinSrgba>,
	) {
		self.add_simple_polygon_inner(style, points, color.into())
	}

	/// Adds a polygon to the mesh where all of the vertices have
	/// the same color and stroke width (if applicable) and
	/// returns the [`MeshBuilder`].
	pub fn with_simple_polygon(
		mut self,
		style: ShapeStyle,
		points: impl IntoIterator<Item = impl Into<Vec2>>,
		color: impl Into<LinSrgba>,
	) -> Self {
		let _span = tracy_client::span!();
		self.add_simple_polygon(style, points, color);
		self
	}

	/// Adds a polyline to the mesh where all of the vertices have
	/// the same stroke width and color.
	pub fn add_simple_polyline(
		&mut self,
		stroke_width: f32,
		points: impl IntoIterator<Item = impl Into<Vec2>>,
		color: impl Into<LinSrgba>,
	) {
		self.add_simple_polyline_inner(stroke_width, points, color.into())
	}

	/// Adds a polyline to the mesh where all of the vertices have
	/// the same stroke width and color and returns the [`MeshBuilder`].
	pub fn with_simple_polyline(
		mut self,
		stroke_width: f32,
		points: impl IntoIterator<Item = impl Into<Vec2>>,
		color: impl Into<LinSrgba>,
	) -> Self {
		let _span = tracy_client::span!();
		self.add_simple_polyline(stroke_width, points, color);
		self
	}

	/// Appends the vertices from another [`MeshBuilder`] onto this one.
	pub fn append(&mut self, mut other: Self) {
		let _span = tracy_client::span!();
		let num_vertices_before_append = self.buffers.vertices.len();
		self.buffers.vertices.append(&mut other.buffers.vertices);
		self.buffers.indices.extend(
			other
				.buffers
				.indices
				.drain(..)
				.map(|index| index + num_vertices_before_append as u32),
		);
	}

	/// Appends the vertices from another [`MeshBuilder`] onto this one
	/// and returns this [`MeshBuilder`].
	pub fn appended_with(mut self, other: Self) -> Self {
		let _span = tracy_client::span!();
		self.append(other);
		self
	}

	/// Consumes the [`MeshBuilder`] and returns a [`Mesh`].
	pub fn build(self, ctx: &Context) -> Mesh {
		let _span = tracy_client::span!();
		Mesh::new(ctx, &self.buffers.vertices, &self.buffers.indices)
	}

	fn add_rectangle_inner(&mut self, style: ShapeStyle, rect: Rect, color: LinSrgba) {
		let _span = tracy_client::span!();
		match style {
			ShapeStyle::Fill => FillTessellator::new()
				.tessellate_rectangle(
					&lyon_tessellation::math::Box2D {
						min: lyon_tessellation::math::point(rect.top_left.x, rect.top_left.y),
						max: lyon_tessellation::math::point(
							rect.top_left.x + rect.size.x,
							rect.top_left.y + rect.size.y,
						),
					},
					&FillOptions::default(),
					&mut BuffersBuilder::new(
						&mut self.buffers,
						vertex_constructors::PointWithoutColorToVertex { color },
					),
				)
				.unwrap(),
			ShapeStyle::Stroke(width) => StrokeTessellator::new()
				.tessellate_rectangle(
					&lyon_tessellation::math::Box2D {
						min: lyon_tessellation::math::point(rect.top_left.x, rect.top_left.y),
						max: lyon_tessellation::math::point(
							rect.top_left.x + rect.size.x,
							rect.top_left.y + rect.size.y,
						),
					},
					&StrokeOptions::default().with_line_width(width),
					&mut BuffersBuilder::new(
						&mut self.buffers,
						vertex_constructors::PointWithoutColorToVertex { color },
					),
				)
				.unwrap(),
		};
	}

	fn add_circle_inner(&mut self, style: ShapeStyle, circle: Circle, color: LinSrgba) {
		let _span = tracy_client::span!();
		match style {
			ShapeStyle::Fill => FillTessellator::new()
				.tessellate_circle(
					lyon_tessellation::math::point(circle.center.x, circle.center.y),
					circle.radius,
					&FillOptions::default(),
					&mut BuffersBuilder::new(
						&mut self.buffers,
						vertex_constructors::PointWithoutColorToVertex { color },
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
						vertex_constructors::PointWithoutColorToVertex { color },
					),
				)
				.unwrap(),
		};
	}

	fn add_ellipse_inner(
		&mut self,
		style: ShapeStyle,
		center: impl Into<Vec2>,
		radii: impl Into<Vec2>,
		rotation: f32,
		color: LinSrgba,
	) {
		let _span = tracy_client::span!();
		let center = center.into();
		let radii = radii.into();
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
						vertex_constructors::PointWithoutColorToVertex { color },
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
						vertex_constructors::PointWithoutColorToVertex { color },
					),
				)
				.unwrap(),
		};
	}

	fn add_simple_polygon_inner(
		&mut self,
		style: ShapeStyle,
		points: impl IntoIterator<Item = impl Into<Vec2>>,
		color: LinSrgba,
	) {
		let _span = tracy_client::span!();
		match style {
			ShapeStyle::Fill => {
				self.add_filled_polygon(points.into_iter().map(|position| FilledPolygonPoint {
					position: position.into(),
					color,
				}))
			}
			ShapeStyle::Stroke(stroke_width) => self.add_polyline(
				points.into_iter().map(|position| StrokePoint {
					position: position.into(),
					color,
					stroke_width,
				}),
				true,
			),
		}
	}

	fn add_simple_polyline_inner(
		&mut self,
		stroke_width: f32,
		points: impl IntoIterator<Item = impl Into<Vec2>>,
		color: LinSrgba,
	) {
		let _span = tracy_client::span!();
		self.add_polyline(
			points.into_iter().map(|position| StrokePoint {
				position: position.into(),
				color,
				stroke_width,
			}),
			false,
		)
	}
}

impl Default for MeshBuilder {
	fn default() -> Self {
		Self::new()
	}
}

/// Whether a shape is filled or outlined.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ShapeStyle {
	/// The shape is filled.
	Fill,
	/// The shape is outlined with the specified stroke width.
	Stroke(f32),
}

/// A vertex of a filled polygon.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FilledPolygonPoint {
	/// The 2D coordinates of the vertex.
	pub position: Vec2,
	/// The color of the vertex.
	pub color: LinSrgba,
}

/// A vertex of a polyline.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StrokePoint {
	/// The 2D coordinates of the vertex.
	pub position: Vec2,
	/// The color of the vertex.
	pub color: LinSrgba,
	/// The stroke width at this vertex.
	pub stroke_width: f32,
}
