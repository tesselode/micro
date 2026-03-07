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
}

impl Button {
	pub(crate) fn from_gilrs_button(gilrs_button: gilrs::ev::Button) -> Option<Self> {
		match gilrs_button {
			gilrs::ev::Button::South => Some(Button::South),
			gilrs::ev::Button::East => Some(Button::East),
			gilrs::ev::Button::West => Some(Button::West),
			gilrs::ev::Button::North => Some(Button::North),
			gilrs::ev::Button::Select => Some(Button::Back),
			gilrs::ev::Button::Mode => Some(Button::Guide),
			gilrs::ev::Button::Start => Some(Button::Start),
			gilrs::ev::Button::LeftThumb => Some(Button::LeftStick),
			gilrs::ev::Button::RightThumb => Some(Button::RightStick),
			gilrs::ev::Button::LeftTrigger => Some(Button::LeftShoulder),
			gilrs::ev::Button::RightTrigger => Some(Button::RightShoulder),
			gilrs::ev::Button::DPadUp => Some(Button::DPadUp),
			gilrs::ev::Button::DPadDown => Some(Button::DPadDown),
			gilrs::ev::Button::DPadLeft => Some(Button::DPadLeft),
			gilrs::ev::Button::DPadRight => Some(Button::DPadRight),
			_ => None,
		}
	}
}

impl From<Button> for gilrs::ev::Button {
	fn from(value: Button) -> Self {
		match value {
			Button::South => gilrs::ev::Button::South,
			Button::East => gilrs::ev::Button::East,
			Button::West => gilrs::ev::Button::West,
			Button::North => gilrs::ev::Button::North,
			Button::Back => gilrs::ev::Button::Select,
			Button::Guide => gilrs::ev::Button::Mode,
			Button::Start => gilrs::ev::Button::Start,
			Button::LeftStick => gilrs::ev::Button::LeftThumb,
			Button::RightStick => gilrs::ev::Button::RightThumb,
			Button::LeftShoulder => gilrs::ev::Button::LeftTrigger,
			Button::RightShoulder => gilrs::ev::Button::RightTrigger,
			Button::DPadUp => gilrs::ev::Button::DPadUp,
			Button::DPadDown => gilrs::ev::Button::DPadDown,
			Button::DPadLeft => gilrs::ev::Button::DPadLeft,
			Button::DPadRight => gilrs::ev::Button::DPadRight,
		}
	}
}
