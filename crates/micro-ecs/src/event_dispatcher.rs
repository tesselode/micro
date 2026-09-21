use std::{any::Any, marker::PhantomData};

use hecs::World;
use micro::Micro;

use crate::{Queues, systems::Systems};

pub trait EventDispatcherTrait<Globals> {
	fn dispatch(
		&self,
		micro: &mut Micro,
		globals: &mut Globals,
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
		world: &mut World,
		queues: &mut Queues<Globals>,
		systems: &mut Systems<Globals>,
		event: Box<dyn Any>,
	) {
		let event = event.downcast_ref::<Event>().unwrap();
		for system in systems.for_event() {
			system(micro, globals, world, queues, event);
		}
	}
}
