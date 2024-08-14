use std::time::Duration;

use crate::{event::Event, Context};

#[allow(unused_variables)]
pub trait App<E> {
	fn debug_ui(&mut self, ctx: &mut Context, egui_ctx: &egui::Context) -> Result<(), E> {
		Ok(())
	}

	fn event(&mut self, ctx: &mut Context, event: Event) -> Result<(), E> {
		Ok(())
	}

	fn update(&mut self, ctx: &mut Context, delta_time: Duration) -> Result<(), E> {
		Ok(())
	}

	fn draw(&mut self, ctx: &mut Context) -> Result<(), E> {
		Ok(())
	}
}
