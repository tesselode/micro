use std::collections::HashMap;

use micro::{
	input::MouseButton,
	is_mouse_button_down,
	math::{Mat4, Vec2},
	mouse_position, mouse_wheel_delta,
};

#[derive(Debug, Clone, PartialEq)]
pub struct MouseInput {
	pub position: Option<Vec2>,
	pub previous_position: Option<Vec2>,
	pub delta: Vec2,
	pub wheel_delta: Vec2,
	held_state: HashMap<MouseButton, HeldState>,
}

impl MouseInput {
	pub fn new() -> Self {
		Self {
			position: None,
			previous_position: None,
			delta: Vec2::ZERO,
			wheel_delta: Vec2::ZERO,
			held_state: MouseButton::KNOWN
				.iter()
				.map(|button| (*button, HeldState::default()))
				.collect(),
		}
	}

	pub fn update(&mut self, transform: Mat4) {
		self.previous_position = self.position;
		let raw_position = mouse_position();
		let transformed_position = transform
			.transform_point3(raw_position.extend(0.0))
			.truncate();
		self.position = Some(transformed_position);
		self.delta = self
			.position
			.zip(self.previous_position)
			.map(|(position, previous)| position - previous)
			.unwrap_or_default();
		self.wheel_delta = mouse_wheel_delta();
		for (button, held_state) in &mut self.held_state {
			held_state.held_previous = held_state.held;
			held_state.held = is_mouse_button_down(*button);
		}
	}

	pub fn transformed(&self, transform: Mat4) -> Self {
		Self {
			position: self
				.position
				.map(|position| transform.transform_point3(position.extend(0.0)).truncate()),
			previous_position: self.previous_position.map(|previous_position| {
				transform
					.transform_point3(previous_position.extend(0.0))
					.truncate()
			}),
			delta: transform
				.transform_vector3(self.delta.extend(0.0))
				.truncate(),
			..self.clone()
		}
	}

	pub fn translated(&self, translation: Vec2) -> Self {
		Self {
			position: self.position.map(|position| position + translation),
			previous_position: self
				.previous_position
				.map(|previous_position| previous_position + translation),
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
