use micro::{
	input::MouseButton,
	is_mouse_button_down,
	math::{Mat4, Vec2},
	mouse_position,
};

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct MouseInput {
	pub mouse_position: Option<Vec2>,
	pub left_held: bool,
	pub left_held_previous: bool,
}

impl MouseInput {
	pub fn update(&mut self, transform: Mat4) {
		let raw_mouse_position = mouse_position().as_vec2();
		let transformed_mouse_position = transform
			.transform_point3(raw_mouse_position.extend(0.0))
			.truncate();
		self.mouse_position = Some(transformed_mouse_position);
		self.left_held_previous = self.left_held;
		self.left_held = is_mouse_button_down(MouseButton::Left);
	}

	pub fn transformed(self, transform: Mat4) -> Self {
		Self {
			mouse_position: self
				.mouse_position
				.map(|position| transform.transform_point3(position.extend(0.0)).truncate()),
			..self
		}
	}

	pub fn translated(self, translation: Vec2) -> Self {
		Self {
			mouse_position: self.mouse_position.map(|position| position + translation),
			..self
		}
	}

	pub fn left_pressed(self) -> bool {
		self.left_held && !self.left_held_previous
	}

	pub fn left_released(self) -> bool {
		self.left_held_previous && !self.left_held
	}
}
