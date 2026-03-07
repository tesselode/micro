use std::{fmt::Debug, time::Duration};

use crate::input::{Axis, Button};

/// A game controller.
pub struct Gamepad {
	pub(crate) id: GamepadId,
	pub(crate) gamepad: sdl3::gamepad::Gamepad,
}

impl Gamepad {
	pub fn id(&self) -> GamepadId {
		self.id
	}

	/// Whether the gamepad is currently connected.
	pub fn is_connected(&self) -> bool {
		self.gamepad.connected()
	}

	/// Returns `true` if the specified `button` is currently held down on
	/// this gamepad.
	pub fn is_button_down(&self, button: Button) -> bool {
		self.gamepad.button(button.into())
	}

	/// Returns the current value of the specified `axis` on this gamepad.
	pub fn axis_value(&self, axis: Axis) -> f32 {
		self.gamepad.axis(axis.into()) as f32 / i16::MAX as f32
	}

	pub fn set_rumble(
		&mut self,
		low_frequency: f32,
		high_frequency: f32,
		duration: Duration,
	) -> Result<(), sdl3::IntegerOrSdlError> {
		self.gamepad.set_rumble(
			(low_frequency * u16::MAX as f32) as u16,
			(high_frequency * u16::MAX as f32) as u16,
			duration.as_millis() as u32,
		)
	}

	pub fn set_trigger_rumble(
		&mut self,
		left: f32,
		right: f32,
		duration: Duration,
	) -> Result<(), sdl3::IntegerOrSdlError> {
		self.gamepad.set_rumble_triggers(
			(left * u16::MAX as f32) as u16,
			(right * u16::MAX as f32) as u16,
			duration.as_millis() as u32,
		)
	}
}

impl Debug for Gamepad {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("Gamepad").field("id", &self.id).finish()
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GamepadId(pub(crate) u32);

impl From<sdl3::sys::joystick::SDL_JoystickID> for GamepadId {
	fn from(value: sdl3::sys::joystick::SDL_JoystickID) -> Self {
		Self(value.0)
	}
}

impl From<GamepadId> for sdl3::sys::joystick::SDL_JoystickID {
	fn from(value: GamepadId) -> Self {
		sdl3::sys::joystick::SDL_JoystickID(value.0)
	}
}
