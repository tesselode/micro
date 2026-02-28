use micro::{Context, color::LinSrgba, graphics::mesh::Mesh, math::Vec2};

use crate::{WidgetInspector, common_functions, common_widget_trait_functions};

use super::{LayoutResult, Widget, WidgetMouseState};

#[derive(Debug)]
pub struct Polyline {
	points: Vec<Vec2>,
	stroke_width: f32,
	color: LinSrgba,
	size: Vec2,
	mouse_state: Option<WidgetMouseState>,
	inspector: Option<WidgetInspector>,
}

impl Polyline {
	pub fn new(
		points: impl IntoIterator<Item = impl Into<Vec2>>,
		stroke_width: f32,
		color: impl Into<LinSrgba>,
	) -> Self {
		let points = points
			.into_iter()
			.map(|point| point.into())
			.collect::<Vec<_>>();
		let size = points.iter().copied().reduce(Vec2::max).unwrap_or_default();
		Self {
			points,
			stroke_width,
			color: color.into(),
			size,
			mouse_state: None,
			inspector: None,
		}
	}

	common_functions!();
}

impl Widget for Polyline {
	common_widget_trait_functions!();

	fn name(&self) -> &'static str {
		"polyline"
	}

	fn children(&self) -> &[Box<dyn Widget>] {
		&[]
	}

	fn allotted_size_for_next_child(
		&self,
		_allotted_size_from_parent: Vec2,
		_previous_child_sizes: &[Vec2],
	) -> Vec2 {
		unreachable!()
	}

	fn layout(
		&self,
		_ctx: &mut Context,
		_allotted_size_from_parent: Vec2,
		_child_sizes: &[Vec2],
	) -> LayoutResult {
		let _span = tracy_client::span!();
		LayoutResult {
			size: self.size,
			child_positions: vec![],
		}
	}

	fn draw_before_children(&self, ctx: &mut Context, _size: Vec2) {
		let _span = tracy_client::span!();
		Mesh::simple_polyline(ctx, self.stroke_width, self.points.iter().copied())
			.color(self.color)
			.draw(ctx);
	}
}
