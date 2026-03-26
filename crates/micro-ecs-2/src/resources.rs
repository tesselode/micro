use std::{
	any::{Any, TypeId},
	cell::{RefCell, RefMut},
	collections::HashMap,
};

pub struct Resources {
	resources: HashMap<TypeId, RefCell<Box<dyn Any>>>,
}

impl Resources {
	pub fn new() -> Self {
		Self {
			resources: HashMap::new(),
		}
	}

	pub fn insert<T: 'static>(&mut self, resource: T) {
		self.resources
			.insert(TypeId::of::<T>(), RefCell::new(Box::new(resource)));
	}

	pub fn get<T: 'static>(&self) -> RefMut<'_, T> {
		let any = self.resources.get(&TypeId::of::<T>()).unwrap().borrow_mut();
		RefMut::map(any, |any| any.downcast_mut::<T>().unwrap())
	}
}

impl Default for Resources {
	fn default() -> Self {
		Self::new()
	}
}
