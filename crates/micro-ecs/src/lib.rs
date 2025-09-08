mod entities;
mod events;
mod systems;
mod triggers;

pub use entities::*;
pub use events::*;

#[cfg(test)]
mod test;

use crate::{systems::Systems, triggers::Triggers};

pub type System<Context> = fn(&Context, &mut Entities, &mut Events);

pub type Trigger<Event> = fn(&mut Entities, &mut Events, &Event);

pub struct World {
	entities: Entities,
	systems: Systems,
	events: Events,
	triggers: Triggers,
}

impl World {
	pub fn new() -> Self {
		Self {
			entities: Entities::new(),
			systems: Systems::new(),
			events: Events::new(),
			triggers: Triggers::new(),
		}
	}

	pub fn with_system<Context: 'static>(mut self, system: System<Context>) -> Self {
		self.systems.get_mut().push(system);
		self
	}

	pub fn with_trigger<Event: 'static>(mut self, trigger: Trigger<Event>) -> Self {
		self.triggers.add(trigger);
		self
	}

	pub fn run_systems<Context: 'static>(&mut self, ctx: &Context) {
		for system in self.systems.get_mut() {
			system(ctx, &mut self.entities, &mut self.events);
		}
		self.dispatch_events();
	}

	pub fn emit_event<Event: 'static>(&mut self, event: Event) {
		self.events.emit(event);
	}

	pub fn dispatch_events(&mut self) {
		loop {
			if self.events.is_empty() {
				break;
			}
			let mut old_events = std::mem::replace(&mut self.events, Events::new());
			for (type_id, events) in &mut old_events.0 {
				let Some(triggers) = self.triggers.0.get_mut(type_id) else {
					continue;
				};
				triggers.run(events.as_mut(), &mut self.entities, &mut self.events);
			}
		}
	}
}

impl Default for World {
	fn default() -> Self {
		Self::new()
	}
}
