use glam::{vec2, Vec2};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Sizing {
	pub horizontal: AxisSizing,
	pub vertical: AxisSizing,
}

impl Sizing {
	pub const MIN: Self = Self {
		horizontal: AxisSizing::Shrink,
		vertical: AxisSizing::Shrink,
	};
	pub const MAX: Self = Self {
		horizontal: AxisSizing::Expand,
		vertical: AxisSizing::Expand,
	};

	pub fn allotted_size_for_children(self, allotted_size_from_parent: Vec2) -> Vec2 {
		vec2(
			self.horizontal
				.allotted_size_for_children(allotted_size_from_parent.x),
			self.vertical
				.allotted_size_for_children(allotted_size_from_parent.y),
		)
	}

	pub fn final_parent_size(
		self,
		allotted_size_from_parent: Vec2,
		child_sizes: impl IntoIterator<Item = Vec2>,
	) -> Vec2 {
		let child_max_size = child_sizes
			.into_iter()
			.reduce(Vec2::max)
			.unwrap_or_default();
		vec2(
			match self.horizontal {
				AxisSizing::Shrink => child_max_size.x,
				AxisSizing::Expand => allotted_size_from_parent.x,
				AxisSizing::Max(size) => size.min(allotted_size_from_parent.x),
				AxisSizing::Fractional(fraction) => fraction * allotted_size_from_parent.x,
			},
			match self.vertical {
				AxisSizing::Shrink => child_max_size.y,
				AxisSizing::Expand => allotted_size_from_parent.y,
				AxisSizing::Max(size) => size.min(allotted_size_from_parent.y),
				AxisSizing::Fractional(fraction) => fraction * allotted_size_from_parent.y,
			},
		)
	}
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AxisSizing {
	Shrink,
	Expand,
	Max(f32),
	Fractional(f32),
}

impl AxisSizing {
	pub fn allotted_size_for_children(self, allotted_size_from_parent: f32) -> f32 {
		match self {
			AxisSizing::Shrink => allotted_size_from_parent,
			AxisSizing::Expand => allotted_size_from_parent,
			AxisSizing::Max(size) => size.min(allotted_size_from_parent),
			AxisSizing::Fractional(fraction) => allotted_size_from_parent * fraction,
		}
	}

	pub fn final_parent_size(
		self,
		allotted_size_from_parent: f32,
		child_sizes: impl Iterator<Item = f32>,
	) -> f32 {
		match self {
			AxisSizing::Shrink => child_sizes.reduce(f32::max).unwrap_or_default(),
			AxisSizing::Expand => allotted_size_from_parent,
			AxisSizing::Max(size) => size.min(allotted_size_from_parent),
			AxisSizing::Fractional(fraction) => allotted_size_from_parent * fraction,
		}
	}
}
