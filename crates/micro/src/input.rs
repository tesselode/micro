//! Types related to input devices.

mod axis;
mod button;
mod mouse_scroll_delta;

pub use axis::Axis;
pub use button::Button;
pub use mouse_scroll_delta::*;

pub use winit::{event::MouseButton, keyboard::KeyCode};
