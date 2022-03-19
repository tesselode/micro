use std::time::Duration;

use sdl2::event::Event;

use crate::Context;

#[allow(unused_variables)]
pub trait State<E> {
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
