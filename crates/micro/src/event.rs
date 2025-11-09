use glam::{UVec2, Vec2};

use crate::input::{Axis, Button, MouseButton, Scancode};

/// An event sent by Micro to your [`App`](crate::App).
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Event {
	/// The window size change.
	WindowSizeChanged(UVec2),
	/// A keyboard key was pressed.
	KeyPressed {
		/// The key that was pressed.
		key: Scancode,
		/// Whether the key press event comes from key repeat from
		/// holding down a key.
		is_repeat: bool,
	},
	/// A keyboard key was released.
	KeyReleased(Scancode),
	/// The mouse was moved.
	MouseMoved {
		/// The new mouse position (in pixels).
		position: Vec2,
		/// How much the mouse moved (in pixels).
		delta: Vec2,
	},
	/// A mouse button was pressed.
	MouseButtonPressed {
		/// The mouse button that was pressed.
		button: MouseButton,
		/// The position of the mouse at the time of the button press (in pixels).
		mouse_position: Vec2,
	},
	/// A mouse button was released.
	MouseButtonReleased {
		/// The mouse button that was released.
		button: MouseButton,
		/// The position of the mouse at the time of the button release (in pixels).
		mouse_position: Vec2,
	},
	/// The mouse scroll wheel was moved.
	MouseWheelMoved(Vec2),
	/// A gamepad axis was moved.
	GamepadAxisMoved {
		/// The index of the gamepad.
		gamepad_id: u32,
		/// Which axis was moved.
		axis: Axis,
		/// The new value of that axis.
		value: f32,
	},
	/// A gamepad button was pressed.
	GamepadButtonPressed {
		/// The index of the gamepad.
		gamepad_id: u32,
		/// The button that was pressed.
		button: Button,
	},
	/// A gamepad button was released.
	GamepadButtonReleased {
		/// The index of the gamepad.
		gamepad_id: u32,
		/// The button that was released.
		button: Button,
	},
	/// A gamepad was connected.
	GamepadConnected(u32),
	/// A gamepad was disconnected.
	GamepadDisconnected(u32),
	/// The app was exited.
	Exited,
}

impl Event {
	pub(crate) fn from_sdl3_event(sdl3_event: sdl3::event::Event) -> Option<Self> {
		match sdl3_event {
			sdl3::event::Event::Quit { .. } => Some(Self::Exited),
			sdl3::event::Event::Window {
				win_event: sdl3::event::WindowEvent::PixelSizeChanged(width, height),
				..
			} => Some(Self::WindowSizeChanged(UVec2::new(
				width.try_into().expect("window width is negative"),
				height.try_into().expect("window height is negative"),
			))),
			sdl3::event::Event::KeyDown {
				scancode: Some(scancode),
				repeat,
				..
			} => Some(Self::KeyPressed {
				key: scancode.into(),
				is_repeat: repeat,
			}),
			sdl3::event::Event::KeyUp {
				scancode: Some(scancode),
				..
			} => Some(Self::KeyReleased(scancode.into())),
			sdl3::event::Event::MouseMotion {
				x, y, xrel, yrel, ..
			} => Some(Self::MouseMoved {
				position: Vec2::new(x, y),
				delta: Vec2::new(xrel, yrel),
			}),
			sdl3::event::Event::MouseButtonDown {
				mouse_btn, x, y, ..
			} => Some(Self::MouseButtonPressed {
				button: mouse_btn.into(),
				mouse_position: Vec2::new(x, y),
			}),
			sdl3::event::Event::MouseButtonUp {
				mouse_btn, x, y, ..
			} => Some(Self::MouseButtonReleased {
				button: mouse_btn.into(),
				mouse_position: Vec2::new(x, y),
			}),
			sdl3::event::Event::MouseWheel { x, y, .. } => {
				Some(Self::MouseWheelMoved(Vec2::new(x, y)))
			}
			sdl3::event::Event::ControllerAxisMotion {
				which, axis, value, ..
			} => Some(Self::GamepadAxisMoved {
				gamepad_id: which,
				axis: axis.into(),
				value: value as f32 / i16::MAX as f32,
			}),
			sdl3::event::Event::ControllerButtonDown { which, button, .. } => {
				Some(Self::GamepadButtonPressed {
					gamepad_id: which,
					button: button.into(),
				})
			}
			sdl3::event::Event::ControllerButtonUp { which, button, .. } => {
				Some(Self::GamepadButtonReleased {
					gamepad_id: which,
					button: button.into(),
				})
			}
			sdl3::event::Event::ControllerDeviceAdded { which, .. } => {
				Some(Self::GamepadConnected(which))
			}
			sdl3::event::Event::ControllerDeviceRemoved { which, .. } => {
				Some(Self::GamepadDisconnected(which))
			}
			_ => None,
		}
	}
}
