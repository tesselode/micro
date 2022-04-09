use std::{collections::HashMap, hash::Hash};

use sdl2::{
	controller::{Axis, Button},
	keyboard::{Keycode, Scancode},
	mouse::MouseButton,
};

use crate::Context;

use super::GameController;

#[derive(Debug)]
pub struct VirtualController<C: VirtualControls, S: VirtualAnalogSticks<C>> {
	pub config: VirtualControllerConfig<C>,
	pub controller: Option<GameController>,
	active_input_kind: InputKind,
	control_state: HashMap<C, VirtualControlState>,
	stick_state: HashMap<S, VirtualAnalogStickState>,
}

impl<C: VirtualControls, S: VirtualAnalogSticks<C>> VirtualController<C, S> {
	pub fn new(config: VirtualControllerConfig<C>, controller: Option<GameController>) -> Self {
		Self {
			config,
			controller,
			active_input_kind: InputKind::KeyboardMouse,
			control_state: C::all()
				.iter()
				.map(|control| (*control, VirtualControlState::default()))
				.collect(),
			stick_state: S::all()
				.iter()
				.map(|stick| (*stick, VirtualAnalogStickState::default()))
				.collect(),
		}
	}

	pub fn update(&mut self, ctx: &Context) {}

	pub fn control(&self, control: C) -> VirtualControlState {
		todo!()
	}

	pub fn stick(&self, stick: S) -> VirtualAnalogStickState {
		todo!()
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct VirtualControlState;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct VirtualAnalogStickState;

#[derive(Debug, Clone)]
pub struct VirtualControllerConfig<C: VirtualControls> {
	pub controls: HashMap<C, RealControl>,
}

pub trait VirtualControls: Sized + Hash + Eq + Copy + 'static {
	fn all() -> &'static [Self];
}

pub trait VirtualAnalogSticks<C: VirtualControls>: Sized + Hash + Eq + Copy + 'static {
	fn all() -> &'static [Self];

	fn controls(&self) -> VirtualAnalogStickControls<C>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VirtualAnalogStickControls<C: VirtualControls> {
	pub left: C,
	pub right: C,
	pub up: C,
	pub down: C,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RealControl {
	Key(Scancode),
	MouseButton(MouseButton),
	GamepadButton(Button),
	GamepadAxis(Axis, AxisDirection),
}

impl RealControl {
	pub fn value(&self, ctx: &Context, controller: Option<&GameController>) -> f32 {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InputKind {
	KeyboardMouse,
	Gamepad,
}
