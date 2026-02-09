use std::collections::HashMap;

use micro::{
	Context,
	input::MouseButton,
	math::{Mat4, Vec2},
};

#[derive(Debug, Clone, PartialEq)]
pub struct MouseInput {
	pub mouse_position: Option<Vec2>,
	held_state: HashMap<MouseButton, HeldState>,
}

impl MouseInput {
	pub fn new() -> Self {
		Self {
			mouse_position: None,
			held_state: MouseButton::KNOWN
				.iter()
				.map(|button| (*button, HeldState::default()))
				.collect(),
		}
	}

	pub fn update(&mut self, ctx: &Context, transform: Mat4) {
		let raw_mouse_position = ctx.mouse_position();
		let transformed_mouse_position = transform
			.transform_point3(raw_mouse_position.extend(0.0))
			.truncate();
		self.mouse_position = Some(transformed_mouse_position);
		for (button, held_state) in &mut self.held_state {
			held_state.held_previous = held_state.held;
			held_state.held = ctx.is_mouse_button_down(*button);
		}
	}

	pub fn transformed(&self, transform: Mat4) -> Self {
		Self {
			mouse_position: self
				.mouse_position
				.map(|position| transform.transform_point3(position.extend(0.0)).truncate()),
			..self.clone()
		}
	}

	pub fn translated(&self, translation: Vec2) -> Self {
		Self {
			mouse_position: self.mouse_position.map(|position| position + translation),
			..self.clone()
		}
	}

	pub fn pressed(&self, button: MouseButton) -> bool {
		let held_state = &self.held_state[&button];
		held_state.held && !held_state.held_previous
	}

	pub fn released(&self, button: MouseButton) -> bool {
		let held_state = &self.held_state[&button];
		!held_state.held && held_state.held_previous
	}
}

impl Default for MouseInput {
	fn default() -> Self {
		Self::new()
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
struct HeldState {
	held: bool,
	held_previous: bool,
}
