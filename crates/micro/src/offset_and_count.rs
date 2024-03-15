use std::ops::{Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OffsetAndCount {
	pub offset: usize,
	pub count: usize,
}

pub trait IntoOffsetAndCount {
	fn into_offset_and_count(self, total_count: usize) -> Option<OffsetAndCount>;
}

impl IntoOffsetAndCount for OffsetAndCount {
	fn into_offset_and_count(self, _total_count: usize) -> Option<OffsetAndCount> {
		Some(self)
	}
}

impl IntoOffsetAndCount for Option<OffsetAndCount> {
	fn into_offset_and_count(self, _total_count: usize) -> Option<OffsetAndCount> {
		self
	}
}

impl IntoOffsetAndCount for Range<usize> {
	fn into_offset_and_count(self, _total_count: usize) -> Option<OffsetAndCount> {
		Some(OffsetAndCount {
			offset: self.start,
			count: self.end - self.start,
		})
	}
}

impl IntoOffsetAndCount for RangeFrom<usize> {
	fn into_offset_and_count(self, total_count: usize) -> Option<OffsetAndCount> {
		Some(OffsetAndCount {
			offset: self.start,
			count: total_count - self.start,
		})
	}
}

impl IntoOffsetAndCount for RangeFull {
	fn into_offset_and_count(self, total_count: usize) -> Option<OffsetAndCount> {
		Some(OffsetAndCount {
			offset: 0,
			count: total_count,
		})
	}
}

impl IntoOffsetAndCount for RangeInclusive<usize> {
	fn into_offset_and_count(self, _total_count: usize) -> Option<OffsetAndCount> {
		Some(OffsetAndCount {
			offset: *self.start(),
			count: *self.end() - *self.start() + 1,
		})
	}
}

impl IntoOffsetAndCount for RangeTo<usize> {
	fn into_offset_and_count(self, _total_count: usize) -> Option<OffsetAndCount> {
		Some(OffsetAndCount {
			offset: 0,
			count: self.end,
		})
	}
}

impl IntoOffsetAndCount for RangeToInclusive<usize> {
	fn into_offset_and_count(self, _total_count: usize) -> Option<OffsetAndCount> {
		Some(OffsetAndCount {
			offset: 0,
			count: self.end + 1,
		})
	}
}
