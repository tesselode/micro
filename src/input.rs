pub mod virtual_controller;

use std::fmt::Debug;

pub use sdl2::{
	controller::{Axis, Button},
	keyboard::{Keycode, Scancode},
	mouse::{MouseButton, MouseWheelDirection},
};

pub struct Gamepad(pub(crate) sdl2::controller::GameController);

impl Gamepad {
	pub fn is_attached(&self) -> bool {
		self.0.attached()
	}

	pub fn is_button_down(&self, button: Button) -> bool {
		self.0.button(button)
	}

	pub fn axis_value(&self, axis: Axis) -> f32 {
		self.0.axis(axis) as f32 / i16::MAX as f32
	}
}

impl Debug for Gamepad {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_tuple("GameController").finish()
	}
}
