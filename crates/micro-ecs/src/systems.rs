use std::{
	any::{Any, TypeId},
	collections::HashMap,
	marker::PhantomData,
};

use crate::System;

pub struct Systems<Globals> {
	systems: HashMap<TypeId, Box<dyn Any>>,
	globals: PhantomData<Globals>,
}

impl<Globals> Systems<Globals> {
	pub fn new() -> Self {
		Self {
			systems: HashMap::new(),
			globals: PhantomData,
		}
	}

	pub fn for_event<Event>(&mut self) -> &mut Vec<System<Globals, Event>>
	where
		Globals: 'static,
		Event: 'static,
	{
		self.systems
			.entry(TypeId::of::<Event>())
			.or_insert_with(|| Box::<Vec<System<Globals, Event>>>::new(vec![]))
			.downcast_mut()
			.unwrap()
	}
}
