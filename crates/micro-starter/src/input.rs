use std::collections::HashMap;

use micro::{
	control_mapping,
	input::{
		virtual_controller::{
			AxisDirection, DeadzoneShape, VirtualAnalogSticks, VirtualControllerConfig,
			VirtualControls,
		},
		Axis, Button, Scancode,
	},
	math::CardinalDirection::{self, *},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Controls {
	Move(CardinalDirection),
	Primary,
}

impl VirtualControls for Controls {
	const ALL: &'static [Self] = &[
		Self::Move(Left),
		Self::Move(Right),
		Self::Move(Up),
		Self::Move(Down),
		Self::Primary,
	];
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Sticks {
	Move,
}

impl VirtualAnalogSticks<Controls> for Sticks {
	const ALL: &'static [Self] = &[Self::Move];

	fn controls(&self) -> fn(CardinalDirection) -> Controls {
		match self {
			Sticks::Move => Controls::Move,
		}
	}
}

pub fn default_input_config() -> VirtualControllerConfig<Controls> {
	VirtualControllerConfig {
		control_mapping: control_mapping! {
			Controls::Move(Left) => [
				Scancode::Left,
				Button::DPadLeft,
				(Axis::LeftX, AxisDirection::Negative)
			],
			Controls::Move(Right) => [
				Scancode::Right,
				Button::DPadRight,
				(Axis::LeftX, AxisDirection::Positive)
			],
			Controls::Move(Up) => [
				Scancode::Up,
				Button::DPadUp,
				(Axis::LeftY, AxisDirection::Negative)
			],
			Controls::Move(Down) => [
				Scancode::Down,
				Button::DPadDown,
				(Axis::LeftY, AxisDirection::Positive)
			],
			Controls::Primary => [Scancode::X, Button::A]
		},
		deadzone: 1.0 / 3.0,
		deadzone_shape: DeadzoneShape::Circle,
	}
}
