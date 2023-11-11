use glam::{Affine2, IVec2, UVec2};

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
	pub fn transform_mouse_events(self, transform: Affine2) -> Self {
		match self {
			Self::MouseMoved { position, delta } => Self::MouseMoved {
				position: transform.transform_point2(position.as_vec2()).as_ivec2(),
				delta: transform.transform_vector2(delta.as_vec2()).as_ivec2(),
			},
			Self::MouseButtonPressed {
				button,
				mouse_position,
			} => Self::MouseButtonPressed {
				button,
				mouse_position: transform
					.transform_point2(mouse_position.as_vec2())
					.as_ivec2(),
			},
			Self::MouseButtonReleased {
				button,
				mouse_position,
			} => Self::MouseButtonReleased {
				button,
				mouse_position: transform
					.transform_point2(mouse_position.as_vec2())
					.as_ivec2(),
			},
			_ => self,
		}
	}

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
				repeat,
				..
			} => Some(Self::KeyPressed {
				key: scancode.into(),
				is_repeat: repeat,
			}),
			sdl2::event::Event::KeyUp {
				scancode: Some(scancode),
				..
			} => Some(Self::KeyReleased(scancode.into())),
			sdl2::event::Event::MouseMotion {
				x, y, xrel, yrel, ..
			} => Some(Self::MouseMoved {
				position: IVec2::new(x, y),
				delta: IVec2::new(xrel, yrel),
			}),
			sdl2::event::Event::MouseButtonDown {
				mouse_btn, x, y, ..
			} => Some(Self::MouseButtonPressed {
				button: mouse_btn.into(),
				mouse_position: IVec2::new(x, y),
			}),
			sdl2::event::Event::MouseButtonUp {
				mouse_btn, x, y, ..
			} => Some(Self::MouseButtonReleased {
				button: mouse_btn.into(),
				mouse_position: IVec2::new(x, y),
			}),
			sdl2::event::Event::MouseWheel { x, y, .. } => {
				Some(Self::MouseWheelMoved(IVec2::new(x, y)))
			}
			sdl2::event::Event::ControllerAxisMotion {
				which, axis, value, ..
			} => Some(Self::GamepadAxisMoved {
				gamepad_id: which,
				axis: axis.into(),
				value: value as f32 / i16::MAX as f32,
			}),
			sdl2::event::Event::ControllerButtonDown { which, button, .. } => {
				Some(Self::GamepadButtonPressed {
					gamepad_id: which,
					button: button.into(),
				})
			}
			sdl2::event::Event::ControllerButtonUp { which, button, .. } => {
				Some(Self::GamepadButtonReleased {
					gamepad_id: which,
					button: button.into(),
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
