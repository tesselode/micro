use std::ops::{Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive};

pub trait IntoRange {
	fn into_range(self, len: u32) -> Option<(u32, u32)>;
}

impl IntoRange for (u32, u32) {
	fn into_range(self, _len: u32) -> Option<(u32, u32)> {
		Some(self)
	}
}

impl IntoRange for Option<(u32, u32)> {
	fn into_range(self, _len: u32) -> Option<(u32, u32)> {
		self
	}
}

impl IntoRange for RangeFull {
	fn into_range(self, _len: u32) -> Option<(u32, u32)> {
		None
	}
}

impl IntoRange for Range<u32> {
	fn into_range(self, _len: u32) -> Option<(u32, u32)> {
		Some((self.start, self.end))
	}
}

impl IntoRange for RangeInclusive<u32> {
	fn into_range(self, _len: u32) -> Option<(u32, u32)> {
		Some((*self.start(), *self.end() + 1))
	}
}

impl IntoRange for RangeFrom<u32> {
	fn into_range(self, len: u32) -> Option<(u32, u32)> {
		Some((self.start, len))
	}
}

impl IntoRange for RangeTo<u32> {
	fn into_range(self, _len: u32) -> Option<(u32, u32)> {
		Some((0, self.end))
	}
}

impl IntoRange for RangeToInclusive<u32> {
	fn into_range(self, _len: u32) -> Option<(u32, u32)> {
		Some((0, self.end + 1))
	}
}
