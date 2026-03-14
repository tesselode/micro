use micro::{
	Context,
	color::LinSrgba,
	graphics::mesh::{Mesh, ShapeStyle},
	math::Vec2,
};

use crate::{WidgetState, common_functions, common_widget_trait_functions};

use super::{LayoutResult, Widget};

#[derive(Debug)]
pub struct Polygon {
	id: Option<String>,
	points: Vec<Vec2>,
	size: Vec2,
	fill: Option<LinSrgba>,
	stroke: Option<(f32, LinSrgba)>,
}

impl Polygon {
	pub fn new(points: impl IntoIterator<Item = impl Into<Vec2>>) -> Self {
		let points = points
			.into_iter()
			.map(|point| point.into())
			.collect::<Vec<_>>();
		let size = points.iter().copied().reduce(Vec2::max).unwrap_or_default();
		Self {
			id: None,
			points,
			size,
			fill: None,
			stroke: None,
		}
	}

	pub fn fill(self, color: impl Into<LinSrgba>) -> Self {
		Self {
			fill: Some(color.into()),
			..self
		}
	}

	pub fn stroke(self, width: f32, color: impl Into<LinSrgba>) -> Self {
		Self {
			stroke: Some((width, color.into())),
			..self
		}
	}

	pub fn fill_if(self, condition: bool, color: impl Into<LinSrgba>) -> Self {
		if condition { self.fill(color) } else { self }
	}

	pub fn stroke_if(self, condition: bool, width: f32, color: impl Into<LinSrgba>) -> Self {
		if condition {
			self.stroke(width, color)
		} else {
			self
		}
	}

	common_functions!();
}

impl Widget for Polygon {
	common_widget_trait_functions!();

	fn name(&self) -> &'static str {
		"polygon"
	}

	fn children(&mut self, _ctx: &mut Context, _state: &mut WidgetState) -> Vec<Box<dyn Widget>> {
		vec![]
	}

	fn allotted_size_for_next_child(
		&mut self,
		_ctx: &mut Context,
		_allotted_size_from_parent: Vec2,
		_previous_child_sizes: &[Vec2],
		_state: &mut WidgetState,
	) -> Vec2 {
		unreachable!()
	}

	fn layout(
		&mut self,
		_ctx: &mut Context,
		_allotted_size_from_parent: Vec2,
		_child_sizes: &[Vec2],
		_state: &mut WidgetState,
	) -> LayoutResult {
		let _span = tracy_client::span!();
		LayoutResult {
			size: self.size,
			child_positions: vec![],
		}
	}

	fn draw_before_children(&mut self, ctx: &mut Context, _size: Vec2, _state: &mut WidgetState) {
		let _span = tracy_client::span!();
		if let Some(fill) = self.fill {
			Mesh::simple_polygon(ctx, ShapeStyle::Fill, self.points.iter().copied())
				.color(fill)
				.draw(ctx);
		}
	}

	fn draw_after_children(&mut self, ctx: &mut Context, _size: Vec2, _state: &mut WidgetState) {
		let _span = tracy_client::span!();
		if let Some((width, color)) = self.stroke {
			Mesh::simple_polygon(ctx, ShapeStyle::Stroke(width), self.points.iter().copied())
				.color(color)
				.draw(ctx);
		}
	}
}
