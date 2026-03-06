use glam::{Mat4, UVec2, Vec2, uvec2, vec2};
use winit::{
	event::KeyEvent,
	keyboard::{KeyCode, PhysicalKey},
};

use crate::input::{Axis, Button, MouseButton, MouseScrollDelta};

/// An event sent by Micro to your [`App`](crate::App).
#[derive(Debug, Clone, PartialEq)]
pub enum Event {
	/// The window size change.
	WindowSizeChanged(UVec2),
	/// A keyboard key was pressed.
	KeyPressed {
		/// The key that was pressed.
		key: KeyCode,
		/// Whether the key press event comes from key repeat from
		/// holding down a key.
		is_repeat: bool,
	},
	/// A keyboard key was released.
	KeyReleased(KeyCode),
	TextInput(String),
	MouseEntered,
	MouseExited,
	/// The mouse cursor position changed.
	CursorPositionChanged(Vec2),
	/// The mouse was moved.
	MouseMoved(Vec2),
	/// A mouse button was pressed.
	MouseButtonPressed {
		button: MouseButton,
		position: Vec2,
	},
	/// A mouse button was released.
	MouseButtonReleased {
		button: MouseButton,
		position: Vec2,
	},
	/// The mouse scroll wheel was moved.
	MouseWheelMoved(MouseScrollDelta),
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
	pub fn transform_mouse_events(self, transform: Mat4) -> Self {
		match self {
			Self::CursorPositionChanged(position) => Self::CursorPositionChanged(
				transform.transform_point3(position.extend(0.0)).truncate(),
			),
			Self::MouseMoved(delta) => Self::CursorPositionChanged(
				transform.transform_vector3(delta.extend(0.0)).truncate(),
			),
			_ => self,
		}
	}

	pub(crate) fn from_window_event(
		event: &winit::event::WindowEvent,
		mouse_position: Option<Vec2>,
	) -> Vec<Event> {
		match event {
			winit::event::WindowEvent::Resized(physical_size) => vec![Event::WindowSizeChanged(
				uvec2(physical_size.width, physical_size.height),
			)],
			winit::event::WindowEvent::KeyboardInput {
				event:
					KeyEvent {
						physical_key: PhysicalKey::Code(code),
						state,
						repeat,
						text,
						..
					},
				..
			} => {
				let mut events = vec![match state {
					winit::event::ElementState::Pressed => Event::KeyPressed {
						key: *code,
						is_repeat: *repeat,
					},
					winit::event::ElementState::Released => Event::KeyReleased(*code),
				}];
				if let Some(text) = text {
					events.push(Event::TextInput(text.to_string()));
				}
				events
			}
			winit::event::WindowEvent::CursorEntered { .. } => vec![Event::MouseEntered],
			winit::event::WindowEvent::CursorLeft { .. } => vec![Event::MouseExited],
			winit::event::WindowEvent::CursorMoved { position, .. } => {
				vec![Event::CursorPositionChanged(vec2(
					position.x as f32,
					position.y as f32,
				))]
			}
			winit::event::WindowEvent::MouseWheel { delta, .. } => {
				vec![Event::MouseWheelMoved((*delta).into())]
			}
			winit::event::WindowEvent::MouseInput { state, button, .. } => vec![match state {
				winit::event::ElementState::Pressed => Event::MouseButtonPressed {
					button: *button,
					position: mouse_position
						.expect("mouse click/release happened without a mouse position"),
				},
				winit::event::ElementState::Released => Event::MouseButtonReleased {
					button: *button,
					position: mouse_position
						.expect("mouse click/release happened without a mouse position"),
				},
			}],
			_ => vec![],
		}
	}

	pub(crate) fn from_device_event(event: &winit::event::DeviceEvent) -> Option<Event> {
		match event {
			winit::event::DeviceEvent::MouseMotion { delta } => {
				Some(Event::MouseMoved(vec2(delta.0 as f32, delta.1 as f32)))
			}
			_ => None,
		}
	}
}
