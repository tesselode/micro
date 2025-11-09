use sdl3::gamepad::Button as sdl3Button;

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

impl From<sdl3Button> for Button {
	fn from(value: sdl3Button) -> Self {
		match value {
			sdl3Button::South => Button::South,
			sdl3Button::East => Button::East,
			sdl3Button::West => Button::West,
			sdl3Button::North => Button::North,
			sdl3Button::Back => Button::Back,
			sdl3Button::Guide => Button::Guide,
			sdl3Button::Start => Button::Start,
			sdl3Button::LeftStick => Button::LeftStick,
			sdl3Button::RightStick => Button::RightStick,
			sdl3Button::LeftShoulder => Button::LeftShoulder,
			sdl3Button::RightShoulder => Button::RightShoulder,
			sdl3Button::DPadUp => Button::DPadUp,
			sdl3Button::DPadDown => Button::DPadDown,
			sdl3Button::DPadLeft => Button::DPadLeft,
			sdl3Button::DPadRight => Button::DPadRight,
			sdl3Button::LeftPaddle1 => Button::LeftPaddle1,
			sdl3Button::LeftPaddle2 => Button::LeftPaddle2,
			sdl3Button::RightPaddle1 => Button::RightPaddle1,
			sdl3Button::RightPaddle2 => Button::RightPaddle2,
			sdl3Button::Touchpad => Button::Touchpad,
			sdl3Button::Misc1 => Button::Misc1,
			sdl3Button::Misc2 => Button::Misc2,
			sdl3Button::Misc3 => Button::Misc3,
			sdl3Button::Misc4 => Button::Misc4,
			sdl3Button::Misc5 => Button::Misc5,
		}
	}
}

impl From<Button> for sdl3Button {
	fn from(value: Button) -> Self {
		match value {
			Button::South => sdl3Button::South,
			Button::East => sdl3Button::East,
			Button::West => sdl3Button::West,
			Button::North => sdl3Button::North,
			Button::Back => sdl3Button::Back,
			Button::Guide => sdl3Button::Guide,
			Button::Start => sdl3Button::Start,
			Button::LeftStick => sdl3Button::LeftStick,
			Button::RightStick => sdl3Button::RightStick,
			Button::LeftShoulder => sdl3Button::LeftShoulder,
			Button::RightShoulder => sdl3Button::RightShoulder,
			Button::DPadUp => sdl3Button::DPadUp,
			Button::DPadDown => sdl3Button::DPadDown,
			Button::DPadLeft => sdl3Button::DPadLeft,
			Button::DPadRight => sdl3Button::DPadRight,
			Button::LeftPaddle1 => sdl3Button::LeftPaddle1,
			Button::LeftPaddle2 => sdl3Button::LeftPaddle2,
			Button::RightPaddle1 => sdl3Button::RightPaddle1,
			Button::RightPaddle2 => sdl3Button::RightPaddle2,
			Button::Touchpad => sdl3Button::Touchpad,
			Button::Misc1 => sdl3Button::Misc1,
			Button::Misc2 => sdl3Button::Misc2,
			Button::Misc3 => sdl3Button::Misc3,
			Button::Misc4 => sdl3Button::Misc4,
			Button::Misc5 => sdl3Button::Misc5,
		}
	}
}

impl Button {
	/// Return a string for the button in the same format used by game controller mapping strings.
	pub fn string(self) -> String {
		sdl3Button::from(self).string()
	}
}
