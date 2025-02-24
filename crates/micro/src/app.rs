use std::time::Duration;

use crate::{Context, event::Event};

#[allow(unused_variables)]
pub trait App {
	type Error;

	fn debug_ui(&mut self, ctx: &mut Context, egui_ctx: &egui::Context) -> Result<(), Self::Error> {
		Ok(())
	}

	fn event(&mut self, ctx: &mut Context, event: Event) -> Result<(), Self::Error> {
		Ok(())
	}

	fn update(&mut self, ctx: &mut Context, delta_time: Duration) -> Result<(), Self::Error> {
		Ok(())
	}

	fn draw(&mut self, ctx: &mut Context) -> Result<(), Self::Error> {
		Ok(())
	}
}
