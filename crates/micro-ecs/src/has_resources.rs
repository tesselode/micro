use hecs::{Component, Entity, World};

pub trait HasResources {
	fn insert_resource<R: Component>(&mut self, resource: R) -> Option<R>;

	fn remove_resource<R: Component>(&mut self);

	fn contains_resource<R: Component>(&self) -> bool;

	fn resource<R: Component>(&mut self) -> Option<&mut R>;

	fn resource_or_insert_with<R: Component>(&mut self, resource: impl FnOnce() -> R) -> &mut R;

	fn resource_or_insert<R: Component>(&mut self, resource: R) -> &mut R;

	fn resource_or_default<R: Component + Default>(&mut self) -> &mut R;
}

impl HasResources for World {
	fn insert_resource<R: Component>(&mut self, mut resource: R) -> Option<R> {
		if let Some(old) = self.resource() {
			std::mem::swap(old, &mut resource);
			Some(resource)
		} else {
			self.spawn((resource,));
			None
		}
	}

	fn remove_resource<R: Component>(&mut self) {
		if let Some(entity) = self.query_mut::<Entity>().with::<&R>().into_iter().next() {
			self.despawn(entity).unwrap();
		}
	}

	fn contains_resource<R: Component>(&self) -> bool {
		self.query::<()>().with::<&R>().into_iter().next().is_some()
	}

	fn resource<R: Component>(&mut self) -> Option<&mut R> {
		self.query_mut::<&mut R>().into_iter().next()
	}

	fn resource_or_insert_with<R: Component>(&mut self, resource: impl FnOnce() -> R) -> &mut R {
		if !self.contains_resource::<R>() {
			self.spawn((resource(),));
		}
		self.resource::<R>().unwrap()
	}

	fn resource_or_insert<R: Component>(&mut self, resource: R) -> &mut R {
		self.resource_or_insert_with(|| resource)
	}

	fn resource_or_default<R: Component + Default>(&mut self) -> &mut R {
		self.resource_or_insert_with(|| R::default())
	}
}
