use sdl2::controller::{Axis, Button};
pub use sdl2::{
	keyboard::{Keycode, Scancode},
	mouse::{MouseButton, MouseWheelDirection},
};

pub struct GameController(pub(crate) sdl2::controller::GameController);

impl GameController {
	pub fn is_attached(&self) -> bool {
		self.0.attached()
	}

	pub fn is_button_down(&self, button: Button) -> bool {
		self.0.button(button)
	}

	pub fn axis_value(&self, axis: Axis) -> f32 {
		self.0.axis(axis) as f32 / i16::MAX as f32
	}
}
