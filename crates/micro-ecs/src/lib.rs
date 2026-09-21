mod ecs;
mod event_dispatcher;
mod queues;
mod systems;

pub use ecs::*;
pub use hecs::*;
use micro::Micro;
pub use queues::*;

pub type System<Globals, Event> =
	fn(&mut Micro, &mut Globals, &mut World, &mut Queues<Globals>, &Event);
