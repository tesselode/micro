use std::hash::Hash;

pub trait VirtualControls: Sized + Hash + Eq + Copy + 'static {
	const ALL: &'static [Self];
}

pub trait VirtualAnalogSticks<C: VirtualControls>: Sized + Hash + Eq + Copy + 'static {
	const ALL: &'static [Self];

	fn controls(&self) -> VirtualAnalogStickControls<C>;
}

impl<C: VirtualControls> VirtualAnalogSticks<C> for () {
	const ALL: &'static [Self] = &[];

	fn controls(&self) -> VirtualAnalogStickControls<C> {
		unreachable!()
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VirtualAnalogStickControls<C: VirtualControls> {
	pub left: C,
	pub right: C,
	pub up: C,
	pub down: C,
}
