use std::any::{Any, TypeId};

use indexmap::IndexMap;

use crate::{Entities, EventStorage, Events, Trigger};

pub(crate) struct Triggers(pub(crate) IndexMap<TypeId, Box<dyn TriggerStorage>>);

impl Triggers {
	pub(crate) fn new() -> Self {
		Self(IndexMap::new())
	}

	pub(crate) fn get_mut<Event: 'static>(&mut self) -> &mut Box<dyn TriggerStorage> {
		self.0
			.entry(TypeId::of::<Event>())
			.or_insert_with(|| Box::new(Vec::<Trigger<Event>>::new()))
	}

	pub(crate) fn add<Event: 'static>(&mut self, trigger: Trigger<Event>) {
		self.get_mut::<Event>().add(Box::new(trigger));
	}
}

pub(crate) trait TriggerStorage {
	fn add(&mut self, trigger: Box<dyn Any>);

	fn run(
		&mut self,
		events: &mut dyn EventStorage,
		entities: &mut Entities,
		new_events: &mut Events,
	);
}

impl<Event: 'static> TriggerStorage for Vec<Trigger<Event>> {
	fn add(&mut self, trigger: Box<dyn Any>) {
		self.push(*trigger.downcast().unwrap());
	}

	fn run(
		&mut self,
		events: &mut dyn EventStorage,
		entities: &mut Entities,
		new_events: &mut Events,
	) {
		while let Some(event) = events.pop() {
			let event = event.downcast::<Event>().unwrap();
			for trigger in &mut *self {
				trigger(entities, new_events, &event);
			}
		}
	}
}
