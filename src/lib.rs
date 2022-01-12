pub mod color;
pub mod context;
pub mod image_data;
pub mod mesh;
pub mod texture;

use context::Context;
use sdl2::event::Event;

#[allow(unused_variables)]
pub trait Game<E> {
	fn event(&mut self, ctx: &mut Context, event: Event) -> Result<(), E> {
		Ok(())
	}

	fn update(&mut self, ctx: &mut Context) -> Result<(), E> {
		Ok(())
	}

	fn draw(&mut self, ctx: &mut Context) -> Result<(), E> {
		Ok(())
	}
}
