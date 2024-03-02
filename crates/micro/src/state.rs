use std::time::Duration;

use crate::event::Event;

#[allow(unused_variables)]
pub trait State<E> {
	fn ui(&mut self, egui_ctx: &egui::Context) -> Result<(), E> {
		Ok(())
	}

	fn event(&mut self, event: Event) -> Result<(), E> {
		Ok(())
	}

	fn update(&mut self, delta_time: Duration) -> Result<(), E> {
		Ok(())
	}

	fn draw(&mut self) -> Result<(), E> {
		Ok(())
	}
}
