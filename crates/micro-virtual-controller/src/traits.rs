use std::hash::Hash;

use exhaust::Exhaust;
use micro::math::CardinalDirection;

pub trait VirtualAnalogSticks<C>: Sized + Hash + Eq + Copy + Exhaust + 'static
where
	C: Sized + Hash + Eq + Copy + Exhaust + 'static,
{
	fn controls(&self) -> fn(CardinalDirection) -> C;
}

impl<C> VirtualAnalogSticks<C> for ()
where
	C: Sized + Hash + Eq + Copy + Exhaust + 'static,
{
	fn controls(&self) -> fn(CardinalDirection) -> C {
		unreachable!()
	}
}
