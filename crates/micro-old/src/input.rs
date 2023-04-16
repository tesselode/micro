mod axis;
mod button;
mod keycode;
mod mouse_button;
mod scancode;
pub mod virtual_controller;

use std::fmt::Debug;

pub use axis::Axis;
pub use button::Button;
pub use keycode::Keycode;
pub use mouse_button::MouseButton;
pub use scancode::Scancode;
pub use sdl2::mouse::MouseWheelDirection;

pub struct Gamepad(pub(crate) sdl2::controller::GameController);

impl Gamepad {
	pub fn is_attached(&self) -> bool {
		self.0.attached()
	}

	pub fn is_button_down(&self, button: Button) -> bool {
		self.0.button(button.into())
	}

	pub fn axis_value(&self, axis: Axis) -> f32 {
		self.0.axis(axis.into()) as f32 / i16::MAX as f32
	}
}

impl Debug for Gamepad {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_tuple("GameController").finish()
	}
}
