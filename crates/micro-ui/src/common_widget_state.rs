use std::{cell::RefCell, collections::HashMap, rc::Rc};

use micro::{
	input::MouseButton,
	math::{Mat4, Rect, Vec2},
};

use crate::mouse_input::MouseInput;

#[derive(Debug, Clone, PartialEq)]
pub struct CommonWidgetState(pub(crate) Rc<RefCell<CommonWidgetStateInner>>);

impl CommonWidgetState {
	pub fn new() -> Self {
		Self(Rc::new(RefCell::new(CommonWidgetStateInner::new())))
	}

	pub fn bounds(&self) -> Option<Rect> {
		self.0.borrow().bounds
	}

	pub fn transform(&self) -> Option<Mat4> {
		self.0.borrow().transform
	}

	/// Returns the mouse position relative to the widget's top-left corner
	/// (in this widget's coordinate system). Returns `None` if the widget
	/// hasn't been rendered yet.
	pub fn relative_pos(&self) -> Option<Vec2> {
		self.0.borrow().relative_pos
	}

	/// Returns how much the mouse moved this frame (in this widget's
	/// coordinate system).
	pub fn delta(&self) -> Vec2 {
		self.0.borrow().delta
	}

	pub fn wheel_delta(&self) -> Vec2 {
		self.0.borrow().wheel_delta
	}

	/// Returns `true` if the mouse is inside the widget.
	pub fn hovered(&self) -> bool {
		self.0.borrow().hovered
	}

	/// Returns `true` if the mouse just started hovering the widget this frame.
	pub fn entered(&self) -> bool {
		let inner = self.0.borrow();
		inner.hovered && !inner.hovered_previous
	}

	/// Returns `true` if the mouse just stopped hovering the widget this frame.
	pub fn exited(&self) -> bool {
		let inner = self.0.borrow();
		!inner.hovered && inner.hovered_previous
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

	pub(crate) fn update_mouse_state(&self, mouse_input: &MouseInput, size: Vec2) {
		let mut inner = self.0.borrow_mut();
		inner.relative_pos = mouse_input.position;
		inner.delta = mouse_input.delta;
		let hovered = mouse_input
			.position
			.is_some_and(|position| Rect::new(Vec2::ZERO, size).contains_point(position));
		inner.wheel_delta = if hovered {
			mouse_input.wheel_delta
		} else {
			Vec2::ZERO
		};
		inner.hovered_previous = inner.hovered;
		inner.hovered = hovered;
		for (
			button,
			ButtonState {
				held,
				held_previous,
				clicked,
			},
		) in &mut inner.button_state
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

impl Default for CommonWidgetState {
	fn default() -> Self {
		Self::new()
	}
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct CommonWidgetStateInner {
	pub(crate) used: bool,
	pub(crate) bounds: Option<Rect>,
	pub(crate) transform: Option<Mat4>,
	relative_pos: Option<Vec2>,
	delta: Vec2,
	wheel_delta: Vec2,
	hovered: bool,
	hovered_previous: bool,
	button_state: HashMap<MouseButton, ButtonState>,
}

impl CommonWidgetStateInner {
	pub(crate) fn new() -> Self {
		Self {
			used: true,
			bounds: None,
			transform: None,
			relative_pos: None,
			delta: Vec2::ZERO,
			wheel_delta: Vec2::ZERO,
			hovered: false,
			hovered_previous: false,
			button_state: MouseButton::KNOWN
				.iter()
				.copied()
				.map(|button| (button, ButtonState::default()))
				.collect(),
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
struct ButtonState {
	held: bool,
	held_previous: bool,
	clicked: bool,
}
