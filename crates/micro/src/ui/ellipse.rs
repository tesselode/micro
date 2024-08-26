use std::path::Path;

use glam::Vec2;
use palette::LinSrgba;

use crate::{
	graphics::mesh::{Mesh, ShapeStyle},
	with_child_fns, with_sizing_fns, Context,
};

use super::{ChildPathGenerator, MouseInput, Sizing, TookMouse, UiState, Widget};

#[derive(Debug)]
pub struct Ellipse {
	sizing: Sizing,
	fill: Option<LinSrgba>,
	stroke: Option<(f32, LinSrgba)>,
	children: Vec<Box<dyn Widget>>,
	size: Option<Vec2>,
}

impl Ellipse {
	pub fn new() -> Self {
		Self::default()
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

	with_child_fns!();
	with_sizing_fns!();
}

impl Default for Ellipse {
	fn default() -> Self {
		Self {
			sizing: Sizing::MAX,
			fill: Default::default(),
			stroke: Default::default(),
			children: Default::default(),
			size: Default::default(),
		}
	}
}

impl Widget for Ellipse {
	fn name(&self) -> &'static str {
		"ellipse"
	}

	fn size(
		&mut self,
		ctx: &mut Context,
		state: &mut UiState,
		path: &Path,
		allotted_size: Vec2,
	) -> Vec2 {
		let mut child_path_generator = ChildPathGenerator::new();
		let allotted_size_for_children = self.sizing.allotted_size_for_children(allotted_size);
		let child_sizes = self.children.iter_mut().map(|child| {
			let child_path = path.join(child_path_generator.generate(child.name()));
			child.size(ctx, state, &child_path, allotted_size_for_children)
		});
		let parent_size = self.sizing.final_parent_size(allotted_size, child_sizes);
		self.size = Some(parent_size);
		parent_size
	}

	fn use_mouse_input(
		&mut self,
		mouse_input: &MouseInput,
		state: &mut UiState,
		path: &Path,
	) -> TookMouse {
		let mut child_path_generator = ChildPathGenerator::new();
		for child in self.children.iter_mut().rev() {
			let child_path = path.join(child_path_generator.generate(child.name()));
			let child_took_input = child.use_mouse_input(mouse_input, state, &child_path);
			if child_took_input {
				return true;
			}
		}
		false
	}

	fn draw(&self, ctx: &mut Context, state: &mut UiState, path: &Path) -> anyhow::Result<()> {
		let mut child_path_generator = ChildPathGenerator::new();
		let size = self.size.unwrap();
		if let Some(fill) = self.fill {
			Mesh::ellipse(ctx, ShapeStyle::Fill, size / 2.0, size / 2.0, 0.0)?
				.color(fill)
				.draw(ctx);
		}
		for child in &self.children {
			let child_path = path.join(child_path_generator.generate(child.name()));
			child.draw(ctx, state, &child_path)?;
		}
		if let Some((width, color)) = self.stroke {
			Mesh::ellipse(ctx, ShapeStyle::Stroke(width), size / 2.0, size / 2.0, 0.0)?
				.color(color)
				.draw(ctx);
		}
		Ok(())
	}
}
