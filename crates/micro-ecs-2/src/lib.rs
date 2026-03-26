mod observer;
mod queues;
mod resources;
mod schedule;
mod system;

pub use queues::*;
pub use resources::*;
pub use schedule::*;
pub use system::*;

pub use hecs::*;

use std::{any::TypeId, vec::Vec};

use indexmap::IndexMap;
use micro::Context;

use crate::observer::{Observer, TypeErasedObserver};

type Observers<Globals> = Vec<Box<dyn TypeErasedObserver<Globals>>>;

pub struct Ecs<Globals> {
	world: World,
	queues: Queues,
	resources: Resources,
	observers: IndexMap<TypeId, Observers<Globals>>,
}

impl<Globals> Ecs<Globals> {
	pub fn new() -> Self {
		Self {
			world: World::new(),
			queues: Queues::new(),
			resources: Resources::new(),
			observers: IndexMap::new(),
		}
	}

	pub fn observe<T: 'static>(mut self, observer: impl Observer<T, Globals> + 'static) -> Self {
		self.observers
			.entry(TypeId::of::<T>())
			.or_default()
			.push(Box::new(observer));
		self
	}

	pub fn world(&self) -> &World {
		&self.world
	}

	pub fn world_mut(&mut self) -> &mut World {
		&mut self.world
	}

	pub fn queues(&self) -> &Queues {
		&self.queues
	}

	pub fn queues_mut(&mut self) -> &mut Queues {
		&mut self.queues
	}

	pub(crate) fn dispatch_events(&mut self, ctx: &mut Context, globals: &mut Globals) {
		while let Some((type_id, event)) = self.queues.pop_event() {
			let Some(observers) = self.observers.get_mut(&type_id) else {
				continue;
			};
			for observer in observers {
				observer.run(
					ctx,
					globals,
					&mut self.world,
					&mut self.queues,
					&mut self.resources,
					&event,
				);
			}
		}
	}
}

impl<Globals> Default for Ecs<Globals> {
	fn default() -> Self {
		Self::new()
	}
}
