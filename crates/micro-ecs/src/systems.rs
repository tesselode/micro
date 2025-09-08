use std::{
	any::{Any, TypeId},
	collections::HashMap,
};

use crate::System;

#[derive(Debug)]
pub(crate) struct Systems(HashMap<TypeId, Box<dyn Any>>);

impl Systems {
	pub(crate) fn new() -> Self {
		Self(HashMap::new())
	}

	pub(crate) fn get_mut<Context: 'static>(&mut self) -> &mut Vec<System<Context>> {
		self.0
			.entry(TypeId::of::<Context>())
			.or_insert_with(|| Box::new(Vec::<System<Context>>::new()))
			.downcast_mut::<Vec<System<Context>>>()
			.unwrap()
	}
}
