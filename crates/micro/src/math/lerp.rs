use std::{
	ops::{Add, Mul, Sub},
	time::Duration,
};

pub trait Lerp {
	fn lerp(self, other: Self, f: f32) -> Self;
}

impl<T> Lerp for T
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

pub trait InverseLerp {
	fn inverse_lerp(self, start: Self, end: Self) -> f32;
}

impl InverseLerp for f32 {
	fn inverse_lerp(self, start: Self, end: Self) -> f32 {
		(self - start) / (end - start)
	}
}

impl InverseLerp for Duration {
	fn inverse_lerp(self, start: Self, end: Self) -> f32 {
		self.as_secs_f32()
			.inverse_lerp(start.as_secs_f32(), end.as_secs_f32())
	}
}
