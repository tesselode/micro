use std::{
	hash::Hash,
	sync::{Arc, Weak},
};

use indexmap::IndexMap;

pub(crate) trait GraphicsResourceId: Copy + Eq + Hash {
	fn next() -> Self;
}

pub(crate) trait GraphicsResource {
	type Id: GraphicsResourceId;
}

pub(crate) struct GraphicsResourceWrapper<T> {
	pub resource: T,
	pub users: Arc<()>,
}

impl<T> GraphicsResourceWrapper<T> {
	pub(crate) fn new(resource: T) -> (Self, Weak<()>) {
		let users = Arc::new(());
		let weak = Arc::downgrade(&users);
		(Self { resource, users }, weak)
	}
}

pub(crate) struct GraphicsResources<T: GraphicsResource> {
	pub resources: IndexMap<T::Id, GraphicsResourceWrapper<T>>,
}

impl<T: GraphicsResource> GraphicsResources<T> {
	pub fn new() -> Self {
		Self {
			resources: IndexMap::new(),
		}
	}

	pub fn insert(&mut self, resource: T) -> (T::Id, Weak<()>) {
		let id = T::Id::next();
		let (wrapper, weak) = GraphicsResourceWrapper::new(resource);
		self.resources.insert(id, wrapper);
		(id, weak)
	}

	pub fn get(&self, id: T::Id) -> &T {
		&self
			.resources
			.get(&id)
			.expect("resource does not exist")
			.resource
	}

	pub fn get_mut(&mut self, id: T::Id) -> &mut T {
		&mut self
			.resources
			.get_mut(&id)
			.expect("resource does not exist")
			.resource
	}

	pub fn delete_unused(&mut self) {
		self.resources
			.retain(|_, resource| Arc::weak_count(&resource.users) > 0)
	}
}
