use std::f32::consts::{FRAC_PI_2, PI};

use exhaust::Exhaust;
use glam::{IVec2, Vec2};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Exhaust)]
#[cfg_attr(feature = "serializing", derive(serde::Serialize, serde::Deserialize))]
pub enum CardinalDirection {
	Left,
	Right,
	Up,
	Down,
}

impl CardinalDirection {
	pub fn as_vec2(self) -> Vec2 {
		match self {
			CardinalDirection::Left => Vec2::new(-1.0, 0.0),
			CardinalDirection::Right => Vec2::new(1.0, 0.0),
			CardinalDirection::Up => Vec2::new(0.0, -1.0),
			CardinalDirection::Down => Vec2::new(0.0, 1.0),
		}
	}

	pub fn as_ivec2(self) -> IVec2 {
		match self {
			CardinalDirection::Left => IVec2::new(-1, 0),
			CardinalDirection::Right => IVec2::new(1, 0),
			CardinalDirection::Up => IVec2::new(0, -1),
			CardinalDirection::Down => IVec2::new(0, 1),
		}
	}

	pub fn as_angle(self) -> f32 {
		match self {
			CardinalDirection::Left => PI,
			CardinalDirection::Right => 0.0,
			CardinalDirection::Up => -FRAC_PI_2,
			CardinalDirection::Down => FRAC_PI_2,
		}
	}

	pub fn is_horizontal(self) -> bool {
		match self {
			CardinalDirection::Left => true,
			CardinalDirection::Right => true,
			CardinalDirection::Up => false,
			CardinalDirection::Down => false,
		}
	}

	pub fn is_vertical(self) -> bool {
		match self {
			CardinalDirection::Left => false,
			CardinalDirection::Right => false,
			CardinalDirection::Up => true,
			CardinalDirection::Down => true,
		}
	}
}

#[cfg(feature = "rand")]
impl rand::distributions::Distribution<CardinalDirection> for rand::distributions::Standard {
	fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> CardinalDirection {
		match rng.gen_range(0..4) {
			0 => CardinalDirection::Left,
			1 => CardinalDirection::Right,
			2 => CardinalDirection::Up,
			3 => CardinalDirection::Down,
			_ => unreachable!(),
		}
	}
}
