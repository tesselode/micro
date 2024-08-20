#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Axis {
	Horizontal,
	Vertical,
}

impl Axis {
	/// Returns `true` if the axis is [`Horizontal`].
	///
	/// [`Horizontal`]: Axis::Horizontal
	#[must_use]
	pub fn is_horizontal(&self) -> bool {
		matches!(self, Self::Horizontal)
	}

	/// Returns `true` if the axis is [`Vertical`].
	///
	/// [`Vertical`]: Axis::Vertical
	#[must_use]
	pub fn is_vertical(&self) -> bool {
		matches!(self, Self::Vertical)
	}
}
