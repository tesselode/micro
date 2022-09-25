use glam::{Mat3, Vec2};

use crate::{
	math::{Axis, MainCross},
	Context,
};

use super::{BuiltWidget, Constraints, Widget};

pub struct List {
	pub main_axis: Axis,
	pub children: Vec<Box<dyn Widget>>,
}

impl List {
	pub fn new(main_axis: Axis) -> Self {
		Self {
			main_axis,
			children: vec![],
		}
	}

	pub fn horizontal() -> Self {
		Self::new(Axis::Horizontal)
	}

	pub fn vertical() -> Self {
		Self::new(Axis::Vertical)
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
		MainCross {
			main: built_children
				.iter()
				.fold(0.0, |main, child| {
					let child_main_size = MainCross::from_vec2(self.main_axis, child.size()).main;
					main + child_main_size
				})
				.min(max_size.main),
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
		cross_axis_size: f32,
	) -> Vec<(Vec2, Box<dyn BuiltWidget>)> {
		let mut next_child_main_axis_position = 0.0;
		built_children
			.drain(..)
			.map(|child| {
				let position =
					MainCross::new(next_child_main_axis_position, 0.0).into_vec2(self.main_axis);
				next_child_main_axis_position +=
					MainCross::from_vec2(self.main_axis, child.size()).main;
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
		let positioned_children = self.position_children(built_children, total_size.cross);
		Box::new(BuiltList {
			size: total_size.into_vec2(self.main_axis),
			children: positioned_children,
		})
	}
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
