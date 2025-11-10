use std::ops::{Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive};

/// A trait for types that can be used as ranges of vertex indices.
pub trait IntoIndexRange {
	/// Given the total number of indices, converts `self` into
	/// a min and max vertex index, or `None` for the full range
	/// of indices.
	fn into_index_range(self, len: u32) -> Option<(u32, u32)>;
}

impl IntoIndexRange for (u32, u32) {
	fn into_index_range(self, _len: u32) -> Option<(u32, u32)> {
		Some(self)
	}
}

impl IntoIndexRange for Option<(u32, u32)> {
	fn into_index_range(self, _len: u32) -> Option<(u32, u32)> {
		self
	}
}

impl IntoIndexRange for RangeFull {
	fn into_index_range(self, _len: u32) -> Option<(u32, u32)> {
		None
	}
}

impl IntoIndexRange for Range<u32> {
	fn into_index_range(self, _len: u32) -> Option<(u32, u32)> {
		Some((self.start, self.end))
	}
}

impl IntoIndexRange for RangeInclusive<u32> {
	fn into_index_range(self, _len: u32) -> Option<(u32, u32)> {
		Some((*self.start(), *self.end() + 1))
	}
}

impl IntoIndexRange for RangeFrom<u32> {
	fn into_index_range(self, len: u32) -> Option<(u32, u32)> {
		Some((self.start, len))
	}
}

impl IntoIndexRange for RangeTo<u32> {
	fn into_index_range(self, _len: u32) -> Option<(u32, u32)> {
		Some((0, self.end))
	}
}

impl IntoIndexRange for RangeToInclusive<u32> {
	fn into_index_range(self, _len: u32) -> Option<(u32, u32)> {
		Some((0, self.end + 1))
	}
}
