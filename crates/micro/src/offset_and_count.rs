use std::ops::{Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OffsetAndCount {
	pub offset: u32,
	pub count: u32,
}

pub trait IntoOffsetAndCount {
	fn into_offset_and_count(self, total_count: u32) -> OffsetAndCount;
}

impl IntoOffsetAndCount for OffsetAndCount {
	fn into_offset_and_count(self, _total_count: u32) -> OffsetAndCount {
		self
	}
}

impl IntoOffsetAndCount for Range<u32> {
	fn into_offset_and_count(self, _total_count: u32) -> OffsetAndCount {
		OffsetAndCount {
			offset: self.start,
			count: self.end - self.start,
		}
	}
}

impl IntoOffsetAndCount for RangeFrom<u32> {
	fn into_offset_and_count(self, total_count: u32) -> OffsetAndCount {
		OffsetAndCount {
			offset: self.start,
			count: total_count - self.start,
		}
	}
}

impl IntoOffsetAndCount for RangeFull {
	fn into_offset_and_count(self, total_count: u32) -> OffsetAndCount {
		OffsetAndCount {
			offset: 0,
			count: total_count,
		}
	}
}

impl IntoOffsetAndCount for RangeInclusive<u32> {
	fn into_offset_and_count(self, _total_count: u32) -> OffsetAndCount {
		OffsetAndCount {
			offset: *self.start(),
			count: *self.end() - *self.start() + 1,
		}
	}
}

impl IntoOffsetAndCount for RangeTo<u32> {
	fn into_offset_and_count(self, _total_count: u32) -> OffsetAndCount {
		OffsetAndCount {
			offset: 0,
			count: self.end,
		}
	}
}

impl IntoOffsetAndCount for RangeToInclusive<u32> {
	fn into_offset_and_count(self, _total_count: u32) -> OffsetAndCount {
		OffsetAndCount {
			offset: 0,
			count: self.end + 1,
		}
	}
}
