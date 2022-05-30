mod context;
pub mod graphics;
pub mod input;
pub mod math;
mod state;

pub use context::{run, Context, ContextSettings};
pub use sdl2::event::Event;
pub use state::*;
