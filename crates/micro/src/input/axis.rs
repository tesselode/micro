/// An analog control on a gamepad.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serializing", derive(serde::Serialize, serde::Deserialize))]
pub enum Axis {
	/// The X axis of the left analog stick.
	LeftX,
	/// The Y axis of the left analog stick.
	LeftY,
	/// The X axis of the right analog stick.
	RightX,
	/// The Y axis of the right analog stick.
	RightY,
	/// The left trigger.
	TriggerLeft,
	/// The right trigger.
	TriggerRight,
}

impl Axis {
	pub(crate) fn from_gilrs_axis(gilrs_axis: gilrs::ev::Axis) -> Option<Self> {
		dbg!(gilrs_axis);
		match gilrs_axis {
			gilrs::ev::Axis::LeftStickX => Some(Axis::LeftX),
			gilrs::ev::Axis::LeftStickY => Some(Axis::LeftY),
			gilrs::ev::Axis::RightStickX => Some(Axis::RightX),
			gilrs::ev::Axis::RightStickY => Some(Axis::RightY),
			gilrs::ev::Axis::LeftZ => Some(Axis::TriggerLeft),
			gilrs::ev::Axis::RightZ => Some(Axis::TriggerRight),
			_ => None,
		}
	}
}

impl From<Axis> for gilrs::ev::Axis {
	fn from(value: Axis) -> Self {
		match value {
			Axis::LeftX => gilrs::ev::Axis::LeftStickX,
			Axis::LeftY => gilrs::ev::Axis::LeftStickY,
			Axis::RightX => gilrs::ev::Axis::RightStickX,
			Axis::RightY => gilrs::ev::Axis::RightStickY,
			Axis::TriggerLeft => gilrs::ev::Axis::LeftZ,
			Axis::TriggerRight => gilrs::ev::Axis::RightZ,
		}
	}
}
