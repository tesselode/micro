#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CrossSizing {
	Min,
	Max,
}

impl CrossSizing {
	/// Returns `true` if the cross sizing is [`Min`].
	///
	/// [`Min`]: CrossSizing::Min
	#[must_use]
	pub fn is_min(&self) -> bool {
		matches!(self, Self::Min)
	}

	/// Returns `true` if the cross sizing is [`Max`].
	///
	/// [`Max`]: CrossSizing::Max
	#[must_use]
	pub fn is_max(&self) -> bool {
		matches!(self, Self::Max)
	}
}
