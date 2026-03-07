//! Types related to input devices.

mod axis;
mod button;
mod gamepad;
mod mouse_button;
mod scancode;

pub use axis::Axis;
pub use button::Button;
pub use gamepad::*;
pub use mouse_button::MouseButton;
pub use scancode::Scancode;
pub use sdl3::{keyboard::Keycode, mouse::MouseWheelDirection};
