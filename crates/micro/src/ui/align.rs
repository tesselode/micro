use glam::Vec2;

use crate::{with_child_fns, Context};

use super::Widget;

#[derive(Debug)]
pub struct Align {
	align: Vec2,
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
}

impl Widget for Align {
	fn size(&mut self, ctx: &mut Context, max_size: Vec2) -> Vec2 {
		let child_sizes = self
			.children
			.iter_mut()
			.map(|child| child.size(ctx, max_size));
		let child_positions = child_sizes
			.map(|size| (max_size - size) * self.align)
			.collect();
		self.sizing_pass_results = Some(SizingPassResults {
			size: max_size,
			child_positions,
		});
		max_size
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
