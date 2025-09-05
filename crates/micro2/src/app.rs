use std::time::Duration;

use crate::event::Event;

#[allow(unused_variables)]
pub trait App {
	fn event(&mut self, event: Event) {}

	fn update(&mut self, delta_time: Duration) {}

	fn draw(&mut self) {}
}
