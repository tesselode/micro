use sdl3::mouse::MouseButton as sdl3MouseButton;

/// A button on a computer mouse.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serializing", derive(serde::Serialize, serde::Deserialize))]
pub enum MouseButton {
	/// An unrecognized button.
	Unknown,
	/// The left mouse button.
	Left,
	/// The middle mouse button (pressing down the scroll wheel).
	Middle,
	/// The right mouse button.
	Right,
	/// Side button 1.
	X1,
	/// Side button 2.
	X2,
}

impl MouseButton {
	pub const KNOWN: [Self; 5] = [Self::Left, Self::Middle, Self::Right, Self::X1, Self::X2];
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
