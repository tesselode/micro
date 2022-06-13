use std::time::Duration;

use sdl2::event::Event;

use crate::Context;

#[allow(unused_variables)]
pub trait State {
	fn event(&mut self, ctx: &mut Context, event: Event) {
	}

	fn update(&mut self, ctx: &mut Context, delta_time: Duration) {
	}

	fn draw(&mut self, ctx: &mut Context) {
	}
}
