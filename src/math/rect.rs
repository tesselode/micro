#[cfg(test)]
mod test;

use num_traits::{AsPrimitive, Float, MulAdd, Num, NumCast};
use vek::{Mat4, Vec2};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Rect<T: Copy = f32> {
	pub top_left: Vec2<T>,
	pub bottom_right: Vec2<T>,
}

impl<T: Copy> Rect<T> {
	pub fn new(top_left: Vec2<T>, bottom_right: Vec2<T>) -> Self {
		Self {
			top_left,
			bottom_right,
		}
	}

	pub fn from_top_left_and_size(top_left: Vec2<T>, size: Vec2<T>) -> Self
	where
		T: Num,
	{
		Self::new(top_left, top_left + size)
	}

	pub fn xywh(x: T, y: T, width: T, height: T) -> Self
	where
		T: Num,
	{
		Self::new(Vec2::new(x, y), Vec2::new(x + width, y + height))
	}

	pub fn as_<D>(&self) -> Rect<D>
	where
		D: Copy + 'static,
		T: AsPrimitive<D>,
	{
		Rect::new(self.top_left.as_(), self.bottom_right.as_())
	}

	pub fn numcast<D>(&self) -> Option<Rect<D>>
	where
		T: NumCast,
		D: Copy + NumCast,
	{
		self.top_left
			.numcast()
			.zip(self.bottom_right.numcast())
			.map(|(top_left, bottom_right)| Rect::new(top_left, bottom_right))
	}

	pub fn size(&self) -> Vec2<T>
	where
		T: Num,
	{
		self.bottom_right - self.top_left
	}

	pub fn left(&self) -> T {
		self.top_left.x
	}

	pub fn right(&self) -> T {
		self.bottom_right.x
	}

	pub fn top(&self) -> T {
		self.top_left.y
	}

	pub fn bottom(&self) -> T {
		self.bottom_right.y
	}

	pub fn top_right(&self) -> Vec2<T> {
		Vec2::new(self.bottom_right.x, self.top_left.y)
	}

	pub fn bottom_left(&self) -> Vec2<T> {
		Vec2::new(self.top_left.x, self.bottom_right.y)
	}

	pub fn fractional_x(&self, fraction: T) -> T
	where
		T: Float,
	{
		self.left() + (self.right() - self.left()) * fraction
	}

	pub fn fractional_y(&self, fraction: T) -> T
	where
		T: Float,
	{
		self.top() + (self.bottom() - self.top()) * fraction
	}

	pub fn fractional_point(&self, fraction: Vec2<T>) -> Vec2<T>
	where
		T: Float,
	{
		Vec2::new(self.fractional_x(fraction.x), self.fractional_y(fraction.y))
	}

	pub fn corners(&self) -> [Vec2<T>; 4] {
		[
			self.bottom_right,
			self.top_right(),
			self.top_left,
			self.bottom_left(),
		]
	}

	pub fn padded(&self, padding: Vec2<T>) -> Self
	where
		T: Num,
	{
		Self {
			top_left: self.top_left - padding,
			bottom_right: self.bottom_right + padding,
		}
	}

	pub fn union(&self, other: Self) -> Self
	where
		T: PartialOrd,
	{
		Self {
			top_left: Vec2::new(
				min(self.top_left.x, other.top_left.x),
				min(self.top_left.y, other.top_left.y),
			),
			bottom_right: Vec2::new(
				max(self.bottom_right.x, other.bottom_right.x),
				max(self.bottom_right.y, other.bottom_right.y),
			),
		}
	}

	pub fn contains_point(&self, point: Vec2<T>) -> bool
	where
		T: PartialOrd,
	{
		point.x >= self.left()
			&& point.x <= self.right()
			&& point.y >= self.top()
			&& point.y <= self.bottom()
	}

	pub fn overlaps(&self, other: Self) -> bool
	where
		T: PartialOrd,
	{
		self.left() < other.right()
			&& other.left() < self.right()
			&& self.top() < other.bottom()
			&& other.top() < self.bottom()
	}

	pub fn transformed(&self, transform: Mat4<T>) -> Self
	where
		T: Float + MulAdd<Output = T>,
	{
		Self::new(
			transform.mul_point(self.top_left),
			transform.mul_point(self.bottom_right),
		)
	}
}

impl Rect<f32> {
	pub fn center_x(&self) -> f32 {
		self.fractional_x(0.5)
	}

	pub fn center_y(&self) -> f32 {
		self.fractional_y(0.5)
	}

	pub fn center(&self) -> Vec2<f32> {
		self.fractional_point(Vec2::broadcast(0.5))
	}
}

impl Rect<f64> {
	pub fn center_x(&self) -> f64 {
		self.fractional_x(0.5)
	}

	pub fn center_y(&self) -> f64 {
		self.fractional_y(0.5)
	}

	pub fn center(&self) -> Vec2<f64> {
		self.fractional_point(Vec2::broadcast(0.5))
	}
}

fn min<T>(a: T, b: T) -> T
where
	T: PartialOrd,
{
	if a < b {
		a
	} else {
		b
	}
}

fn max<T>(a: T, b: T) -> T
where
	T: PartialOrd,
{
	if a > b {
		a
	} else {
		b
	}
}
