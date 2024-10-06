use micro::math::{Rect, Vec2};

use super::mouse_input::MouseInput;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct WidgetMouseState {
	pub hovered: bool,
	pub hovered_previous: bool,
	pub pressed: bool,
}

impl WidgetMouseState {
	pub fn update(&mut self, mouse_input: MouseInput, size: Vec2) -> UpdateMouseStateResult {
		self.hovered_previous = self.hovered;
		self.hovered = mouse_input
			.mouse_position
			.is_some_and(|position| Rect::new(Vec2::ZERO, size).contains_point(position));
		let click_started = if mouse_input.left_pressed() && self.hovered {
			self.pressed = true;
			true
		} else {
			false
		};
		let clicked = if mouse_input.left_released() && self.pressed && self.hovered {
			self.pressed = false;
			true
		} else {
			false
		};
		if mouse_input.left_released() {
			self.pressed = false;
		}
		UpdateMouseStateResult {
			click_started,
			clicked,
			hovered: self.hovered && !self.hovered_previous,
			unhovered: self.hovered_previous && !self.hovered,
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UpdateMouseStateResult {
	pub hovered: bool,
	pub unhovered: bool,
	pub click_started: bool,
	pub clicked: bool,
}
