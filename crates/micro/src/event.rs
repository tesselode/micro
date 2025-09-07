use glam::{UVec2, Vec2};

use crate::input::{Axis, Button, MouseButton, Scancode};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Event {
	WindowSizeChanged(UVec2),
	KeyPressed {
		key: Scancode,
		is_repeat: bool,
	},
	KeyReleased(Scancode),
	MouseMoved {
		position: Vec2,
		delta: Vec2,
	},
	MouseButtonPressed {
		button: MouseButton,
		mouse_position: Vec2,
	},
	MouseButtonReleased {
		button: MouseButton,
		mouse_position: Vec2,
	},
	MouseWheelMoved(Vec2),
	GamepadAxisMoved {
		gamepad_id: u32,
		axis: Axis,
		value: f32,
	},
	GamepadButtonPressed {
		gamepad_id: u32,
		button: Button,
	},
	GamepadButtonReleased {
		gamepad_id: u32,
		button: Button,
	},
	GamepadConnected(u32),
	GamepadDisconnected(u32),
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
