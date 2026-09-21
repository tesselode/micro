use std::{
	any::{Any, TypeId},
	cell::{Ref, RefCell, RefMut},
	collections::HashMap,
};

#[derive(Debug)]
pub struct Resources(HashMap<TypeId, Box<dyn Any>>);

impl Resources {
	pub fn new() -> Self {
		Self(HashMap::new())
	}

	pub fn insert<R>(&mut self, resource: R)
	where
		R: 'static,
	{
		self.0
			.insert(TypeId::of::<R>(), Box::new(RefCell::new(resource)));
	}

	pub fn get<R>(&self) -> RefMut<'_, R>
	where
		R: 'static,
	{
		self.0
			.get(&TypeId::of::<R>())
			.map(|v| v.downcast_ref::<RefCell<R>>().unwrap().borrow_mut())
			.unwrap()
	}
}

impl Default for Resources {
	fn default() -> Self {
		Self::new()
	}
}
