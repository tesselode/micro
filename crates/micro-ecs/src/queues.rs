use std::{
	any::{Any, TypeId},
	collections::VecDeque,
};

use hecs::{Bundle, CommandBuffer, Component, DynamicBundle, Entity};
use indexmap::IndexSet;

use crate::event_dispatcher::{EventDispatcher, EventDispatcherTrait};

pub struct Queues<Globals> {
	event_queue: VecDeque<(TypeId, Box<dyn Any>)>,
	command_buffer: CommandBuffer,
	queued_for_despawn: IndexSet<Entity>,
	known_events: IndexSet<TypeId>,
	new_event_dispatchers: Vec<(TypeId, Box<dyn EventDispatcherTrait<Globals>>)>,
}

impl<Globals> Queues<Globals> {
	pub fn push_event<Event>(&mut self, event: Event)
	where
		Globals: 'static,
		Event: 'static,
	{
		let event_type_id = TypeId::of::<Event>();
		self.event_queue.push_back((event_type_id, Box::new(event)));
		if !self.known_events.contains(&event_type_id) {
			self.known_events.insert(event_type_id);
			self.new_event_dispatchers.push((
				event_type_id,
				Box::new(EventDispatcher::<Globals, Event>::new()),
			));
		}
	}

	pub fn pop_event(&mut self) -> Option<(TypeId, Box<dyn Any>)> {
		self.event_queue.pop_front()
	}

	pub fn despawn(&mut self, entity: Entity)
	where
		Globals: 'static,
	{
		self.command_buffer.despawn(entity);
		if !self.queued_for_despawn.contains(&entity) {
			self.push_event(EntityWillDespawn(entity));
			self.queued_for_despawn.insert(entity);
		}
	}

	pub fn insert(&mut self, entity: Entity, components: impl DynamicBundle) {
		self.command_buffer.insert(entity, components)
	}

	pub fn insert_one(&mut self, entity: Entity, component: impl Component) {
		self.command_buffer.insert_one(entity, component)
	}

	pub fn remove<T: Bundle + 'static>(&mut self, ent: Entity) {
		self.command_buffer.remove::<T>(ent)
	}

	pub fn remove_one<T: Component>(&mut self, ent: Entity) {
		self.command_buffer.remove_one::<T>(ent)
	}

	pub fn flush_world_queue(&mut self, world: &mut hecs::World) {
		self.command_buffer.run_on(world);
		self.queued_for_despawn.clear();
	}

	pub fn spawn(&mut self, components: impl DynamicBundle) {
		self.command_buffer.spawn(components)
	}

	pub(crate) fn new() -> Self {
		Self {
			event_queue: VecDeque::new(),
			command_buffer: CommandBuffer::new(),
			queued_for_despawn: IndexSet::new(),
			known_events: IndexSet::new(),
			new_event_dispatchers: vec![],
		}
	}

	pub(crate) fn drain_event_dispatchers(
		&mut self,
	) -> impl Iterator<Item = (TypeId, Box<dyn EventDispatcherTrait<Globals>>)> {
		self.new_event_dispatchers.drain(..)
	}
}

impl<Globals> Default for Queues<Globals> {
	fn default() -> Self {
		Self::new()
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EntityWillDespawn(pub Entity);
