use std::{cell::RefCell, collections::HashMap, rc::Rc};

use micro::{
	input::MouseButton,
	math::{Mat4, Rect, Vec2},
};

use crate::{ButtonState, WidgetState};

#[derive(Debug, Clone)]
pub struct WidgetInspector(Rc<RefCell<WidgetInspectorInner>>);

impl WidgetInspector {
	pub fn new() -> Self {
		Self(Rc::new(RefCell::new(WidgetInspectorInner::new())))
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

	pub(crate) fn populate_from_state(&self, state: &WidgetState) {
		let mut inner = self.0.borrow_mut();
		inner.bounds = state.bounds;
		inner.transform = state.transform;
		inner.relative_pos = state.relative_pos;
		inner.delta = state.delta;
		inner.wheel_delta = state.wheel_delta;
		inner.hovered = state.hovered;
		inner.hovered_previous = state.hovered_previous;
		inner.button_state = state.button_state.clone();
	}
}

impl Default for WidgetInspector {
	fn default() -> Self {
		Self::new()
	}
}

#[derive(Debug, Clone, PartialEq)]
struct WidgetInspectorInner {
	pub(crate) bounds: Option<Rect>,
	pub(crate) transform: Option<Mat4>,
	relative_pos: Option<Vec2>,
	delta: Vec2,
	wheel_delta: Vec2,
	hovered: bool,
	hovered_previous: bool,
	button_state: HashMap<MouseButton, ButtonState>,
}

impl WidgetInspectorInner {
	fn new() -> Self {
		Self {
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
