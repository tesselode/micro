use micro::Context;

use crate::{Ecs, System};

pub struct Schedule<Globals> {
	systems: Vec<Box<dyn System<Globals>>>,
}

impl<Globals> Schedule<Globals> {
	pub fn new() -> Self {
		Self { systems: vec![] }
	}

	pub fn system(mut self, system: impl System<Globals> + 'static) -> Self {
		self.systems.push(Box::new(system));
		self
	}

	pub fn run(&mut self, ctx: &mut Context, globals: &mut Globals, ecs: &mut Ecs<Globals>) {
		for system in &mut self.systems {
			system.run(ctx, globals, ecs);
		}
		ecs.dispatch_events(ctx, globals);
	}
}

impl<Globals> Default for Schedule<Globals> {
	fn default() -> Self {
		Self::new()
	}
}
