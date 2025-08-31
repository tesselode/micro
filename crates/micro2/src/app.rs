use std::time::Duration;

use crate::{Context, event::Event};

#[allow(unused_variables)]
pub trait App {
	fn event(&mut self, ctx: &mut Context, event: Event) {}

	fn update(&mut self, ctx: &mut Context, delta_time: Duration) {}

	fn draw(&mut self, ctx: &mut Context) {}
}
