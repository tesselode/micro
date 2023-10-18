use sdl2::mouse::MouseButton as Sdl2MouseButton;

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

impl From<Sdl2MouseButton> for MouseButton {
	fn from(value: Sdl2MouseButton) -> Self {
		match value {
			Sdl2MouseButton::Unknown => MouseButton::Unknown,
			Sdl2MouseButton::Left => MouseButton::Left,
			Sdl2MouseButton::Middle => MouseButton::Middle,
			Sdl2MouseButton::Right => MouseButton::Right,
			Sdl2MouseButton::X1 => MouseButton::X1,
			Sdl2MouseButton::X2 => MouseButton::X2,
		}
	}
}

impl From<MouseButton> for Sdl2MouseButton {
	fn from(value: MouseButton) -> Self {
		match value {
			MouseButton::Unknown => Sdl2MouseButton::Unknown,
			MouseButton::Left => Sdl2MouseButton::Left,
			MouseButton::Middle => Sdl2MouseButton::Middle,
			MouseButton::Right => Sdl2MouseButton::Right,
			MouseButton::X1 => Sdl2MouseButton::X1,
			MouseButton::X2 => Sdl2MouseButton::X2,
		}
	}
}
