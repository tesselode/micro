use glam::{Mat3, Vec2};

use crate::{
	math::{Axis, MainCross},
	Context,
};

use super::{BuiltWidget, Widget};

pub struct Flex {
	pub main_axis: Axis,
	pub children: Vec<(f32, Box<dyn Widget>)>,
}

impl Flex {
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

	pub fn with_child(mut self, weight: f32, child: impl Widget + 'static) -> Self {
		self.children.push((weight, Box::new(child)));
		self
	}

	fn build_children(
		&self,
		ctx: &mut Context,
		parent_size: MainCross,
	) -> Vec<(f32, Box<dyn BuiltWidget>)> {
		let total_weight: f32 = self.children.iter().map(|(weight, _)| *weight).sum();
		self.children
			.iter()
			.map(|(weight, child)| {
				let max_child_size = MainCross::new(
					parent_size.main * (weight / total_weight),
					parent_size.cross,
				);
				(
					*weight,
					child.build(ctx, max_child_size.into_vec2(self.main_axis)),
				)
			})
			.collect()
	}

	fn position_children(
		&self,
		mut built_children: Vec<(f32, Box<dyn BuiltWidget>)>,
		parent_size: MainCross,
	) -> Vec<(Vec2, Box<dyn BuiltWidget>)> {
		let total_weight: f32 = built_children.iter().map(|(weight, _)| *weight).sum();
		let mut next_child_main_axis_position = 0.0;
		built_children
			.drain(..)
			.map(|(weight, child)| {
				let position =
					MainCross::new(next_child_main_axis_position, 0.0).into_vec2(self.main_axis);
				next_child_main_axis_position += parent_size.main * (weight / total_weight);
				(position, child)
			})
			.collect()
	}
}

impl Widget for Flex {
	fn build(&self, ctx: &mut Context, max_size: Vec2) -> Box<dyn BuiltWidget> {
		let parent_size = MainCross::from_vec2(self.main_axis, max_size);
		let built_children = self.build_children(ctx, parent_size);
		let positioned_children = self.position_children(built_children, parent_size);
		Box::new(BuiltFlex {
			size: max_size,
			children: positioned_children,
		})
	}
}

struct BuiltFlex {
	size: Vec2,
	children: Vec<(Vec2, Box<dyn BuiltWidget>)>,
}

impl BuiltWidget for BuiltFlex {
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
