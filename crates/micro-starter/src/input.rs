use std::collections::HashMap;

use micro::input::{
	virtual_controller::{
		AxisDirection, DeadzoneShape, RealControl, VirtualAnalogStickControls, VirtualAnalogSticks,
		VirtualControllerConfig, VirtualControls,
	},
	Axis, Button, Scancode,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Controls {
	Left,
	Right,
	Up,
	Down,
}

impl VirtualControls for Controls {
	const ALL: &'static [Self] = &[Self::Left, Self::Right, Self::Up, Self::Down];
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
				left: Controls::Left,
				right: Controls::Right,
				up: Controls::Up,
				down: Controls::Down,
			},
		}
	}
}

pub fn default_input_config() -> VirtualControllerConfig<Controls> {
	VirtualControllerConfig {
		control_mapping: {
			let mut mapping = HashMap::new();
			mapping.insert(
				Controls::Left,
				vec![
					RealControl::Key(Scancode::Left),
					RealControl::GamepadButton(Button::DPadLeft),
					RealControl::GamepadAxis(Axis::LeftX, AxisDirection::Negative),
				],
			);
			mapping.insert(
				Controls::Right,
				vec![
					RealControl::Key(Scancode::Right),
					RealControl::GamepadButton(Button::DPadRight),
					RealControl::GamepadAxis(Axis::LeftX, AxisDirection::Positive),
				],
			);
			mapping.insert(
				Controls::Up,
				vec![
					RealControl::Key(Scancode::Up),
					RealControl::GamepadButton(Button::DPadUp),
					RealControl::GamepadAxis(Axis::LeftY, AxisDirection::Negative),
				],
			);
			mapping.insert(
				Controls::Down,
				vec![
					RealControl::Key(Scancode::Down),
					RealControl::GamepadButton(Button::DPadDown),
					RealControl::GamepadAxis(Axis::LeftY, AxisDirection::Positive),
				],
			);
			mapping
		},
		deadzone: 1.0 / 3.0,
		deadzone_shape: DeadzoneShape::Circle,
	}
}
