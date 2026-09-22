use hecs::World;
use micro::Micro;

use crate::{Queues, Resources};

pub trait System<Globals, Event> {
	fn run(
		&mut self,
		micro: &mut Micro,
		globals: &mut Globals,
		resources: &mut Resources,
		world: &mut World,
		queues: &mut Queues<Globals>,
		event: &Event,
	);
}

impl<T, Globals, Event> System<Globals, Event> for T
where
	T: FnMut(&mut Micro, &mut Globals, &mut Resources, &mut World, &mut Queues<Globals>, &Event),
{
	fn run(
		&mut self,
		micro: &mut Micro,
		globals: &mut Globals,
		resources: &mut Resources,
		world: &mut World,
		queues: &mut Queues<Globals>,
		event: &Event,
	) {
		self(micro, globals, resources, world, queues, event)
	}
}
