use exhaust::Exhaust;
use micro::{
	input::{Axis, Button, Scancode},
	math::CardinalDirection::{self, *},
};
use micro_virtual_controller::{
	AxisDirection, DeadzoneShape, VirtualAnalogSticks, VirtualControllerConfig, control_mapping,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Exhaust)]
pub enum Controls {
	Move(CardinalDirection),
	Primary,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Exhaust)]
pub enum Sticks {
	Move,
}

impl VirtualAnalogSticks<Controls> for Sticks {
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
				(Axis::LeftX, AxisDirection::Negative),
			],
			Controls::Move(Right) => [
				Scancode::Right,
				Button::DPadRight,
				(Axis::LeftX, AxisDirection::Positive),
			],
			Controls::Move(Up) => [
				Scancode::Up,
				Button::DPadUp,
				(Axis::LeftY, AxisDirection::Negative),
			],
			Controls::Move(Down) => [
				Scancode::Down,
				Button::DPadDown,
				(Axis::LeftY, AxisDirection::Positive),
			],
			Controls::Primary => [Scancode::X, Button::South],
		},
		deadzone: 1.0 / 3.0,
		deadzone_shape: DeadzoneShape::Circle,
	}
}
