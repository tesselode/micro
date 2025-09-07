#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serializing", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "exhaust", derive(exhaust::Exhaust))]
pub enum ClockDirection {
	Clockwise,
	CounterClockwise,
}

impl ClockDirection {
	pub fn as_f32(self) -> f32 {
		match self {
			ClockDirection::Clockwise => -1.0,
			ClockDirection::CounterClockwise => 1.0,
		}
	}
}

#[cfg(feature = "rand")]
impl rand::distr::Distribution<ClockDirection> for rand::distr::StandardUniform {
	fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> ClockDirection {
		match rng.random_range(0..2) {
			0 => ClockDirection::Clockwise,
			1 => ClockDirection::CounterClockwise,
			_ => unreachable!(),
		}
	}
}
