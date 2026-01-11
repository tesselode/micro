//! Types related to input devices.

mod axis;
mod button;
mod mouse_button;
mod scancode;

use std::fmt::Debug;

pub use axis::Axis;
pub use button::Button;
pub use mouse_button::MouseButton;
pub use scancode::Scancode;
pub use sdl3::keyboard::Keycode;
pub use sdl3::mouse::MouseWheelDirection;

/// A game controller.
pub struct Gamepad(pub(crate) sdl3::gamepad::Gamepad);

impl Gamepad {
	/// Whether the gamepad is currently connected.
	pub fn is_connected(&self) -> bool {
		self.0.connected()
	}

	/// Returns `true` if the specified `button` is currently held down on
	/// this gamepad.
	pub fn is_button_down(&self, button: Button) -> bool {
		self.0.button(button.into())
	}

	/// Returns the current value of the specified `axis` on this gamepad.
	pub fn axis_value(&self, axis: Axis) -> f32 {
		self.0.axis(axis.into()) as f32 / i16::MAX as f32
	}
}

impl Debug for Gamepad {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_tuple("GameController").finish()
	}
}
