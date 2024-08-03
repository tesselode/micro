use std::time::Duration;

use crate::{event::Event, Context, Globals};

#[allow(unused_variables)]
pub trait State<E> {
	fn ui(
		&mut self,
		ctx: &mut Context,
		globals: &mut Globals,
		egui_ctx: &egui::Context,
	) -> Result<(), E> {
		Ok(())
	}

	fn event(&mut self, ctx: &mut Context, globals: &mut Globals, event: Event) -> Result<(), E> {
		Ok(())
	}

	fn update(
		&mut self,
		ctx: &mut Context,
		globals: &mut Globals,
		delta_time: Duration,
	) -> Result<(), E> {
		Ok(())
	}

	fn draw(&mut self, ctx: &mut Context, globals: &mut Globals) -> Result<(), E> {
		Ok(())
	}
}
