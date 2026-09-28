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

	pub fn insert<R>(&mut self, resource: R) -> Option<R>
	where
		R: 'static,
	{
		let previous = self
			.0
			.insert(TypeId::of::<R>(), Box::new(RefCell::new(resource)));
		previous.map(|previous| RefCell::into_inner(*previous.downcast::<RefCell<R>>().unwrap()))
	}

	pub fn remove<R>(&mut self) -> Option<R>
	where
		R: 'static,
	{
		self.0
			.remove(&TypeId::of::<R>())
			.map(|previous| RefCell::into_inner(*previous.downcast::<RefCell<R>>().unwrap()))
	}

	pub fn get<R>(&self) -> Ref<'_, R>
	where
		R: 'static,
	{
		self.0
			.get(&TypeId::of::<R>())
			.map(|v| v.downcast_ref::<RefCell<R>>().unwrap().borrow())
			.unwrap()
	}

	pub fn get_mut<R>(&self) -> RefMut<'_, R>
	where
		R: 'static,
	{
		self.0
			.get(&TypeId::of::<R>())
			.map(|v| v.downcast_ref::<RefCell<R>>().unwrap().borrow_mut())
			.unwrap()
	}

	pub fn get_or_insert<R>(&mut self, resource: R) -> RefMut<'_, R>
	where
		R: 'static,
	{
		self.0
			.entry(TypeId::of::<R>())
			.or_insert(Box::new(RefCell::new(resource)))
			.downcast_ref::<RefCell<R>>()
			.unwrap()
			.borrow_mut()
	}

	pub fn get_or_insert_with<R>(&mut self, resource: impl FnOnce() -> R) -> RefMut<'_, R>
	where
		R: 'static,
	{
		self.0
			.entry(TypeId::of::<R>())
			.or_insert_with(|| Box::new(RefCell::new(resource())))
			.downcast_ref::<RefCell<R>>()
			.unwrap()
			.borrow_mut()
	}

	pub fn get_or_insert_default<R>(&mut self) -> RefMut<'_, R>
	where
		R: Default + 'static,
	{
		self.get_or_insert_with(R::default)
	}

	pub fn contains<R>(&self) -> bool
	where
		R: 'static,
	{
		self.0.contains_key(&TypeId::of::<R>())
	}
}

impl Default for Resources {
	fn default() -> Self {
		Self::new()
	}
}
