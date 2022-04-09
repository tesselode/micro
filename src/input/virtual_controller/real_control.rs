use sdl2::{
	controller::{Axis, Button},
	keyboard::Scancode,
	mouse::MouseButton,
};

use crate::{input::GameController, Context};

use super::InputKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

	pub(super) fn value(&self, ctx: &Context, controller: Option<&GameController>) -> f32 {
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
				let controller = match controller {
					Some(controller) => controller,
					None => return 0.0,
				};
				if controller.is_button_down(*button) {
					1.0
				} else {
					0.0
				}
			}
			RealControl::GamepadAxis(axis, direction) => {
				let controller = match controller {
					Some(controller) => controller,
					None => return 0.0,
				};
				(controller.axis_value(*axis) * direction.as_f32()).max(0.0)
			}
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
