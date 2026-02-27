use std::{cell::RefCell, collections::HashMap, rc::Rc};

use micro::{
	input::MouseButton,
	math::{Rect, Vec2},
};

use super::mouse_input::MouseInput;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct WidgetMouseState(Rc<RefCell<WidgetMouseStateInner>>);

impl WidgetMouseState {
	pub fn new() -> Self {
		Self(Rc::new(RefCell::new(WidgetMouseStateInner::new())))
	}

	/// If the mouse is inside the widget, returns the mouse position relative
	/// to the widget's top-left corner. Otherwise, returns `None`.
	pub fn relative_pos(&self) -> Option<Vec2> {
		self.0.borrow().relative_pos
	}

	/// If the mouse was inside the widget this frame and the previous frame,
	/// returns how much it moved this frame. Otherwise, returns `None`.
	pub fn delta(&self) -> Option<Vec2> {
		let state = self.0.borrow();
		state
			.relative_pos
			.zip(state.relative_pos_previous)
			.map(|(current, previous)| current - previous)
	}

	pub fn wheel_delta(&self) -> Vec2 {
		self.0.borrow().wheel_delta
	}

	/// Returns `true` if the mouse is inside the widget.
	pub fn hovered(&self) -> bool {
		self.0.borrow().relative_pos.is_some()
	}

	/// Returns `true` if the mouse just started hovering the widget this frame.
	pub fn entered(&self) -> bool {
		let state = self.0.borrow();
		state.relative_pos.is_some() && state.relative_pos_previous.is_none()
	}

	/// Returns `true` if the mouse just stopped hovering the widget this frame.
	pub fn exited(&self) -> bool {
		let state = self.0.borrow();
		state.relative_pos.is_none() && state.relative_pos_previous.is_some()
	}

	/// Returns `true` if the widget is being held down by the specified mouse button.
	pub fn held(&self, button: MouseButton) -> bool {
		self.0.borrow().button_state[&button].held
	}

	/// Returns `true` if the widget just started being held by the specified mouse
	/// button this frame.
	pub fn pressed(&self, button: MouseButton) -> bool {
		let button_state = self.0.borrow().button_state[&button];
		button_state.held && !button_state.held_previous
	}

	/// Returns `true` if the widget just stopped being held by the specified mouse
	/// button this frame.
	pub fn released(&self, button: MouseButton) -> bool {
		let button_state = self.0.borrow().button_state[&button];
		!button_state.held && button_state.held_previous
	}

	/// Returns `true` if the widget was clicked by the specified mouse button this
	/// frame. A "click" means the widget was released while hovered.
	pub fn clicked(&self, button: MouseButton) -> bool {
		self.0.borrow().button_state[&button].clicked
	}

	pub(crate) fn update(&self, mouse_input: &MouseInput, size: Vec2) {
		let mut state = self.0.borrow_mut();
		state.relative_pos_previous = state.relative_pos;
		state.relative_pos = mouse_input
			.position
			.filter(|position| Rect::new(Vec2::ZERO, size).contains_point(*position));
		let hovered = state.relative_pos.is_some();
		state.wheel_delta = if hovered {
			mouse_input.wheel_delta
		} else {
			Vec2::ZERO
		};
		for (
			button,
			ButtonState {
				held,
				held_previous,
				clicked,
			},
		) in &mut state.button_state
		{
			*clicked = false;
			*held_previous = *held;
			if mouse_input.pressed(*button) && hovered {
				*held = true;
			}
			if mouse_input.released(*button) {
				if *held && hovered {
					*clicked = true;
				}
				*held = false;
			}
		}
	}
}

#[derive(Debug, Clone, PartialEq)]
struct WidgetMouseStateInner {
	relative_pos: Option<Vec2>,
	relative_pos_previous: Option<Vec2>,
	wheel_delta: Vec2,
	button_state: HashMap<MouseButton, ButtonState>,
}

impl WidgetMouseStateInner {
	fn new() -> Self {
		Self {
			relative_pos: None,
			relative_pos_previous: None,
			wheel_delta: Vec2::ZERO,
			button_state: MouseButton::KNOWN
				.iter()
				.copied()
				.map(|button| (button, ButtonState::default()))
				.collect(),
		}
	}
}

impl Default for WidgetMouseStateInner {
	fn default() -> Self {
		Self::new()
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
struct ButtonState {
	held: bool,
	held_previous: bool,
	clicked: bool,
}
