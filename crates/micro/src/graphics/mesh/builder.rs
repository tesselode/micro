mod vertex_constructors;

use std::{fmt::Debug, path::Path};

use glam::{Mat4, Vec2, Vec3};
use itertools::{izip, Itertools};
use lyon_tessellation::{
	geom::euclid::Point2D,
	path::{
		traits::{Build, PathBuilder},
		Winding,
	},
	BuffersBuilder, FillOptions, FillTessellator, StrokeOptions, StrokeTessellator,
	TessellationError, VertexBuffers,
};
use palette::{LinSrgb, LinSrgba, WithAlpha};

use crate::{
	graphics::ColorConstants,
	math::{Circle, Rect},
	Context,
};

use super::{Mesh, Vertex};

#[derive(Debug, Clone)]
pub struct MeshBuilder {
	pub(crate) buffers: VertexBuffers<Vertex, u32>,
}

impl MeshBuilder {
	pub fn new() -> Self {
		Self {
			buffers: VertexBuffers::new(),
		}
	}

	pub fn from_obj_file(path: impl AsRef<Path> + Debug) -> Result<Self, tobj::LoadError> {
		let (mut models, _) = tobj::load_obj(path, &tobj::GPU_LOAD_OPTIONS)?;
		let model = models.drain(..).next().expect("no models in obj file");
		Ok(Self::from_tobj_model(model))
	}

	pub fn rectangle(
		style: ShapeStyle,
		rect: Rect,
		color: LinSrgba,
	) -> Result<Self, TessellationError> {
		Self::new().with_rectangle(style, rect, color)
	}

	pub fn circle(
		style: ShapeStyle,
		circle: Circle,
		color: LinSrgba,
	) -> Result<Self, TessellationError> {
		Self::new().with_circle(style, circle, color)
	}

	pub fn ellipse(
		style: ShapeStyle,
		center: Vec2,
		radii: Vec2,
		rotation: f32,
		color: LinSrgba,
	) -> Result<Self, TessellationError> {
		Self::new().with_ellipse(style, center, radii, rotation, color)
	}

	pub fn filled_polygon(
		points: impl IntoIterator<Item = FilledPolygonPoint>,
	) -> Result<Self, TessellationError> {
		Self::new().with_filled_polygon(points)
	}

	pub fn polyline(
		points: impl IntoIterator<Item = StrokePoint>,
		closed: bool,
	) -> Result<Self, TessellationError> {
		Self::new().with_polyline(points, closed)
	}

	pub fn simple_polygon(
		style: ShapeStyle,
		points: impl IntoIterator<Item = Vec2>,
		color: LinSrgba,
	) -> Result<Self, TessellationError> {
		Self::new().with_simple_polygon(style, points, color)
	}

	pub fn simple_polyline(
		stroke_width: f32,
		points: impl IntoIterator<Item = Vec2>,
		color: LinSrgba,
	) -> Result<Self, TessellationError> {
		Self::new().with_simple_polyline(stroke_width, points, color)
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
				&mut BuffersBuilder::new(
					&mut self.buffers,
					vertex_constructors::PointWithoutColorToVertex { color },
				),
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
				&mut BuffersBuilder::new(
					&mut self.buffers,
					vertex_constructors::PointWithoutColorToVertex { color },
				),
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
		let mut buffers_builder = BuffersBuilder::new(
			&mut self.buffers,
			vertex_constructors::PointWithColorToVertex,
		);
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
		let mut buffers_builder = BuffersBuilder::new(
			&mut self.buffers,
			vertex_constructors::PointWithColorToVertex,
		);
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

	pub fn transform(&mut self, transform: Mat4) {
		for vertex in &mut self.buffers.vertices {
			vertex.position = transform.transform_point3(vertex.position);
		}
	}

	pub fn transformed(mut self, transform: Mat4) -> Self {
		self.transform(transform);
		self
	}

	pub fn append(&mut self, mut other: Self) {
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

	pub fn appended_with(mut self, other: Self) -> Self {
		self.append(other);
		self
	}

	pub fn build(self, ctx: &Context) -> Mesh {
		Mesh::new(ctx, &self.buffers.vertices, &self.buffers.indices)
	}

	fn from_tobj_model(model: tobj::Model) -> Self {
		let tobj_mesh = model.mesh;
		let num_vertices = tobj_mesh.positions.len() / 3;
		let positions = tobj_mesh
			.positions
			.chunks_exact(3)
			.map(|coords| Vec3::new(coords[0], coords[1], coords[2]));
		let texture_coords = tobj_mesh
			.texcoords
			.chunks_exact(2)
			.map(|coords| Vec2::new(coords[0], coords[1]))
			.pad_using(num_vertices, |_| Vec2::ZERO);
		let colors = tobj_mesh
			.vertex_color
			.chunks_exact(3)
			.map(|components| {
				LinSrgb::new(components[0], components[1], components[2]).with_alpha(1.0)
			})
			.pad_using(num_vertices, |_| LinSrgba::WHITE);
		let vertices = izip!(positions, texture_coords, colors)
			.map(|(position, texture_coords, color)| Vertex {
				position,
				texture_coords,
				color,
			})
			.collect::<Vec<_>>();
		Self {
			buffers: VertexBuffers {
				vertices,
				indices: tobj_mesh.indices,
			},
		}
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
