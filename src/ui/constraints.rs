use glam::Vec2;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Constraints {
	pub min_size: Vec2,
	pub max_size: Vec2,
}

impl Constraints {
	pub fn min_only(min_size: Vec2) -> Self {
		Self {
			min_size,
			max_size: Vec2::splat(f32::INFINITY),
		}
	}

	pub fn max_only(max_size: Vec2) -> Self {
		Self {
			min_size: Vec2::ZERO,
			max_size,
		}
	}

	pub fn union(&self, other: Constraints) -> Self {
		Self {
			min_size: self.min_size.max(other.min_size),
			max_size: self.max_size.min(other.max_size),
		}
	}
}
