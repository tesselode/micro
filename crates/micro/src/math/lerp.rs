use std::{
	ops::{Add, Mul, Sub},
	time::Duration,
};

use super::{Circle, Rect};

/// A trait for values that can be linearly interpolated to another value.
pub trait Lerp {
	/// Returns the value `f` of the way from `self` to `other`, where an
	/// `f` of `0.0` yields `self`, an `f` of `1.0` yields `other`, and an
	/// `f` of `0.5` yields a value halfway between `self` and `other`.
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

impl Lerp for Circle {
	fn lerp(self, other: Self, f: f32) -> Self {
		Self {
			center: self.center.lerp(other.center, f),
			radius: self.radius.lerp(other.radius, f),
		}
	}
}

impl Lerp for Rect {
	fn lerp(self, other: Self, f: f32) -> Self {
		Self {
			top_left: self.top_left.lerp(other.top_left, f),
			size: self.size.lerp(other.size, f),
		}
	}
}

/// A trait for values that can have inverse lerps performed.
pub trait InverseLerp {
	/// Returns a value from `0.0` to `1.0` which represents how
	/// far `self` is along an imaginary line from `start` to `end`.
	fn inverse_lerp(self, start: Self, end: Self) -> f32;
}

impl InverseLerp for f32 {
	fn inverse_lerp(self, start: Self, end: Self) -> f32 {
		(self - start) / (end - start)
	}
}

impl InverseLerp for f64 {
	fn inverse_lerp(self, start: Self, end: Self) -> f32 {
		((self - start) / (end - start)) as f32
	}
}

impl InverseLerp for Duration {
	fn inverse_lerp(self, start: Self, end: Self) -> f32 {
		self.as_secs_f32()
			.inverse_lerp(start.as_secs_f32(), end.as_secs_f32())
	}
}
