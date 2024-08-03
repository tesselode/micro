use std::{
	any::{Any, TypeId},
	collections::HashMap,
};

#[derive(Debug)]
pub struct Globals(HashMap<TypeId, Box<dyn Any>>);

impl Globals {
	pub(crate) fn new() -> Self {
		Self(HashMap::new())
	}

	pub fn add<T: 'static>(&mut self, userdata: T) -> &mut Self {
		self.0.insert(userdata.type_id(), Box::new(userdata));
		self
	}

	pub fn remove<T: 'static>(&mut self) -> Option<T> {
		self.0
			.remove(&TypeId::of::<T>())
			.map(|userdata| *userdata.downcast().unwrap())
	}

	pub fn get<T: 'static>(&self) -> &T {
		self.0
			.get(&TypeId::of::<T>())
			.unwrap()
			.downcast_ref()
			.unwrap()
	}

	pub fn get_mut<T: 'static>(&mut self) -> &mut T {
		self.0
			.get_mut(&TypeId::of::<T>())
			.unwrap()
			.downcast_mut()
			.unwrap()
	}
}
