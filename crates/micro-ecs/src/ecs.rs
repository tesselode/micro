use std::{any::TypeId, collections::HashMap};

use hecs::World;
use micro::Micro;

use crate::{Queues, System, event_dispatcher::EventDispatcherTrait, systems::Systems};

pub struct Ecs<Globals> {
	pub world: World,
	pub queues: Queues<Globals>,
	systems: Systems<Globals>,
	event_dispatchers: HashMap<TypeId, Box<dyn EventDispatcherTrait<Globals>>>,
}

impl<Globals> Ecs<Globals> {
	pub fn new() -> Self {
		Self {
			world: World::new(),
			queues: Queues::new(),
			systems: Systems::new(),
			event_dispatchers: HashMap::new(),
		}
	}

	pub fn system<Event>(mut self, system: System<Globals, Event>) -> Self
	where
		Globals: 'static,
		Event: 'static,
	{
		self.systems.for_event().push(system);
		self
	}

	pub fn emit<Event>(&mut self, micro: &mut Micro, globals: &mut Globals, event: Event)
	where
		Globals: 'static,
		Event: 'static,
	{
		for system in self.systems.for_event::<Event>() {
			system(micro, globals, &mut self.world, &mut self.queues, &event);
		}
		self.flush_events(micro, globals);
	}

	pub fn flush_world_queue(&mut self) {
		self.queues.flush_world_queue(&mut self.world)
	}

	fn flush_events(&mut self, micro: &mut Micro, globals: &mut Globals) {
		for (type_id, event_dispatcher) in self.queues.drain_event_dispatchers() {
			self.event_dispatchers.insert(type_id, event_dispatcher);
		}
		while let Some((type_id, event)) = self.queues.pop_event() {
			self.event_dispatchers[&type_id].dispatch(
				micro,
				globals,
				&mut self.world,
				&mut self.queues,
				&mut self.systems,
				event,
			);
		}
	}
}

impl<Globals> Default for Ecs<Globals> {
	fn default() -> Self {
		Self::new()
	}
}
