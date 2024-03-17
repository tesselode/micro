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
	macro_rules! control_mapping {
		($($virtual:expr => [$($real:expr),*]),*) => {{
			let mut mapping = HashMap::new();
			$(mapping.insert($virtual, vec![$($real.into()),*]);)*
			mapping
		}};
	}

	VirtualControllerConfig {
		control_mapping: control_mapping! {
			Controls::Move(CardinalDirection::Left) => [
				Scancode::Left,
				Button::DPadLeft,
				(Axis::LeftX, AxisDirection::Negative)
			],
			Controls::Move(CardinalDirection::Right) => [
				Scancode::Right,
				Button::DPadRight,
				(Axis::LeftX, AxisDirection::Positive)
			],
			Controls::Move(CardinalDirection::Up) => [
				Scancode::Up,
				Button::DPadUp,
				(Axis::LeftY, AxisDirection::Negative)
			],
			Controls::Move(CardinalDirection::Down) => [
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
