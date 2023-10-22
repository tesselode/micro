use std::collections::HashMap;

use micro::{
	input::{
		virtual_controller::{
			AxisDirection, DeadzoneShape, RealControl, VirtualAnalogStickControls,
			VirtualAnalogSticks, VirtualControllerConfig, VirtualControls,
		},
		Axis, Button, Scancode,
	},
	math::CardinalDirection,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Controls {
	Move(CardinalDirection),
	Primary,
}

impl VirtualControls for Controls {
	const ALL: &'static [Self] = &[
		Self::Move(CardinalDirection::Left),
		Self::Move(CardinalDirection::Right),
		Self::Move(CardinalDirection::Up),
		Self::Move(CardinalDirection::Down),
		Self::Primary,
	];
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Sticks {
	Move,
}

impl VirtualAnalogSticks<Controls> for Sticks {
	const ALL: &'static [Self] = &[Self::Move];

	fn controls(&self) -> VirtualAnalogStickControls<Controls> {
		match self {
			Sticks::Move => VirtualAnalogStickControls {
				left: Controls::Move(CardinalDirection::Left),
				right: Controls::Move(CardinalDirection::Right),
				up: Controls::Move(CardinalDirection::Up),
				down: Controls::Move(CardinalDirection::Down),
			},
		}
	}
}

pub fn default_input_config() -> VirtualControllerConfig<Controls> {
	VirtualControllerConfig {
		control_mapping: {
			let mut mapping = HashMap::new();
			mapping.insert(
				Controls::Move(CardinalDirection::Left),
				vec![
					RealControl::Key(Scancode::Left),
					RealControl::GamepadButton(Button::DPadLeft),
					RealControl::GamepadAxis(Axis::LeftX, AxisDirection::Negative),
				],
			);
			mapping.insert(
				Controls::Move(CardinalDirection::Right),
				vec![
					RealControl::Key(Scancode::Right),
					RealControl::GamepadButton(Button::DPadRight),
					RealControl::GamepadAxis(Axis::LeftX, AxisDirection::Positive),
				],
			);
			mapping.insert(
				Controls::Move(CardinalDirection::Up),
				vec![
					RealControl::Key(Scancode::Up),
					RealControl::GamepadButton(Button::DPadUp),
					RealControl::GamepadAxis(Axis::LeftY, AxisDirection::Negative),
				],
			);
			mapping.insert(
				Controls::Move(CardinalDirection::Down),
				vec![
					RealControl::Key(Scancode::Down),
					RealControl::GamepadButton(Button::DPadDown),
					RealControl::GamepadAxis(Axis::LeftY, AxisDirection::Positive),
				],
			);
			mapping.insert(
				Controls::Primary,
				vec![
					RealControl::Key(Scancode::X),
					RealControl::GamepadButton(Button::A),
				],
			);
			mapping
		},
		deadzone: 1.0 / 3.0,
		deadzone_shape: DeadzoneShape::Circle,
	}
}
