use std::ops::{Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive};

pub trait IntoIndexRange {
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
