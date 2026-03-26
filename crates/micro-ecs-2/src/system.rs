use micro::Context;

use crate::{Ecs, Queues, World};

pub trait System<Globals> {
	fn run(&mut self, ctx: &mut Context, globals: &mut Globals, ecs: &mut Ecs<Globals>);
}

impl<F, Globals, M> System<Globals> for F
where
	F: SystemFunction<Globals, M>,
{
	fn run(&mut self, ctx: &mut Context, globals: &mut Globals, ecs: &mut Ecs<Globals>) {
		self.run_fn(ctx, globals, ecs, M);
	}
}

trait SystemFunction<Globals, T> {
	fn run_fn(
		&mut self,
		ctx: &mut Context,
		globals: &mut Globals,
		ecs: &mut Ecs<Globals>,
		dummy: T,
	);
}

macro_rules! system_impl {
	($m:ident, $($t:ident),*) => {
		#[derive(Default)]
		struct $m;

		impl<F, Globals, $($t: 'static),*> SystemFunction<Globals, $m>
			for F
		where
			F: FnMut(&mut Context, &mut Globals, &mut World, &mut Queues, $(&mut $t),*)
		{
			fn run_fn(&mut self, ctx: &mut Context, globals: &mut Globals, ecs: &mut Ecs<Globals>, dummy: $m) {
				self(ctx, globals, &mut ecs.world, &mut ecs.queues, $(&mut *ecs.resources.get::<$t>()),*)
			}
		}
	};
}

system_impl!(M1, T1);
system_impl!(M2, T1, T2);
system_impl!(M3, T1, T2, T3);
system_impl!(M4, T1, T2, T3, T4);
system_impl!(M5, T1, T2, T3, T4, T5);
system_impl!(M6, T1, T2, T3, T4, T5, T6);
system_impl!(M7, T1, T2, T3, T4, T5, T6, T7);
system_impl!(M8, T1, T2, T3, T4, T5, T6, T7, T8);

impl<F, Globals> SystemFunction<Globals, fn(&mut Context, &mut Globals, &mut World, &mut Queues)>
	for F
where
	F: FnMut(&mut Context, &mut Globals, &mut World, &mut Queues),
{
	fn run_fn(&mut self, ctx: &mut Context, globals: &mut Globals, ecs: &mut Ecs<Globals>) {
		self(ctx, globals, &mut ecs.world, &mut ecs.queues)
	}
}
