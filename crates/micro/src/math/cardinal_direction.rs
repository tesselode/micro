use std::f32::consts::{FRAC_PI_2, PI};

use glam::{IVec2, Vec2};

use super::ClockDirection;

/// A non-diagonal direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serializing", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "exhaust", derive(exhaust::Exhaust))]
pub enum CardinalDirection {
	/// West or negative X in the default coordinate system.
	Left,
	/// East or positive X in the default coordinate system.
	Right,
	/// North or negative Y in the default coordinate system.
	Up,
	/// South or positive Y in the default coordinate system.
	Down,
}

impl CardinalDirection {
	/// Returns the direction as a unit float vector in the default coordinate system.
	pub fn as_vec2(self) -> Vec2 {
		match self {
			CardinalDirection::Left => Vec2::new(-1.0, 0.0),
			CardinalDirection::Right => Vec2::new(1.0, 0.0),
			CardinalDirection::Up => Vec2::new(0.0, -1.0),
			CardinalDirection::Down => Vec2::new(0.0, 1.0),
		}
	}

	/// Returns the direction as a unit integer vector in the default coordinate system.
	pub fn as_ivec2(self) -> IVec2 {
		match self {
			CardinalDirection::Left => IVec2::new(-1, 0),
			CardinalDirection::Right => IVec2::new(1, 0),
			CardinalDirection::Up => IVec2::new(0, -1),
			CardinalDirection::Down => IVec2::new(0, 1),
		}
	}

	/// Returns an angle (in radians), which, when passed into [`Vec2::from_angle`],
	/// returns a unit vector in the default coordinate system.
	pub fn as_angle(self) -> f32 {
		match self {
			CardinalDirection::Left => PI,
			CardinalDirection::Right => 0.0,
			CardinalDirection::Up => -FRAC_PI_2,
			CardinalDirection::Down => FRAC_PI_2,
		}
	}

	/// Returns `true` if the direction is left or right, false otherwise.
	pub fn is_horizontal(self) -> bool {
		match self {
			CardinalDirection::Left => true,
			CardinalDirection::Right => true,
			CardinalDirection::Up => false,
			CardinalDirection::Down => false,
		}
	}

	/// Returns `true` if the direction is up or down, false otherwise.
	pub fn is_vertical(self) -> bool {
		match self {
			CardinalDirection::Left => false,
			CardinalDirection::Right => false,
			CardinalDirection::Up => true,
			CardinalDirection::Down => true,
		}
	}

	/// Returns the direction rotated clockwise or counter-clockwise.
	pub fn rotated(self, direction: ClockDirection) -> Self {
		match direction {
			ClockDirection::Clockwise => match self {
				Self::Left => Self::Up,
				Self::Right => Self::Down,
				Self::Up => Self::Right,
				Self::Down => Self::Left,
			},
			ClockDirection::CounterClockwise => match self {
				Self::Left => Self::Down,
				Self::Right => Self::Up,
				Self::Up => Self::Left,
				Self::Down => Self::Right,
			},
		}
	}
}

#[cfg(feature = "rand")]
impl rand::distr::Distribution<CardinalDirection> for rand::distr::StandardUniform {
	fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> CardinalDirection {
		match rng.random_range(0..4) {
			0 => CardinalDirection::Left,
			1 => CardinalDirection::Right,
			2 => CardinalDirection::Up,
			3 => CardinalDirection::Down,
			_ => unreachable!(),
		}
	}
}
