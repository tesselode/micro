use glam::UVec2;

/// A trait for 2D coordinates that can be converted to and from
/// an index into a 1-dimensional array.
///
/// This is useful for indexing into 2D grids represented by a 1D array.
pub trait ToIndex {
	/// A type that can represent the width of the grid.
	type Width;

	/// Converts the coordinates into an index.
	fn to_index(self, width: Self::Width) -> usize;

	/// Converts an index into coordinates.
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
