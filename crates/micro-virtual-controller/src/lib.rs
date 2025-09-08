mod config;
mod real_control;
mod traits;

pub use config::*;
use exhaust::Exhaust;
pub use real_control::*;
pub use traits::*;

use std::{collections::HashMap, hash::Hash};

use micro::{
	Context,
	input::Gamepad,
	math::{CardinalDirection, Vec2},
};

#[derive(Debug)]
pub struct VirtualController<C, S = ()>
where
	C: Sized + Hash + Eq + Copy + Exhaust + 'static,
	S: VirtualAnalogSticks<C>,
{
	pub config: VirtualControllerConfig<C>,
	pub gamepad: Option<Gamepad>,
	active_input_kind: Option<InputKind>,
	control_state: HashMap<C, VirtualControlState>,
	stick_state: HashMap<S, VirtualAnalogStickState>,
}

impl<C, S> VirtualController<C, S>
where
	C: Sized + Hash + Eq + Copy + Exhaust + 'static,
	S: VirtualAnalogSticks<C>,
{
	pub fn new(config: VirtualControllerConfig<C>, gamepad: Option<Gamepad>) -> Self {
		Self {
			config,
			gamepad,
			active_input_kind: None,
			control_state: C::exhaust()
				.map(|control| (control, VirtualControlState::default()))
				.collect(),
			stick_state: S::exhaust()
				.map(|stick| (stick, VirtualAnalogStickState::default()))
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
						real_control.value(ctx, self.gamepad.as_ref()) > self.config.deadzone
					})
			})
	}

	fn update_control_state(&mut self, ctx: &Context, active_input_kind: InputKind) {
		for (control, state) in &mut self.control_state {
			let down_previous = state.down;
			let raw_value = Self::control_raw_value(
				ctx,
				&self.config,
				self.gamepad.as_ref(),
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
			let controls = stick.controls();
			let left = controls(CardinalDirection::Left);
			let right = controls(CardinalDirection::Right);
			let up = controls(CardinalDirection::Up);
			let down = controls(CardinalDirection::Down);
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
		ctx: &Context,
		config: &VirtualControllerConfig<C>,
		gamepad: Option<&Gamepad>,
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
					.fold(0.0, |previous, control: &RealControl| {
						previous + control.value(ctx, gamepad)
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

#[macro_export]
macro_rules! control_mapping {
	($($virtual:expr => [$($real:expr),*$(,)?]),*$(,)?) => {{
		let mut mapping = std::collections::HashMap::new();
		$(mapping.insert($virtual, vec![$($real.into()),*]);)*
		mapping
	}};
}
