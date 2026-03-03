/// A gamepad button.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serializing", derive(serde::Serialize, serde::Deserialize))]
#[allow(missing_docs)]
pub enum Button {
	/// The south face button (A on an Xbox controller).
	South,
	/// The east face button (B on an Xbox controller).
	East,
	/// The west face button (X on an Xbox controller).
	West,
	/// The north face button (Y on an Xbox controller).
	North,
	Back,
	Guide,
	Start,
	/// The button pressed by clicking the left stick in.
	LeftStick,
	/// The button pressed by clicking the right stick in.
	RightStick,
	LeftShoulder,
	RightShoulder,
	DPadUp,
	DPadDown,
	DPadLeft,
	DPadRight,
	LeftPaddle1,
	LeftPaddle2,
	RightPaddle1,
	RightPaddle2,
	Touchpad,
	Misc1,
	Misc2,
	Misc3,
	Misc4,
	Misc5,
}
