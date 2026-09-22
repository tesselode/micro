use std::{any::Any, marker::PhantomData};

use hecs::World;
use micro::Micro;

use crate::{Queues, Resources, systems::Systems};

pub trait EventDispatcherTrait<Globals> {
	#[allow(clippy::too_many_arguments)]
	fn dispatch(
		&self,
		micro: &mut Micro,
		globals: &mut Globals,
		resources: &mut Resources,
		world: &mut World,
		queues: &mut Queues<Globals>,
		systems: &mut Systems<Globals>,
		event: Box<dyn Any>,
	);
}

pub struct EventDispatcher<Globals, Event> {
	globals: PhantomData<Globals>,
	event: PhantomData<Event>,
}

impl<Globals, Event> EventDispatcher<Globals, Event> {
	pub fn new() -> Self {
		Self {
			globals: PhantomData,
			event: PhantomData,
		}
	}
}

impl<Globals: 'static, Event: 'static> EventDispatcherTrait<Globals>
	for EventDispatcher<Globals, Event>
{
	fn dispatch(
		&self,
		micro: &mut Micro,
		globals: &mut Globals,
		resources: &mut Resources,
		world: &mut World,
		queues: &mut Queues<Globals>,
		systems: &mut Systems<Globals>,
		event: Box<dyn Any>,
	) {
		let event = event.downcast_ref::<Event>().unwrap();
		for system in systems.for_event() {
			system.run(micro, globals, resources, world, queues, event);
		}
	}
}
