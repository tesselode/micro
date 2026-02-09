use micro::{
	egui::ahash::HashSet,
	input::MouseButton,
	math::{Rect, Vec2},
};

use super::mouse_input::MouseInput;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct WidgetMouseState {
	pub hovered: bool,
	pub pressed: HashSet<MouseButton>,
}

impl WidgetMouseState {
	pub fn update(&mut self, mouse_input: &MouseInput, size: Vec2) -> UpdateMouseStateResult {
		let hovered_previous = self.hovered;
		self.hovered = mouse_input
			.mouse_position
			.is_some_and(|position| Rect::new(Vec2::ZERO, size).contains_point(position));
		let click_started = MouseButton::KNOWN
			.iter()
			.copied()
			.filter(|button| {
				if mouse_input.pressed(*button) && self.hovered {
					self.pressed.insert(*button);
					true
				} else {
					false
				}
			})
			.collect();
		let clicked = MouseButton::KNOWN
			.iter()
			.copied()
			.filter(|button| {
				if mouse_input.released(*button) && self.pressed.contains(button) && self.hovered {
					self.pressed.remove(button);
					true
				} else {
					false
				}
			})
			.collect();
		for button in MouseButton::KNOWN {
			if mouse_input.released(button) {
				self.pressed.remove(&button);
			}
		}
		UpdateMouseStateResult {
			hovered: self.hovered && !hovered_previous,
			unhovered: hovered_previous && !self.hovered,
			click_started,
			clicked,
		}
	}
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpdateMouseStateResult {
	pub hovered: bool,
	pub unhovered: bool,
	pub click_started: HashSet<MouseButton>,
	pub clicked: HashSet<MouseButton>,
}
