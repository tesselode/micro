use exhaust::Exhaust;
use glam::Vec2;
use indexmap::IndexSet;

use crate::{input::MouseButton, Context};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct MouseInput {
	previous: MouseState,
	current: MouseState,
}

impl MouseInput {
	pub fn new() -> Self {
		Self::default()
	}

	pub fn translated(&self, translation: Vec2) -> Self {
		Self {
			previous: self.previous.translated(translation),
			current: self.current.translated(translation),
		}
	}

	pub fn update(&mut self, ctx: &Context) {
		std::mem::swap(&mut self.previous, &mut self.current);
		self.current = MouseState::current(ctx);
	}

	pub fn position(&self) -> Option<Vec2> {
		self.current.position
	}

	pub fn delta(&self) -> Option<Vec2> {
		self.previous
			.position
			.zip(self.current.position)
			.map(|(previous, current)| current - previous)
	}

	pub fn held(&self, button: MouseButton) -> bool {
		self.current.held_buttons.contains(&button)
	}

	pub fn pressed(&self, button: MouseButton) -> bool {
		self.current.held_buttons.contains(&button) && !self.previous.held_buttons.contains(&button)
	}

	pub fn released(&self, button: MouseButton) -> bool {
		self.previous.held_buttons.contains(&button) && !self.current.held_buttons.contains(&button)
	}
}

#[derive(Debug, Clone, PartialEq, Default)]
struct MouseState {
	position: Option<Vec2>,
	held_buttons: IndexSet<MouseButton>,
}

impl MouseState {
	fn current(ctx: &Context) -> Self {
		Self {
			position: Some(ctx.mouse_position().as_vec2()),
			held_buttons: MouseButton::exhaust()
				.filter(|button| ctx.is_mouse_button_down(*button))
				.collect(),
		}
	}

	fn translated(&self, translation: Vec2) -> Self {
		Self {
			position: self.position.map(|position| position + translation),
			held_buttons: self.held_buttons.clone(),
		}
	}
}
