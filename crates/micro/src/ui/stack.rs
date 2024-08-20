use glam::{vec2, Vec2};

use crate::{with_child_fns, Context};

use super::{Axis, CrossSizing, Widget};

#[derive(Debug)]
pub struct Stack {
	direction: Axis,
	settings: StackSettings,
	children: Vec<Box<dyn Widget>>,
	sizing_pass_results: Option<SizingPassResults>,
}

impl Stack {
	pub fn horizontal(settings: StackSettings) -> Self {
		Self {
			direction: Axis::Horizontal,
			settings,
			children: vec![],
			sizing_pass_results: None,
		}
	}

	pub fn vertical(settings: StackSettings) -> Self {
		Self {
			direction: Axis::Vertical,
			settings,
			children: vec![],
			sizing_pass_results: None,
		}
	}

	with_child_fns!();
}

impl Widget for Stack {
	fn size(&mut self, max_size: Vec2) -> Vec2 {
		match self.direction {
			Axis::Horizontal => {
				let mut max_width = max_size.x;
				let child_sizes = self
					.children
					.iter_mut()
					.map(|child| {
						let size = child.size(vec2(max_width, max_size.y));
						max_width -= size.x + self.settings.gap;
						size
					})
					.collect::<Vec<_>>();
				let total_child_width = child_sizes.iter().map(|size| size.x).sum::<f32>();
				let total_gap = self.settings.gap * self.children.len().saturating_sub(1) as f32;
				let stack_size = vec2(
					total_child_width + total_gap,
					match self.settings.cross_sizing {
						CrossSizing::Min => child_sizes
							.iter()
							.map(|size| size.y)
							.reduce(f32::max)
							.unwrap_or_default(),
						CrossSizing::Max => max_size.y,
					},
				);
				let mut next_child_x = 0.0;
				let child_positions = child_sizes
					.iter()
					.map(|child_size| {
						let position = vec2(
							next_child_x,
							(stack_size.y - child_size.y) * self.settings.cross_align,
						);
						next_child_x += child_size.x + self.settings.gap;
						position
					})
					.collect::<Vec<_>>();
				self.sizing_pass_results = Some(SizingPassResults {
					size: stack_size,
					child_positions,
				});
				stack_size
			}
			Axis::Vertical => {
				let mut max_height = max_size.y;
				let child_sizes = self
					.children
					.iter_mut()
					.map(|child| {
						let size = child.size(vec2(max_size.x, max_height));
						max_height -= size.y + self.settings.gap;
						size
					})
					.collect::<Vec<_>>();
				let total_child_height = child_sizes.iter().map(|size| size.y).sum::<f32>();
				let total_gap = self.settings.gap * self.children.len().saturating_sub(1) as f32;
				let stack_size = vec2(
					match self.settings.cross_sizing {
						CrossSizing::Min => child_sizes
							.iter()
							.map(|size| size.x)
							.reduce(f32::max)
							.unwrap_or_default(),
						CrossSizing::Max => max_size.x,
					},
					total_child_height + total_gap,
				);
				let mut next_child_y = 0.0;
				let child_positions = child_sizes
					.iter()
					.map(|child_size| {
						let position = vec2(
							(stack_size.x - child_size.x) * self.settings.cross_align,
							next_child_y,
						);
						next_child_y += child_size.y + self.settings.gap;
						position
					})
					.collect::<Vec<_>>();
				self.sizing_pass_results = Some(SizingPassResults {
					size: stack_size,
					child_positions,
				});
				stack_size
			}
		}
	}

	fn draw(&self, ctx: &mut Context) -> anyhow::Result<()> {
		let SizingPassResults {
			child_positions, ..
		} = self.sizing_pass_results.as_ref().unwrap();
		for (child, &position) in self.children.iter().zip(child_positions.iter()) {
			let ctx = &mut ctx.push_translation_2d(position);
			child.draw(ctx)?;
		}
		Ok(())
	}
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StackSettings {
	pub gap: f32,
	pub cross_align: f32,
	pub cross_sizing: CrossSizing,
}

#[derive(Debug, Clone, PartialEq)]
struct SizingPassResults {
	size: Vec2,
	child_positions: Vec<Vec2>,
}
