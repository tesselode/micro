use std::{any::Any, collections::HashMap};

use micro::{
	input::MouseButton,
	math::{Mat4, Rect, Vec2},
};

use crate::mouse_input::MouseInput;

#[derive(Debug)]
pub struct WidgetState {
	pub(crate) used: bool,
	pub(crate) bounds: Option<Rect>,
	pub(crate) transform: Option<Mat4>,
	pub(crate) relative_pos: Option<Vec2>,
	pub(crate) delta: Vec2,
	pub(crate) wheel_delta: Vec2,
	pub(crate) hovered: bool,
	pub(crate) hovered_previous: bool,
	pub(crate) button_state: HashMap<MouseButton, ButtonState>,
	custom: Option<Box<dyn Any>>,
}

impl WidgetState {
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
			custom: None,
		}
	}

	pub fn bounds(&self) -> Option<Rect> {
		self.bounds
	}

	pub fn transform(&self) -> Option<Mat4> {
		self.transform
	}

	/// Returns the mouse position relative to the widget's top-left corner
	/// (in this widget's coordinate system). Returns `None` if the widget
	/// hasn't been rendered yet.
	pub fn relative_pos(&self) -> Option<Vec2> {
		self.relative_pos
	}

	/// Returns how much the mouse moved this frame (in this widget's
	/// coordinate system).
	pub fn delta(&self) -> Vec2 {
		self.delta
	}

	pub fn wheel_delta(&self) -> Vec2 {
		self.wheel_delta
	}

	/// Returns `true` if the mouse is inside the widget.
	pub fn hovered(&self) -> bool {
		self.hovered
	}

	/// Returns `true` if the mouse just started hovering the widget this frame.
	pub fn entered(&self) -> bool {
		self.hovered && !self.hovered_previous
	}

	/// Returns `true` if the mouse just stopped hovering the widget this frame.
	pub fn exited(&self) -> bool {
		!self.hovered && self.hovered_previous
	}

	/// Returns `true` if the widget is being held down by the specified mouse button.
	pub fn held(&self, button: MouseButton) -> bool {
		self.button_state[&button].held
	}

	/// Returns `true` if the widget just started being held by the specified mouse
	/// button this frame.
	pub fn pressed(&self, button: MouseButton) -> bool {
		let button_state = self.button_state[&button];
		button_state.held && !button_state.held_previous
	}

	/// Returns `true` if the widget just stopped being held by the specified mouse
	/// button this frame.
	pub fn released(&self, button: MouseButton) -> bool {
		let button_state = self.button_state[&button];
		!button_state.held && button_state.held_previous
	}

	/// Returns `true` if the widget was clicked by the specified mouse button this
	/// frame. A "click" means the widget was released while hovered.
	pub fn clicked(&self, button: MouseButton) -> bool {
		self.button_state[&button].clicked
	}

	pub fn custom<T: 'static>(&self) -> Option<&T> {
		self.custom.as_ref().and_then(|inner| inner.downcast_ref())
	}

	pub fn custom_mut<T: 'static>(&mut self) -> Option<&mut T> {
		self.custom.as_mut().and_then(|inner| inner.downcast_mut())
	}

	pub fn custom_or_insert<T: 'static>(&mut self, value: T) -> &mut T {
		if self.custom_mut::<T>().is_none() {
			self.custom = Some(Box::new(value));
		}
		self.custom_mut().unwrap()
	}

	pub fn custom_or_insert_with<T: 'static>(&mut self, f: impl FnOnce() -> T) -> &mut T {
		if self.custom_mut::<T>().is_none() {
			self.custom = Some(Box::new(f()));
		}
		self.custom_mut().unwrap()
	}

	pub fn custom_or_insert_default<T: Default + 'static>(&mut self) -> &mut T {
		if self.custom_mut::<T>().is_none() {
			self.custom = Some(Box::new(T::default()));
		}
		self.custom_mut().unwrap()
	}

	pub(crate) fn update_mouse_state(&mut self, mouse_input: &MouseInput, size: Vec2) {
		self.relative_pos = mouse_input.position;
		self.delta = mouse_input.delta;
		let hovered = mouse_input
			.position
			.is_some_and(|position| Rect::new(Vec2::ZERO, size).contains_point(position));
		self.wheel_delta = if hovered {
			mouse_input.wheel_delta
		} else {
			Vec2::ZERO
		};
		self.hovered_previous = self.hovered;
		self.hovered = hovered;
		for (
			button,
			ButtonState {
				held,
				held_previous,
				clicked,
			},
		) in &mut self.button_state
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

impl Default for WidgetState {
	fn default() -> Self {
		Self::new()
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub(crate) struct ButtonState {
	pub(crate) held: bool,
	pub(crate) held_previous: bool,
	pub(crate) clicked: bool,
}
