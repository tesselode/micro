use std::ops::{Range, RangeInclusive, RangeTo, RangeToInclusive};

pub trait IntoInstanceRange {
	fn into_instance_range(self) -> (u32, u32);
}

impl IntoInstanceRange for RangeTo<u32> {
	fn into_instance_range(self) -> (u32, u32) {
		(0, self.end)
	}
}

impl IntoInstanceRange for RangeToInclusive<u32> {
	fn into_instance_range(self) -> (u32, u32) {
		(0, self.end + 1)
	}
}

impl IntoInstanceRange for Range<u32> {
	fn into_instance_range(self) -> (u32, u32) {
		(self.start, self.end)
	}
}

impl IntoInstanceRange for RangeInclusive<u32> {
	fn into_instance_range(self) -> (u32, u32) {
		(*self.start(), self.end() + 1)
	}
}
