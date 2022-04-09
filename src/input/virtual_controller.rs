use std::{collections::HashMap, hash::Hash};

use sdl2::{
	controller::{Axis, Button},
	keyboard::Scancode,
	mouse::MouseButton,
};
use vek::Vec2;

use crate::Context;

use super::GameController;

#[derive(Debug)]
pub struct VirtualController<C: VirtualControls, S: VirtualAnalogSticks<C> = ()> {
	pub config: VirtualControllerConfig<C>,
	pub controller: Option<GameController>,
	active_input_kind: Option<InputKind>,
	control_state: HashMap<C, VirtualControlState>,
	stick_state: HashMap<S, VirtualAnalogStickState>,
}

impl<C: VirtualControls, S: VirtualAnalogSticks<C>> VirtualController<C, S> {
	pub fn new(config: VirtualControllerConfig<C>, controller: Option<GameController>) -> Self {
		Self {
			config,
			controller,
			active_input_kind: None,
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

	pub fn update(&mut self, ctx: &Context) {
		self.update_active_input_kind(ctx);
		if let Some(active_input_kind) = self.active_input_kind {
			self.update_control_state(ctx, active_input_kind);
		}
		self.update_stick_state();
	}

	pub fn control(&self, control: C) -> VirtualControlState {
		self.control_state[&control]
	}

	pub fn stick(&self, stick: S) -> VirtualAnalogStickState {
		self.stick_state[&stick]
	}

	pub fn active_input_kind(&self) -> Option<InputKind> {
		self.active_input_kind
	}

	fn update_active_input_kind(&mut self, ctx: &Context) {
		if self.any_input_of_kind_used(ctx, InputKind::KeyboardMouse) {
			self.active_input_kind = Some(InputKind::KeyboardMouse);
		} else if self.any_input_of_kind_used(ctx, InputKind::Gamepad) {
			self.active_input_kind = Some(InputKind::Gamepad);
		}
	}

	fn any_input_of_kind_used(&self, ctx: &Context, kind: InputKind) -> bool {
		self.config
			.control_mapping
			.iter()
			.any(|(_, real_controls)| {
				real_controls
					.iter()
					.filter(|real_control| real_control.kind() == kind)
					.any(|real_control| {
						real_control.value(ctx, self.controller.as_ref()) > self.config.deadzone
					})
			})
	}

	fn update_control_state(&mut self, ctx: &Context, active_input_kind: InputKind) {
		for (control, state) in &mut self.control_state {
			let down_previous = state.down;
			let raw_value = Self::control_raw_value(
				&self.config,
				self.controller.as_ref(),
				ctx,
				*control,
				active_input_kind,
			);
			let value = if raw_value >= self.config.deadzone {
				raw_value
			} else {
				0.0
			};
			let down = value > 0.0;
			let pressed = down && !down_previous;
			let released = down_previous && !down;
			*state = VirtualControlState {
				value,
				raw_value,
				down,
				pressed,
				released,
			};
		}
	}

	fn update_stick_state(&mut self) {
		for (stick, VirtualAnalogStickState { value, raw_value }) in &mut self.stick_state {
			let VirtualAnalogStickControls {
				left,
				right,
				up,
				down,
			} = stick.controls();
			*raw_value = Vec2 {
				x: self.control_state[&right].raw_value - self.control_state[&left].raw_value,
				y: self.control_state[&down].raw_value - self.control_state[&up].raw_value,
			};
			if raw_value.magnitude_squared() > 1.0 {
				raw_value.normalize();
			}
			*value = self
				.config
				.deadzone_shape
				.apply_deadzone(*raw_value, self.config.deadzone);
		}
	}

	fn control_raw_value(
		config: &VirtualControllerConfig<C>,
		controller: Option<&GameController>,
		ctx: &Context,
		control: C,
		active_input_kind: InputKind,
	) -> f32 {
		config
			.control_mapping
			.get(&control)
			.map(|controls| {
				controls
					.iter()
					.filter(|control| control.kind() == active_input_kind)
					.fold(0.0, |previous, control| {
						previous + control.value(ctx, controller)
					})
					.min(1.0)
			})
			.unwrap_or(0.0)
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct VirtualControlState {
	pub value: f32,
	pub raw_value: f32,
	pub down: bool,
	pub pressed: bool,
	pub released: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct VirtualAnalogStickState {
	pub value: Vec2<f32>,
	pub raw_value: Vec2<f32>,
}

#[derive(Debug, Clone, Default)]
pub struct VirtualControllerConfig<C: VirtualControls> {
	pub control_mapping: HashMap<C, Vec<RealControl>>,
	pub deadzone: f32,
	pub deadzone_shape: DeadzoneShape,
}

pub trait VirtualControls: Sized + Hash + Eq + Copy + 'static {
	fn all() -> &'static [Self];
}

pub trait VirtualAnalogSticks<C: VirtualControls>: Sized + Hash + Eq + Copy + 'static {
	fn all() -> &'static [Self];

	fn controls(&self) -> VirtualAnalogStickControls<C>;
}

impl<C: VirtualControls> VirtualAnalogSticks<C> for () {
	fn all() -> &'static [Self] {
		&[]
	}

	fn controls(&self) -> VirtualAnalogStickControls<C> {
		unreachable!()
	}
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
	fn kind(&self) -> InputKind {
		match self {
			RealControl::Key(_) => InputKind::KeyboardMouse,
			RealControl::MouseButton(_) => InputKind::KeyboardMouse,
			RealControl::GamepadButton(_) => InputKind::Gamepad,
			RealControl::GamepadAxis(_, _) => InputKind::Gamepad,
		}
	}

	fn value(&self, ctx: &Context, controller: Option<&GameController>) -> f32 {
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
pub enum DeadzoneShape {
	Circle,
	Square,
}

impl DeadzoneShape {
	fn apply_deadzone(&self, input: Vec2<f32>, deadzone: f32) -> Vec2<f32> {
		match self {
			DeadzoneShape::Circle => {
				if input.magnitude() >= deadzone {
					input
				} else {
					Vec2::zero()
				}
			}
			DeadzoneShape::Square => Vec2 {
				x: if input.x.abs() >= deadzone {
					input.x
				} else {
					0.0
				},
				y: if input.y.abs() >= deadzone {
					input.y
				} else {
					0.0
				},
			},
		}
	}
}

impl Default for DeadzoneShape {
	fn default() -> Self {
		Self::Circle
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InputKind {
	KeyboardMouse,
	Gamepad,
}
