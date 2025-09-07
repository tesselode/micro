use glam::UVec2;

pub trait ToIndex {
	type Width;

	fn to_index(self, width: Self::Width) -> usize;

	fn from_index(index: usize, width: Self::Width) -> Self;
}

impl ToIndex for UVec2 {
	type Width = u32;

	fn to_index(self, width: u32) -> usize {
		(self.y * width + self.x)
			.try_into()
			.expect("could not convert u32 to usize")
	}

	fn from_index(index: usize, width: u32) -> Self {
		let index: u32 = index.try_into().expect("could not convert usize to u32");
		Self::new(index % width, index / width)
	}
}
