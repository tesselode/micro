use crate::{
	context::Context,
	input::{Axis, Button, Gamepad, MouseButton, Scancode},
};

use super::InputKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serializing", derive(serde::Serialize, serde::Deserialize))]
pub enum RealControl {
	Key(Scancode),
	MouseButton(MouseButton),
	GamepadButton(Button),
	GamepadAxis(Axis, AxisDirection),
}

impl RealControl {
	pub(super) fn kind(&self) -> InputKind {
		match self {
			RealControl::Key(_) => InputKind::KeyboardMouse,
			RealControl::MouseButton(_) => InputKind::KeyboardMouse,
			RealControl::GamepadButton(_) => InputKind::Gamepad,
			RealControl::GamepadAxis(_, _) => InputKind::Gamepad,
		}
	}

	pub(super) fn value(&self, ctx: &Context, gamepad: Option<&Gamepad>) -> f32 {
		match self {
			RealControl::Key(scancode) => {
				if ctx.is_key_down(*scancode) {
					1.0
				} else {
					0.0
				}
			}
			RealControl::MouseButton(mouse_button) => {
				if ctx.is_mouse_button_down(*mouse_button) {
					1.0
				} else {
					0.0
				}
			}
			RealControl::GamepadButton(button) => {
				let gamepad = match gamepad {
					Some(gamepad) => gamepad,
					None => return 0.0,
				};
				if gamepad.is_button_down(*button) {
					1.0
				} else {
					0.0
				}
			}
			RealControl::GamepadAxis(axis, direction) => {
				let gamepad = match gamepad {
					Some(gamepad) => gamepad,
					None => return 0.0,
				};
				(gamepad.axis_value(*axis) * direction.as_f32()).max(0.0)
			}
		}
	}
}

impl From<Scancode> for RealControl {
	fn from(v: Scancode) -> Self {
		Self::Key(v)
	}
}

impl From<MouseButton> for RealControl {
	fn from(v: MouseButton) -> Self {
		Self::MouseButton(v)
	}
}

impl From<Button> for RealControl {
	fn from(v: Button) -> Self {
		Self::GamepadButton(v)
	}
}

impl From<(Axis, AxisDirection)> for RealControl {
	fn from((axis, direction): (Axis, AxisDirection)) -> Self {
		Self::GamepadAxis(axis, direction)
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serializing", derive(serde::Serialize, serde::Deserialize))]
pub enum AxisDirection {
	Negative,
	Positive,
}

impl AxisDirection {
	fn as_f32(&self) -> f32 {
		match self {
			AxisDirection::Negative => -1.0,
			AxisDirection::Positive => 1.0,
		}
	}
}
