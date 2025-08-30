use sdl3::mouse::MouseButton as sdl3MouseButton;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serializing", derive(serde::Serialize, serde::Deserialize))]
pub enum MouseButton {
	Unknown,
	Left,
	Middle,
	Right,
	X1,
	X2,
}

impl From<sdl3MouseButton> for MouseButton {
	fn from(value: sdl3MouseButton) -> Self {
		match value {
			sdl3MouseButton::Unknown => MouseButton::Unknown,
			sdl3MouseButton::Left => MouseButton::Left,
			sdl3MouseButton::Middle => MouseButton::Middle,
			sdl3MouseButton::Right => MouseButton::Right,
			sdl3MouseButton::X1 => MouseButton::X1,
			sdl3MouseButton::X2 => MouseButton::X2,
		}
	}
}

impl From<MouseButton> for sdl3MouseButton {
	fn from(value: MouseButton) -> Self {
		match value {
			MouseButton::Unknown => sdl3MouseButton::Unknown,
			MouseButton::Left => sdl3MouseButton::Left,
			MouseButton::Middle => sdl3MouseButton::Middle,
			MouseButton::Right => sdl3MouseButton::Right,
			MouseButton::X1 => sdl3MouseButton::X1,
			MouseButton::X2 => sdl3MouseButton::X2,
		}
	}
}
