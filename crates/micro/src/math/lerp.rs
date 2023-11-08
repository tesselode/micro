use std::ops::{Add, Div, Mul, Sub};

pub trait Lerp {
	fn lerp(self, other: Self, f: f32) -> Self;

	fn inverse_lerp(self, min: Self, max: Self) -> f32;
}

impl<T> Lerp for T
where
	T: Copy,
	T: Add<Output = T>,
	T: Sub<Output = T>,
	T: Mul<f32, Output = T>,
	T: Div<T, Output = f32>,
{
	fn lerp(self, other: Self, f: f32) -> Self {
		self + (other - self) * f
	}

	fn inverse_lerp(self, min: Self, max: Self) -> f32 {
		(self - min) / (max - min)
	}
}
