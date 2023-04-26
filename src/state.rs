use std::time::Duration;

use crate::{Context, Event};

#[allow(unused_variables)]
pub trait State {
	fn ui(&mut self, ctx: &mut Context, egui_ctx: &egui::Context) {}

	fn event(&mut self, ctx: &mut Context, event: Event) {}

	fn update(&mut self, ctx: &mut Context, delta_time: Duration) {}

	fn draw(&mut self, ctx: &mut Context) {}
}
