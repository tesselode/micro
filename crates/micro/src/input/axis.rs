use sdl3::gamepad::Axis as sdl3Axis;

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

impl From<sdl3Axis> for Axis {
	fn from(value: sdl3Axis) -> Self {
		match value {
			sdl3Axis::LeftX => Axis::LeftX,
			sdl3Axis::LeftY => Axis::LeftY,
			sdl3Axis::RightX => Axis::RightX,
			sdl3Axis::RightY => Axis::RightY,
			sdl3Axis::TriggerLeft => Axis::TriggerLeft,
			sdl3Axis::TriggerRight => Axis::TriggerRight,
		}
	}
}

impl From<Axis> for sdl3Axis {
	fn from(value: Axis) -> Self {
		match value {
			Axis::LeftX => sdl3Axis::LeftX,
			Axis::LeftY => sdl3Axis::LeftY,
			Axis::RightX => sdl3Axis::RightX,
			Axis::RightY => sdl3Axis::RightY,
			Axis::TriggerLeft => sdl3Axis::TriggerLeft,
			Axis::TriggerRight => sdl3Axis::TriggerRight,
		}
	}
}

impl Axis {
	/// Return a string for the axis in the same format used by game controller mapping strings.
	pub fn string(self) -> String {
		sdl3Axis::from(self).string()
	}
}
