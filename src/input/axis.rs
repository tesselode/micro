use sdl2::controller::Axis as Sdl2Axis;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Axis {
	LeftX,
	LeftY,
	RightX,
	RightY,
	TriggerLeft,
	TriggerRight,
}

impl From<Sdl2Axis> for Axis {
	fn from(value: Sdl2Axis) -> Self {
		match value {
			Sdl2Axis::LeftX => Axis::LeftX,
			Sdl2Axis::LeftY => Axis::LeftY,
			Sdl2Axis::RightX => Axis::RightX,
			Sdl2Axis::RightY => Axis::RightY,
			Sdl2Axis::TriggerLeft => Axis::TriggerLeft,
			Sdl2Axis::TriggerRight => Axis::TriggerRight,
		}
	}
}

impl From<Axis> for Sdl2Axis {
	fn from(value: Axis) -> Self {
		match value {
			Axis::LeftX => Sdl2Axis::LeftX,
			Axis::LeftY => Sdl2Axis::LeftY,
			Axis::RightX => Sdl2Axis::RightX,
			Axis::RightY => Sdl2Axis::RightY,
			Axis::TriggerLeft => Sdl2Axis::TriggerLeft,
			Axis::TriggerRight => Sdl2Axis::TriggerRight,
		}
	}
}
