mod config;
mod real_control;
mod traits;

pub use config::*;
use glam::Vec2;
pub use real_control::*;
pub use traits::*;

use std::{collections::HashMap, hash::Hash};

#[derive(Debug)]
pub struct VirtualController<C: VirtualControls, S: VirtualAnalogSticks<C> = ()> {
	pub config: VirtualControllerConfig<C>,
	pub gamepad_index: Option<u32>,
	active_input_kind: Option<InputKind>,
	control_state: HashMap<C, VirtualControlState>,
	stick_state: HashMap<S, VirtualAnalogStickState>,
}

impl<C: VirtualControls, S: VirtualAnalogSticks<C>> VirtualController<C, S> {
	pub fn new(config: VirtualControllerConfig<C>, gamepad_index: Option<u32>) -> Self {
		Self {
			config,
			gamepad_index,
			active_input_kind: None,
			control_state: C::ALL
				.iter()
				.map(|control| (*control, VirtualControlState::default()))
				.collect(),
			stick_state: S::ALL
				.iter()
				.map(|stick| (*stick, VirtualAnalogStickState::default()))
				.collect(),
		}
	}

	pub fn update(&mut self) {
		self.update_active_input_kind();
		if let Some(active_input_kind) = self.active_input_kind {
			self.update_control_state(active_input_kind);
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

	fn update_active_input_kind(&mut self) {
		if self.any_input_of_kind_used(InputKind::KeyboardMouse) {
			self.active_input_kind = Some(InputKind::KeyboardMouse);
		} else if self.any_input_of_kind_used(InputKind::Gamepad) {
			self.active_input_kind = Some(InputKind::Gamepad);
		}
	}

	fn any_input_of_kind_used(&self, kind: InputKind) -> bool {
		self.config
			.control_mapping
			.iter()
			.any(|(_, real_controls)| {
				real_controls
					.iter()
					.filter(|real_control| real_control.kind() == kind)
					.any(|real_control| {
						real_control.value(self.gamepad_index) > self.config.deadzone
					})
			})
	}

	fn update_control_state(&mut self, active_input_kind: InputKind) {
		for (control, state) in &mut self.control_state {
			let down_previous = state.down;
			let raw_value = Self::control_raw_value(
				&self.config,
				self.gamepad_index,
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
			if raw_value.length_squared() > 1.0 {
				*raw_value = raw_value.normalize();
			}
			*value = self
				.config
				.deadzone_shape
				.apply_deadzone(*raw_value, self.config.deadzone);
		}
	}

	fn control_raw_value(
		config: &VirtualControllerConfig<C>,
		gamepad_index: Option<u32>,
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
						previous + control.value(gamepad_index)
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
	pub value: Vec2,
	pub raw_value: Vec2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InputKind {
	KeyboardMouse,
	Gamepad,
}
