use glam::{IVec2, UVec2};
use sdl2::{
	controller::{Axis, Button},
	keyboard::Scancode,
	mouse::MouseButton,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Event {
	WindowSizeChanged(UVec2),
	KeyPressed(Scancode),
	KeyReleased(Scancode),
	MouseMoved {
		position: IVec2,
		delta: IVec2,
	},
	MouseButtonPressed {
		button: MouseButton,
		mouse_position: IVec2,
	},
	MouseButtonReleased {
		button: MouseButton,
		mouse_position: IVec2,
	},
	MouseWheelMoved(IVec2),
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
	pub(crate) fn from_sdl2_event(sdl2_event: sdl2::event::Event) -> Option<Self> {
		match sdl2_event {
			sdl2::event::Event::Quit { .. } => Some(Self::Exited),
			sdl2::event::Event::Window {
				win_event: sdl2::event::WindowEvent::SizeChanged(width, height),
				..
			} => Some(Self::WindowSizeChanged(UVec2::new(
				width.try_into().expect("window width is negative"),
				height.try_into().expect("window height is negative"),
			))),
			sdl2::event::Event::KeyDown {
				scancode: Some(scancode),
				..
			} => Some(Self::KeyPressed(scancode)),
			sdl2::event::Event::KeyUp {
				scancode: Some(scancode),
				..
			} => Some(Self::KeyReleased(scancode)),
			sdl2::event::Event::MouseMotion {
				x, y, xrel, yrel, ..
			} => Some(Self::MouseMoved {
				position: IVec2::new(x, y),
				delta: IVec2::new(xrel, yrel),
			}),
			sdl2::event::Event::MouseButtonDown {
				mouse_btn, x, y, ..
			} => Some(Self::MouseButtonPressed {
				button: mouse_btn,
				mouse_position: IVec2::new(x, y),
			}),
			sdl2::event::Event::MouseButtonUp {
				mouse_btn, x, y, ..
			} => Some(Self::MouseButtonReleased {
				button: mouse_btn,
				mouse_position: IVec2::new(x, y),
			}),
			sdl2::event::Event::MouseWheel { x, y, .. } => {
				Some(Self::MouseWheelMoved(IVec2::new(x, y)))
			}
			sdl2::event::Event::ControllerAxisMotion {
				which, axis, value, ..
			} => Some(Self::GamepadAxisMoved {
				gamepad_id: which,
				axis,
				value: value as f32 / i16::MAX as f32,
			}),
			sdl2::event::Event::ControllerButtonDown { which, button, .. } => {
				Some(Self::GamepadButtonPressed {
					gamepad_id: which,
					button,
				})
			}
			sdl2::event::Event::ControllerButtonUp { which, button, .. } => {
				Some(Self::GamepadButtonReleased {
					gamepad_id: which,
					button,
				})
			}
			sdl2::event::Event::ControllerDeviceAdded { which, .. } => {
				Some(Self::GamepadConnected(which))
			}
			sdl2::event::Event::ControllerDeviceRemoved { which, .. } => {
				Some(Self::GamepadDisconnected(which))
			}
			_ => None,
		}
	}
}
