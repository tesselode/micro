use std::time::Duration;

use crate::event::Event;

#[allow(unused_variables)]
pub trait App {
	fn debug_ui(&mut self, egui_ctx: &crate::egui::Context) {}

	fn event(&mut self, event: Event) {}

	fn update(&mut self, delta_time: Duration) {}

	fn draw(&mut self) {}
}
