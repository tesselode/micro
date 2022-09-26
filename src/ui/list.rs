use glam::{Mat3, Vec2};

use crate::{
	math::{Axis, MainCross},
	Context,
};

use super::{BuiltWidget, Constraints, Widget};

pub struct List {
	pub main_axis: Axis,
	pub mode: Mode,
	pub cross_axis_alignment: f32,
	pub children: Vec<Box<dyn Widget>>,
}

impl List {
	pub fn new(main_axis: Axis) -> Self {
		Self {
			main_axis,
			mode: Mode::Stack { item_gap: 0.0 },
			cross_axis_alignment: 0.0,
			children: vec![],
		}
	}

	pub fn horizontal() -> Self {
		Self::new(Axis::Horizontal)
	}

	pub fn vertical() -> Self {
		Self::new(Axis::Vertical)
	}

	pub fn with_mode(self, mode: Mode) -> Self {
		Self { mode, ..self }
	}

	pub fn with_cross_axis_alignment(self, cross_axis_alignment: f32) -> Self {
		Self {
			cross_axis_alignment,
			..self
		}
	}

	pub fn with_child(mut self, child: impl Widget + 'static) -> Self {
		self.children.push(Box::new(child));
		self
	}

	fn build_children(&self, ctx: &mut Context, max_size: MainCross) -> Vec<Box<dyn BuiltWidget>> {
		let mut remaining_main_axis_size = max_size.main;
		self.children
			.iter()
			.map(|child| {
				let max_child_size = MainCross::new(remaining_main_axis_size, max_size.cross);
				let built_child = child.build(
					ctx,
					Constraints {
						min_size: Vec2::ZERO,
						max_size: max_child_size.into_vec2(self.main_axis),
					},
				);
				remaining_main_axis_size -=
					MainCross::from_vec2(self.main_axis, built_child.size()).main;
				built_child
			})
			.collect()
	}

	fn total_size(
		&self,
		built_children: &[Box<dyn BuiltWidget>],
		max_size: MainCross,
	) -> MainCross {
		let main_axis_size = match self.mode {
			Mode::Stack { item_gap } => {
				let main_axis_size_without_gap = built_children.iter().fold(0.0, |main, child| {
					let child_main_size = MainCross::from_vec2(self.main_axis, child.size()).main;
					main + child_main_size
				});
				let item_gap_total_size = if built_children.is_empty() {
					0.0
				} else {
					(built_children.len() - 1) as f32 * item_gap
				};
				(main_axis_size_without_gap + item_gap_total_size).min(max_size.main)
			}
			Mode::SpaceEvenly => max_size.main,
		};
		MainCross {
			main: main_axis_size,
			cross: built_children
				.iter()
				.fold(0.0f32, |cross, child| {
					let child_cross_size = MainCross::from_vec2(self.main_axis, child.size()).cross;
					cross.max(child_cross_size)
				})
				.min(max_size.cross),
		}
	}

	fn position_children(
		&self,
		mut built_children: Vec<Box<dyn BuiltWidget>>,
		main_axis_max_size: f32,
		cross_axis_size: f32,
	) -> Vec<(Vec2, Box<dyn BuiltWidget>)> {
		match self.mode {
			Mode::Stack { item_gap } => {
				self.position_children_as_stack(&mut built_children, cross_axis_size, item_gap)
			}
			Mode::SpaceEvenly => {
				if built_children.is_empty() {
					return vec![];
				}
				let children_main_axis_size =
					built_children
						.iter()
						.fold(0.0, |children_main_axis_size, child| {
							children_main_axis_size
								+ MainCross::from_vec2(self.main_axis, child.size()).main
						});
				let item_gap = (main_axis_max_size - children_main_axis_size)
					/ (built_children.len() as f32 - 1.0);
				self.position_children_as_stack(&mut built_children, cross_axis_size, item_gap)
			}
		}
	}

	fn position_children_as_stack(
		&self,
		built_children: &mut Vec<Box<dyn BuiltWidget>>,
		cross_axis_size: f32,
		item_gap: f32,
	) -> Vec<(Vec2, Box<dyn BuiltWidget>)> {
		let mut next_child_main_axis_position = 0.0;
		built_children
			.drain(..)
			.map(|child| {
				let child_size = MainCross::from_vec2(self.main_axis, child.size());
				let position = MainCross::new(
					next_child_main_axis_position,
					(cross_axis_size - child_size.cross) * self.cross_axis_alignment,
				)
				.into_vec2(self.main_axis);
				next_child_main_axis_position +=
					MainCross::from_vec2(self.main_axis, child.size()).main + item_gap;
				(position, child)
			})
			.collect()
	}
}

impl Widget for List {
	fn build(&self, ctx: &mut Context, constraints: Constraints) -> Box<dyn BuiltWidget> {
		let max_size = MainCross::from_vec2(self.main_axis, constraints.max_size);
		let built_children = self.build_children(ctx, max_size);
		let total_size = self.total_size(&built_children, max_size);
		let positioned_children =
			self.position_children(built_children, max_size.main, total_size.cross);
		Box::new(BuiltList {
			size: total_size.into_vec2(self.main_axis),
			children: positioned_children,
		})
	}
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Mode {
	Stack { item_gap: f32 },
	SpaceEvenly,
}

struct BuiltList {
	size: Vec2,
	children: Vec<(Vec2, Box<dyn BuiltWidget>)>,
}

impl BuiltWidget for BuiltList {
	fn size(&self) -> Vec2 {
		self.size
	}

	fn draw(&self, ctx: &mut Context) {
		for (position, child) in &self.children {
			ctx.with_transform(Mat3::from_translation(*position), |ctx| {
				child.draw(ctx);
			});
		}
	}
}
