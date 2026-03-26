use std::any::Any;

use micro::Context;

use crate::{Queues, Resources, World};

pub trait Observer<T, Globals>: TypeErasedObserver<Globals> {}

pub trait TypeErasedObserver<Globals> {
	fn run(
		&mut self,
		ctx: &mut Context,
		globals: &mut Globals,
		world: &mut World,
		queues: &mut Queues,
		resources: &mut Resources,
		event: &dyn Any,
	);
}

macro_rules! observer_impl {
	($($t:ident),*) => {
		impl<T: 'static, Globals, $($t: 'static),*> Observer<T, Globals> for fn(&mut Context, &mut Globals, &mut World, &mut Queues, &T, $(&mut $t),*) {}

		impl<T: 'static, Globals, $($t: 'static),*> TypeErasedObserver<Globals> for fn(&mut Context, &mut Globals, &mut World, &mut Queues, &T, $(&mut $t),*) {
			fn run(
				&mut self,
				ctx: &mut Context,
				globals: &mut Globals,
				world: &mut World,
				queues: &mut Queues,
				resources: &mut Resources,
				event: &dyn Any,
			) {
				self(
					ctx,
					globals,
					world,
					queues,
					event.downcast_ref().unwrap(),
					$(&mut *resources.get::<$t>()),*,
				)
			}
		}
	};
}

observer_impl!(T1);
observer_impl!(T1, T2);
observer_impl!(T1, T2, T3);
observer_impl!(T1, T2, T3, T4);
observer_impl!(T1, T2, T3, T4, T5);
observer_impl!(T1, T2, T3, T4, T5, T6);
observer_impl!(T1, T2, T3, T4, T5, T6, T7);
observer_impl!(T1, T2, T3, T4, T5, T6, T7, T8);

impl<T: 'static, Globals> TypeErasedObserver<Globals> for fn(&mut Context, &mut Globals, &T) {
	fn run(
		&mut self,
		ctx: &mut Context,
		globals: &mut Globals,
		_world: &mut World,
		_queues: &mut Queues,
		_resources: &mut Resources,
		event: &dyn Any,
	) {
		self(ctx, globals, event.downcast_ref().unwrap())
	}
}
