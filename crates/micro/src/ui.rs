mod align;
mod child_path_generator;
mod ellipse;
mod image;
mod macros;
mod mask;
mod match_size;
mod mouse_input;
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
pub use child_path_generator::*;
pub use ellipse::*;
pub use image::*;
pub use mask::*;
pub use match_size::*;
pub use mouse_input::*;
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
	mouse_input: MouseInput,
}

impl Ui {
	pub fn new() -> Self {
		Self {
			state: UiState::new(),
			mouse_input: MouseInput::new(),
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
		self.mouse_input.update(ctx);
		let root_path = PathBuf::new();
		widget.size(ctx, &mut self.state, &root_path, size);
		widget.use_mouse_input(&self.mouse_input, &mut self.state, &root_path);
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

	fn use_mouse_input(
		&mut self,
		mouse_input: &MouseInput,
		state: &mut UiState,
		path: &Path,
	) -> TookMouse;

	fn draw(&self, ctx: &mut Context, state: &mut UiState, path: &Path) -> anyhow::Result<()>;
}

pub type TookMouse = bool;
