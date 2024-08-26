use glam::Vec2;

use crate::{with_child_fns, with_sizing_fns, Context};

use super::{Sizing, Widget};

#[derive(Debug)]
pub struct Align {
	align: Vec2,
	sizing: Sizing,
	children: Vec<Box<dyn Widget>>,
	sizing_pass_results: Option<SizingPassResults>,
}

macro_rules! align_constructors {
	($($name:ident: $align:expr),*$(,)?) => {
		$(
			pub fn $name() -> Self {
				Self::new($align)
			}
		)*
	};
}

impl Align {
	pub fn new(align: impl Into<Vec2>) -> Self {
		Self {
			align: align.into(),
			sizing: Sizing::MAX,
			children: vec![],
			sizing_pass_results: None,
		}
	}

	align_constructors! {
		top_left: (0.0, 0.0),
		top_center: (0.5, 0.0),
		top_right: (1.0, 0.0),
		middle_right: (1.0, 0.5),
		bottom_right: (1.0, 1.0),
		bottom_center: (0.5, 1.0),
		bottom_left: (0.0, 1.0),
		middle_left: (0.0, 0.5),
		center: (0.5, 0.5),
	}

	with_child_fns!();
	with_sizing_fns!();
}

impl Widget for Align {
	fn size(&mut self, ctx: &mut Context, allotted_size: Vec2) -> Vec2 {
		let allotted_size_for_children = self.sizing.allotted_size_for_children(allotted_size);
		let child_sizes = self
			.children
			.iter_mut()
			.map(|child| child.size(ctx, allotted_size_for_children))
			.collect::<Vec<_>>();
		let parent_size = self
			.sizing
			.final_parent_size(allotted_size, child_sizes.iter().copied());
		let child_positions = child_sizes
			.iter()
			.copied()
			.map(|size| (parent_size - size) * self.align)
			.collect();
		self.sizing_pass_results = Some(SizingPassResults {
			size: parent_size,
			child_positions,
		});
		parent_size
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

#[derive(Debug, Clone, PartialEq)]
struct SizingPassResults {
	size: Vec2,
	child_positions: Vec<Vec2>,
}
