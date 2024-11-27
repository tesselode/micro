use micro::{
	color::LinSrgba,
	graphics::mesh::{Mesh, ShapeStyle},
	math::Vec2,
	Context,
};

use super::{LayoutResult, Widget, WidgetMouseEventChannel};

#[derive(Debug)]
pub struct Polygon {
	points: Vec<Vec2>,
	size: Vec2,
	fill: Option<LinSrgba>,
	stroke: Option<(f32, LinSrgba)>,
	mouse_event_channel: Option<WidgetMouseEventChannel>,
}

impl Polygon {
	pub fn new(points: impl IntoIterator<Item = impl Into<Vec2>>) -> Self {
		let points = points
			.into_iter()
			.map(|point| point.into())
			.collect::<Vec<_>>();
		let size = points.iter().copied().reduce(Vec2::max).unwrap_or_default();
		Self {
			points,
			size,
			fill: None,
			stroke: None,
			mouse_event_channel: None,
		}
	}

	pub fn with_fill(self, color: impl Into<LinSrgba>) -> Self {
		Self {
			fill: Some(color.into()),
			..self
		}
	}

	pub fn with_stroke(self, width: f32, color: impl Into<LinSrgba>) -> Self {
		Self {
			stroke: Some((width, color.into())),
			..self
		}
	}

	pub fn with_mouse_event_channel(self, channel: &WidgetMouseEventChannel) -> Self {
		Self {
			mouse_event_channel: Some(channel.clone()),
			..self
		}
	}
}

impl Widget for Polygon {
	fn name(&self) -> &'static str {
		"polygon"
	}

	fn children(&self) -> &[Box<dyn Widget>] {
		&[]
	}

	fn mouse_event_channel(&self) -> Option<&WidgetMouseEventChannel> {
		self.mouse_event_channel.as_ref()
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
		LayoutResult {
			size: self.size,
			child_positions: vec![],
		}
	}

	fn draw_before_children(&self, ctx: &mut Context, _size: Vec2) -> anyhow::Result<()> {
		if let Some(fill) = self.fill {
			Mesh::simple_polygon(ctx, ShapeStyle::Fill, self.points.iter().copied())?
				.color(fill)
				.draw(ctx);
		}
		Ok(())
	}

	fn draw_after_children(&self, ctx: &mut Context, _size: Vec2) -> anyhow::Result<()> {
		if let Some((width, color)) = self.stroke {
			Mesh::simple_polygon(ctx, ShapeStyle::Stroke(width), self.points.iter().copied())?
				.color(color)
				.draw(ctx);
		}
		Ok(())
	}
}
