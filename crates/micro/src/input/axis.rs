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
