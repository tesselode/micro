//! Types related to input devices.

mod axis;
mod button;
mod mouse_scroll_delta;

pub use axis::Axis;
pub use button::Button;
pub use mouse_scroll_delta::*;

pub use gilrs::GamepadId;
pub use winit::{event::MouseButton, keyboard::KeyCode};

pub struct GamepadIterator<'a> {
	pub(crate) inner: Option<gilrs::ConnectedGamepadsIterator<'a>>,
}

impl<'a> Iterator for GamepadIterator<'a> {
	type Item = GamepadId;

	fn next(&mut self) -> Option<Self::Item> {
		self.inner
			.as_mut()
			.and_then(|inner| inner.next().map(|(id, _)| id))
	}
}
