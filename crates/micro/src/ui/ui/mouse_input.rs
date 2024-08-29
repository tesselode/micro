use glam::{Mat4, Vec2};

use crate::{input::MouseButton, Context};

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct MouseInput {
	pub mouse_position: Option<Vec2>,
	pub left_held: bool,
	pub left_held_previous: bool,
}

impl MouseInput {
	pub fn update(&mut self, ctx: &Context) {
		self.mouse_position = Some(ctx.mouse_position().as_vec2());
		self.left_held_previous = self.left_held;
		self.left_held = ctx.is_mouse_button_down(MouseButton::Left);
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
