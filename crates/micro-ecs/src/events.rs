use std::{
	any::{Any, TypeId},
	collections::VecDeque,
};

use indexmap::IndexMap;

pub struct Events(pub(crate) IndexMap<TypeId, Box<dyn EventStorage>>);

impl Events {
	pub fn emit<Event: 'static>(&mut self, event: Event) {
		self.get_mut::<Event>().push(Box::new(event));
	}

	pub(crate) fn new() -> Self {
		Self(IndexMap::new())
	}

	pub(crate) fn get_mut<Event: 'static>(&mut self) -> &mut Box<dyn EventStorage> {
		self.0
			.entry(TypeId::of::<Event>())
			.or_insert_with(|| Box::new(VecDeque::<Event>::new()))
	}

	pub(crate) fn is_empty(&self) -> bool {
		self.0.values().all(|events| events.is_empty())
	}
}

pub(crate) trait EventStorage {
	fn is_empty(&self) -> bool;

	fn push(&mut self, event: Box<dyn Any>);

	fn pop(&mut self) -> Option<Box<dyn Any>>;
}

impl<Event: 'static> EventStorage for VecDeque<Event> {
	fn is_empty(&self) -> bool {
		self.is_empty()
	}

	fn push(&mut self, event: Box<dyn Any>) {
		self.push_back(*event.downcast().unwrap());
	}

	fn pop(&mut self) -> Option<Box<dyn Any>> {
		self.pop_front().map(|event| {
			let event: Box<dyn Any> = Box::new(event);
			event
		})
	}
}
