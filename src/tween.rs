use std::{
	f32::consts::PI,
	ops::{Add, Mul, Sub},
};

pub trait Tweenable {
	fn lerp(self, other: Self, f: f32) -> Self;
}

impl<T> Tweenable for T
where
	T: Copy,
	T: Add<Output = T>,
	T: Sub<Output = T>,
	T: Mul<f32, Output = T>,
{
	fn lerp(self, other: Self, f: f32) -> Self {
		self + (other - self) * f
	}
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Easing {
	Linear,
	InSine,
	OutSine,
	InOutSine,
	InPowi(i32),
	OutPowi(i32),
	InOutPowi(i32),
	InPowf(f32),
	OutPowf(f32),
	InOutPowf(f32),
}

impl Easing {
	pub fn ease(self, x: f32) -> f32 {
		match self {
			Easing::Linear => x,
			Easing::InSine => 1.0 - ((x * PI) / 2.0).cos(),
			Easing::OutSine => ((x * PI) / 2.0).sin(),
			Easing::InOutSine => -((x * PI).cos() - 1.0) / 2.0,
			Easing::InPowi(n) => x.powi(n),
			Easing::OutPowi(n) => 1.0 - (1.0 - x).powi(n),
			Easing::InOutPowi(n) => {
				if x < 0.5 {
					2.0 * x.powi(n)
				} else {
					1.0 - (-2.0 * x + 2.0).powi(n) / 2.0
				}
			}
			Easing::InPowf(n) => x.powf(n),
			Easing::OutPowf(n) => 1.0 - (1.0 - x).powf(n),
			Easing::InOutPowf(n) => {
				if x < 0.5 {
					2.0 * x.powf(n)
				} else {
					1.0 - (-2.0 * x + 2.0).powf(n) / 2.0
				}
			}
		}
	}
}
