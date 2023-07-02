use std::ops::{Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OffsetAndCount<T> {
	pub offset: T,
	pub count: T,
}

pub trait IntoOffsetAndCount<T> {
	fn into_offset_and_count(self, total_count: T) -> OffsetAndCount<T>;
}

impl<T> IntoOffsetAndCount<T> for OffsetAndCount<T> {
	fn into_offset_and_count(self, _total_count: T) -> OffsetAndCount<T> {
		self
	}
}

impl IntoOffsetAndCount<u32> for Range<u32> {
	fn into_offset_and_count(self, _total_count: u32) -> OffsetAndCount<u32> {
		OffsetAndCount {
			offset: self.start,
			count: self.end - self.start,
		}
	}
}

impl IntoOffsetAndCount<u32> for RangeFrom<u32> {
	fn into_offset_and_count(self, total_count: u32) -> OffsetAndCount<u32> {
		OffsetAndCount {
			offset: self.start,
			count: total_count - self.start,
		}
	}
}

impl IntoOffsetAndCount<u32> for RangeFull {
	fn into_offset_and_count(self, total_count: u32) -> OffsetAndCount<u32> {
		OffsetAndCount {
			offset: 0,
			count: total_count,
		}
	}
}

impl IntoOffsetAndCount<u32> for RangeInclusive<u32> {
	fn into_offset_and_count(self, _total_count: u32) -> OffsetAndCount<u32> {
		OffsetAndCount {
			offset: *self.start(),
			count: *self.end() - *self.start() + 1,
		}
	}
}

impl IntoOffsetAndCount<u32> for RangeTo<u32> {
	fn into_offset_and_count(self, _total_count: u32) -> OffsetAndCount<u32> {
		OffsetAndCount {
			offset: 0,
			count: self.end,
		}
	}
}

impl IntoOffsetAndCount<u32> for RangeToInclusive<u32> {
	fn into_offset_and_count(self, _total_count: u32) -> OffsetAndCount<u32> {
		OffsetAndCount {
			offset: 0,
			count: self.end + 1,
		}
	}
}

impl IntoOffsetAndCount<usize> for Range<usize> {
	fn into_offset_and_count(self, _total_count: usize) -> OffsetAndCount<usize> {
		OffsetAndCount {
			offset: self.start,
			count: self.end - self.start,
		}
	}
}

impl IntoOffsetAndCount<usize> for RangeFrom<usize> {
	fn into_offset_and_count(self, total_count: usize) -> OffsetAndCount<usize> {
		OffsetAndCount {
			offset: self.start,
			count: total_count - self.start,
		}
	}
}

impl IntoOffsetAndCount<usize> for RangeFull {
	fn into_offset_and_count(self, total_count: usize) -> OffsetAndCount<usize> {
		OffsetAndCount {
			offset: 0,
			count: total_count,
		}
	}
}

impl IntoOffsetAndCount<usize> for RangeInclusive<usize> {
	fn into_offset_and_count(self, _total_count: usize) -> OffsetAndCount<usize> {
		OffsetAndCount {
			offset: *self.start(),
			count: *self.end() - *self.start() + 1,
		}
	}
}

impl IntoOffsetAndCount<usize> for RangeTo<usize> {
	fn into_offset_and_count(self, _total_count: usize) -> OffsetAndCount<usize> {
		OffsetAndCount {
			offset: 0,
			count: self.end,
		}
	}
}

impl IntoOffsetAndCount<usize> for RangeToInclusive<usize> {
	fn into_offset_and_count(self, _total_count: usize) -> OffsetAndCount<usize> {
		OffsetAndCount {
			offset: 0,
			count: self.end + 1,
		}
	}
}
