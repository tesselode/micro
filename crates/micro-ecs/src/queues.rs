use std::collections::VecDeque;

use hecs::{Bundle, CommandBuffer, Component, DynamicBundle, Entity};
use indexmap::IndexSet;

pub struct Queues<WorldEvent> {
	event_queue: VecDeque<WorldEvent>,
	command_buffer: CommandBuffer,
	queued_for_despawn: IndexSet<Entity>,
}

impl<WorldEvent> Queues<WorldEvent> {
	pub fn push_event(&mut self, event: WorldEvent) {
		self.event_queue.push_back(event);
	}

	pub fn despawn(&mut self, entity: Entity)
	where
		WorldEvent: From<EntityWillDespawn>,
	{
		self.command_buffer.despawn(entity);
		if !self.queued_for_despawn.contains(&entity) {
			self.event_queue.push_back(EntityWillDespawn(entity).into());
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
		}
	}

	pub(crate) fn pop_event(&mut self) -> Option<WorldEvent> {
		self.event_queue.pop_front()
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EntityWillDespawn(pub Entity);
