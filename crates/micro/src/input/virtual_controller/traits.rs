use std::hash::Hash;

use crate::math::CardinalDirection;

pub trait VirtualControls: Sized + Hash + Eq + Copy + 'static {
	const ALL: &'static [Self];
}

pub trait VirtualAnalogSticks<C: VirtualControls>: Sized + Hash + Eq + Copy + 'static {
	const ALL: &'static [Self];

	fn controls(&self) -> fn(CardinalDirection) -> C;
}

impl<C: VirtualControls> VirtualAnalogSticks<C> for () {
	const ALL: &'static [Self] = &[];

	fn controls(&self) -> fn(CardinalDirection) -> C {
		unreachable!()
	}
}
