use sdl2::controller::Button as Sdl2Button;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Button {
	A,
	B,
	X,
	Y,
	Back,
	Guide,
	Start,
	LeftStick,
	RightStick,
	LeftShoulder,
	RightShoulder,
	DPadUp,
	DPadDown,
	DPadLeft,
	DPadRight,
	Misc1,
	Paddle1,
	Paddle2,
	Paddle3,
	Paddle4,
	Touchpad,
}

impl From<Sdl2Button> for Button {
	fn from(value: Sdl2Button) -> Self {
		match value {
			Sdl2Button::A => Button::A,
			Sdl2Button::B => Button::B,
			Sdl2Button::X => Button::X,
			Sdl2Button::Y => Button::Y,
			Sdl2Button::Back => Button::Back,
			Sdl2Button::Guide => Button::Guide,
			Sdl2Button::Start => Button::Start,
			Sdl2Button::LeftStick => Button::LeftStick,
			Sdl2Button::RightStick => Button::RightStick,
			Sdl2Button::LeftShoulder => Button::LeftShoulder,
			Sdl2Button::RightShoulder => Button::RightShoulder,
			Sdl2Button::DPadUp => Button::DPadUp,
			Sdl2Button::DPadDown => Button::DPadDown,
			Sdl2Button::DPadLeft => Button::DPadLeft,
			Sdl2Button::DPadRight => Button::DPadRight,
			Sdl2Button::Misc1 => Button::Misc1,
			Sdl2Button::Paddle1 => Button::Paddle1,
			Sdl2Button::Paddle2 => Button::Paddle2,
			Sdl2Button::Paddle3 => Button::Paddle3,
			Sdl2Button::Paddle4 => Button::Paddle4,
			Sdl2Button::Touchpad => Button::Touchpad,
		}
	}
}

impl From<Button> for Sdl2Button {
	fn from(value: Button) -> Self {
		match value {
			Button::A => Sdl2Button::A,
			Button::B => Sdl2Button::B,
			Button::X => Sdl2Button::X,
			Button::Y => Sdl2Button::Y,
			Button::Back => Sdl2Button::Back,
			Button::Guide => Sdl2Button::Guide,
			Button::Start => Sdl2Button::Start,
			Button::LeftStick => Sdl2Button::LeftStick,
			Button::RightStick => Sdl2Button::RightStick,
			Button::LeftShoulder => Sdl2Button::LeftShoulder,
			Button::RightShoulder => Sdl2Button::RightShoulder,
			Button::DPadUp => Sdl2Button::DPadUp,
			Button::DPadDown => Sdl2Button::DPadDown,
			Button::DPadLeft => Sdl2Button::DPadLeft,
			Button::DPadRight => Sdl2Button::DPadRight,
			Button::Misc1 => Sdl2Button::Misc1,
			Button::Paddle1 => Sdl2Button::Paddle1,
			Button::Paddle2 => Sdl2Button::Paddle2,
			Button::Paddle3 => Sdl2Button::Paddle3,
			Button::Paddle4 => Sdl2Button::Paddle4,
			Button::Touchpad => Sdl2Button::Touchpad,
		}
	}
}

impl Button {
	pub fn string(self) -> String {
		Sdl2Button::from(self).string()
	}
}
