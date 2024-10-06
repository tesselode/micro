use std::fmt::Debug;

use micro::{
	math::{vec2, Vec2},
	Context,
};

use crate::with_child_fns;

use super::{AxisSizing, LayoutResult, Widget, WidgetMouseEventChannel};

#[derive(Debug)]
pub struct Stack {
	direction: Axis,
	settings: StackSettings,
	children: Vec<Box<dyn Widget>>,
	mouse_event_channel: Option<WidgetMouseEventChannel>,
}

impl Stack {
	pub fn horizontal(settings: StackSettings) -> Self {
		Self {
			direction: Axis::Horizontal,
			settings,
			children: vec![],
			mouse_event_channel: None,
		}
	}

	pub fn vertical(settings: StackSettings) -> Self {
		Self {
			direction: Axis::Vertical,
			settings,
			children: vec![],
			mouse_event_channel: None,
		}
	}

	pub fn with_mouse_event_channel(self, channel: &WidgetMouseEventChannel) -> Self {
		Self {
			mouse_event_channel: Some(channel.clone()),
			..self
		}
	}

	with_child_fns!();
}

impl Widget for Stack {
	fn name(&self) -> &'static str {
		"stack"
	}

	fn children(&self) -> &[Box<dyn Widget>] {
		&self.children
	}

	fn mouse_event_channel(&self) -> Option<&WidgetMouseEventChannel> {
		self.mouse_event_channel.as_ref()
	}

	fn allotted_size_for_next_child(
		&self,
		allotted_size_from_parent: Vec2,
		previous_child_sizes: &[Vec2],
	) -> Vec2 {
		let total_child_main_axis_size =
			total_child_main_axis_size(self.direction, previous_child_sizes, self.settings.gap);
		match self.direction {
			Axis::Horizontal => vec2(
				allotted_size_from_parent.x - total_child_main_axis_size,
				allotted_size_from_parent.y,
			),
			Axis::Vertical => vec2(
				allotted_size_from_parent.x,
				allotted_size_from_parent.y - total_child_main_axis_size,
			),
		}
	}

	fn layout(
		&self,
		_ctx: &mut Context,
		allotted_size_from_parent: Vec2,
		child_sizes: &[Vec2],
	) -> LayoutResult {
		match self.direction {
			Axis::Horizontal => {
				let parent_size = vec2(
					total_child_main_axis_size(self.direction, child_sizes, self.settings.gap),
					self.settings.cross_sizing.final_parent_size(
						allotted_size_from_parent.y,
						child_sizes.iter().map(|size| size.y),
					),
				);
				let mut next_child_x = 0.0;
				let child_positions = child_sizes
					.iter()
					.map(|size| {
						let x = next_child_x;
						next_child_x += size.x + self.settings.gap;
						vec2(x, (parent_size.y - size.y) * self.settings.cross_align)
					})
					.collect();
				LayoutResult {
					size: parent_size,
					child_positions,
				}
			}
			Axis::Vertical => {
				let parent_size = vec2(
					self.settings.cross_sizing.final_parent_size(
						allotted_size_from_parent.x,
						child_sizes.iter().map(|size| size.x),
					),
					total_child_main_axis_size(self.direction, child_sizes, self.settings.gap),
				);
				let mut next_child_y = 0.0;
				let child_positions = child_sizes
					.iter()
					.map(|size| {
						let y = next_child_y;
						next_child_y += size.y + self.settings.gap;
						vec2((parent_size.x - size.x) * self.settings.cross_align, y)
					})
					.collect();
				LayoutResult {
					size: parent_size,
					child_positions,
				}
			}
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StackSettings {
	pub gap: f32,
	pub cross_align: f32,
	pub cross_sizing: AxisSizing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Axis {
	Horizontal,
	Vertical,
}

fn total_child_main_axis_size(direction: Axis, child_sizes: &[Vec2], gap: f32) -> f32 {
	let num_gaps = child_sizes.len().saturating_sub(1);
	let total_gap_size = num_gaps as f32 * gap;
	child_sizes
		.iter()
		.map(|size| match direction {
			Axis::Horizontal => size.x,
			Axis::Vertical => size.y,
		})
		.sum::<f32>()
		+ total_gap_size
}
