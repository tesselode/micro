mod align;
mod ellipse;
mod image;
mod macros;
mod mask;
mod match_size;
mod padding;
mod polygon;
mod polyline;
mod rectangle;
mod sizing;
mod stack;
mod state;
mod text;
mod transform;

pub use align::*;
pub use ellipse::*;
pub use image::*;
use indexmap::IndexMap;
pub use mask::*;
pub use match_size::*;
pub use padding::*;
pub use polygon::*;
pub use polyline::*;
pub use rectangle::*;
pub use sizing::*;
pub use stack::*;
pub use state::*;
pub use text::{TextSettings, TextShadow, TextSizeReporting, TextSizing, TextWidget as Text};
pub use transform::*;

use std::{
	fmt::Debug,
	path::{Path, PathBuf},
};

use glam::Vec2;

use crate::Context;

pub struct Ui {
	state: UiState,
}

impl Ui {
	pub fn new() -> Self {
		Self {
			state: UiState::new(),
		}
	}

	pub fn render(
		&mut self,
		ctx: &mut Context,
		size: Vec2,
		mut widget: impl Widget,
	) -> anyhow::Result<()> {
		self.state.remove_unused();
		self.state.reset_used();
		let root_path = PathBuf::new();
		widget.size(ctx, &mut self.state, &root_path, size);
		widget.draw(ctx, &mut self.state, &root_path)?;
		Ok(())
	}
}

impl Default for Ui {
	fn default() -> Self {
		Self::new()
	}
}

pub trait Widget: Debug {
	fn name(&self) -> &'static str;

	fn size(
		&mut self,
		ctx: &mut Context,
		state: &mut UiState,
		path: &Path,
		allotted_size: Vec2,
	) -> Vec2;

	fn draw(&self, ctx: &mut Context, state: &mut UiState, path: &Path) -> anyhow::Result<()>;
}

pub struct ChildPathGenerator {
	num_widgets: IndexMap<&'static str, usize>,
}

impl ChildPathGenerator {
	pub fn new() -> Self {
		Self {
			num_widgets: IndexMap::new(),
		}
	}

	pub fn generate(&mut self, widget_name: &'static str) -> String {
		let num_widgets_with_name = self.num_widgets.entry(widget_name).or_default();
		let path = format!("{}{}", widget_name, *num_widgets_with_name);
		*num_widgets_with_name += 1;
		path
	}
}

impl Default for ChildPathGenerator {
	fn default() -> Self {
		Self::new()
	}
}
