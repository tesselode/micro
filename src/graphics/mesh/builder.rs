use glam::Vec2;
use lyon_tessellation::{
	path::Winding, BuffersBuilder, FillOptions, FillTessellator, FillVertex, FillVertexConstructor,
	StrokeOptions, StrokeTessellator, StrokeVertex, StrokeVertexConstructor, TessellationError,
	VertexBuffers,
};
use palette::LinSrgba;

use crate::{graphics::color_constants::ColorConstants, math::Rect, Context};

use super::{Mesh, Vertex};

pub struct MeshBuilder {
	buffers: VertexBuffers<Vertex, u32>,
	color: LinSrgba,
}

impl MeshBuilder {
	pub fn new() -> Self {
		Self {
			buffers: VertexBuffers::new(),
			color: LinSrgba::WHITE,
		}
	}

	pub fn set_color(&mut self, color: LinSrgba) {
		self.color = color;
	}

	pub fn with_color(mut self, color: LinSrgba, mut f: impl FnMut(Self) -> Self) -> Self {
		let previous_color = self.color;
		self.set_color(color);
		let mut this = f(self);
		this.set_color(previous_color);
		this
	}

	pub fn add_rectangle(
		&mut self,
		style: ShapeStyle,
		rect: Rect,
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
					FillVertexToVertex { color: self.color },
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
					StrokeVertexToVertex { color: self.color },
				),
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
		center: Vec2,
		radius: f32,
	) -> Result<(), TessellationError> {
		match style {
			ShapeStyle::Fill => FillTessellator::new().tessellate_circle(
				lyon_tessellation::math::point(center.x, center.y),
				radius,
				&FillOptions::default(),
				&mut BuffersBuilder::new(
					&mut self.buffers,
					FillVertexToVertex { color: self.color },
				),
			)?,
			ShapeStyle::Stroke(width) => StrokeTessellator::new().tessellate_circle(
				lyon_tessellation::math::point(center.x, center.y),
				radius,
				&StrokeOptions::default().with_line_width(width),
				&mut BuffersBuilder::new(
					&mut self.buffers,
					StrokeVertexToVertex { color: self.color },
				),
			)?,
		};
		Ok(())
	}

	pub fn with_circle(
		mut self,
		style: ShapeStyle,
		center: Vec2,
		radius: f32,
	) -> Result<Self, TessellationError> {
		self.add_circle(style, center, radius)?;
		Ok(self)
	}

	pub fn add_ellipse(
		&mut self,
		style: ShapeStyle,
		center: Vec2,
		radii: Vec2,
		rotation: f32,
	) -> Result<(), TessellationError> {
		match style {
			ShapeStyle::Fill => FillTessellator::new().tessellate_ellipse(
				lyon_tessellation::math::point(center.x, center.y),
				lyon_tessellation::math::vector(radii.x, radii.y),
				lyon_tessellation::math::Angle::radians(rotation),
				Winding::Positive,
				&FillOptions::default(),
				&mut BuffersBuilder::new(
					&mut self.buffers,
					FillVertexToVertex { color: self.color },
				),
			)?,
			ShapeStyle::Stroke(width) => StrokeTessellator::new().tessellate_ellipse(
				lyon_tessellation::math::point(center.x, center.y),
				lyon_tessellation::math::vector(radii.x, radii.y),
				lyon_tessellation::math::Angle::radians(rotation),
				Winding::Positive,
				&StrokeOptions::default().with_line_width(width),
				&mut BuffersBuilder::new(
					&mut self.buffers,
					StrokeVertexToVertex { color: self.color },
				),
			)?,
		};
		Ok(())
	}

	pub fn with_ellipse(
		mut self,
		style: ShapeStyle,
		center: Vec2,
		radii: Vec2,
		rotation: f32,
	) -> Result<Self, TessellationError> {
		self.add_ellipse(style, center, radii, rotation)?;
		Ok(self)
	}

	pub fn add_polygon(
		&mut self,
		style: ShapeStyle,
		points: &[Vec2],
	) -> Result<(), TessellationError> {
		let polygon = lyon_tessellation::path::Polygon {
			points: &points
				.iter()
				.map(|point| lyon_tessellation::math::point(point.x, point.y))
				.collect::<Vec<_>>(),
			closed: true,
		};
		match style {
			ShapeStyle::Fill => FillTessellator::new().tessellate_polygon(
				polygon,
				&FillOptions::default(),
				&mut BuffersBuilder::new(
					&mut self.buffers,
					FillVertexToVertex { color: self.color },
				),
			)?,
			ShapeStyle::Stroke(width) => StrokeTessellator::new().tessellate_polygon(
				polygon,
				&StrokeOptions::default().with_line_width(width),
				&mut BuffersBuilder::new(
					&mut self.buffers,
					StrokeVertexToVertex { color: self.color },
				),
			)?,
		};
		Ok(())
	}

	pub fn with_polygon(
		mut self,
		style: ShapeStyle,
		points: &[Vec2],
	) -> Result<Self, TessellationError> {
		self.add_polygon(style, points)?;
		Ok(self)
	}

	pub fn add_polyline(&mut self, line_width: f32, points: &[Vec2]) {
		if points.is_empty() {
			panic!("Need at least one point to build a polyline");
		}
		let mut stroke_tessellator = StrokeTessellator::new();
		let options = StrokeOptions::default().with_line_width(line_width);
		let mut buffers_builder = BuffersBuilder::new(
			&mut self.buffers,
			StrokeVertexToVertex { color: self.color },
		);
		let mut builder = stroke_tessellator.builder(&options, &mut buffers_builder);
		builder.begin(lyon_tessellation::math::point(points[0].x, points[0].y));
		for point in &points[1..] {
			builder.line_to(lyon_tessellation::math::point(point.x, point.y));
		}
		builder.end(false);
	}

	pub fn with_polyline(mut self, line_width: f32, points: &[Vec2]) -> Self {
		self.add_polyline(line_width, points);
		self
	}

	pub fn build(self, ctx: &Context) -> Mesh {
		Mesh::new(ctx, &self.buffers.vertices, &self.buffers.indices)
	}

	pub(crate) fn buffers(&self) -> &VertexBuffers<Vertex, u32> {
		&self.buffers
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

struct FillVertexToVertex {
	color: LinSrgba,
}

impl FillVertexConstructor<Vertex> for FillVertexToVertex {
	fn new_vertex(&mut self, vertex: FillVertex) -> Vertex {
		Vertex {
			position: Vec2::new(vertex.position().x, vertex.position().y),
			texture_coords: Vec2::ZERO,
			color: self.color,
		}
	}
}

struct StrokeVertexToVertex {
	color: LinSrgba,
}

impl StrokeVertexConstructor<Vertex> for StrokeVertexToVertex {
	fn new_vertex(&mut self, vertex: StrokeVertex) -> Vertex {
		Vertex {
			position: Vec2::new(vertex.position().x, vertex.position().y),
			texture_coords: Vec2::ZERO,
			color: self.color,
		}
	}
}
