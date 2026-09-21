mod ecs;
mod event_dispatcher;
mod has_resources;
mod queues;
mod systems;

pub use ecs::*;
pub use has_resources::*;
pub use hecs::*;
pub use queues::*;

use micro::Micro;

pub type System<Globals, Event> =
	fn(&mut Micro, &mut Globals, &mut World, &mut Queues<Globals>, &Event);
