use std::{
	any::{Any, TypeId},
	cell::{Ref, RefCell, RefMut},
	collections::HashMap,
	rc::Rc,
};

pub struct Resources(HashMap<TypeId, Rc<RefCell<dyn Any>>>);

impl Resources {
	pub fn get<T: 'static>(&'_ self) -> Ref<'_, T> {
		Ref::map(
			self.0.get(&TypeId::of::<T>()).unwrap().borrow(),
			|resource| resource.downcast_ref().unwrap(),
		)
	}

	pub fn get_mut<T: 'static>(&'_ self) -> RefMut<'_, T> {
		RefMut::map(
			self.0.get(&TypeId::of::<T>()).unwrap().borrow_mut(),
			|resource| resource.downcast_mut().unwrap(),
		)
	}

	pub(crate) fn new() -> Self {
		Self(HashMap::new())
	}

	pub(crate) fn insert<T: 'static>(&mut self, resource: T) {
		self.0
			.insert(TypeId::of::<T>(), Rc::new(RefCell::new(resource)));
	}
}
