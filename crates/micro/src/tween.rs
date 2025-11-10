mod tween_sequence;

pub use tween_sequence::*;

use std::f32::consts::PI;

use crate::math::Lerp;

/// Represents the curve of an animation from one value to another.
///
/// Most of these are [Robert Penner's easing curves](https://easings.net/).
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serializing", derive(serde::Serialize, serde::Deserialize))]
#[allow(missing_docs)]
pub enum Easing {
	Linear,
	InSine,
	OutSine,
	InOutSine,
	/// Corresponds to InQuad, InCubic, InQuart, etc. Faster to calculate
	/// than InPowf.
	InPowi(i32),
	/// Corresponds to OutQuad, OutCubic, OutQuart, etc. Faster to calculate
	/// than OutPowf.
	OutPowi(i32),
	/// Corresponds to InOutQuad, InOutCubic, InOutQuart, etc. Faster to calculate
	/// than InOutPowf.
	InOutPowi(i32),
	/// Corresponds to InQuad, InCubic, InQuart, etc.
	InPowf(f32),
	/// Corresponds to OutQuad, OutCubic, OutQuart, etc.
	OutPowf(f32),
	/// Corresponds to InOutQuad, InOutCubic, InOutQuart, etc.
	InOutPowf(f32),
	InBack {
		overshoot: f32,
	},
	OutBack {
		overshoot: f32,
	},
	InOutBack {
		overshoot: f32,
	},
	/// Blends between two easing curves over time.
	Mix {
		a: Box<Easing>,
		b: Box<Easing>,
		f: Box<Easing>,
	},
}

impl Easing {
	/// Applies the easing curve to a value of a linear animation from
	/// `0.0` to `1.0`.
	pub fn ease(&self, x: f32) -> f32 {
		match self {
			Easing::Linear => x,
			Easing::InSine => 1.0 - ((x * PI) / 2.0).cos(),
			Easing::OutSine => reverse(Easing::InSine, x),
			Easing::InOutSine => in_out(Easing::InSine, x),
			Easing::InPowi(n) => x.powi(*n),
			Easing::OutPowi(n) => reverse(Easing::InPowi(*n), x),
			Easing::InOutPowi(n) => in_out(Easing::InPowi(*n), x),
			Easing::InPowf(n) => x.powf(*n),
			Easing::OutPowf(n) => reverse(Easing::InPowf(*n), x),
			Easing::InOutPowf(n) => in_out(Easing::InPowf(*n), x),
			Easing::InBack { overshoot } => {
				let c3 = overshoot + 1.0;
				c3 * x.powi(3) - overshoot * x.powi(2)
			}
			&Easing::OutBack { overshoot } => reverse(Easing::InBack { overshoot }, x),
			&Easing::InOutBack { overshoot } => in_out(Easing::InBack { overshoot }, x),
			Easing::Mix { a, b, f } => a.ease(x).lerp(b.ease(x), f.ease(x)),
		}
	}

	/// Returns a value of `Easing::InBack` with a reasonable default
	/// overshoot amount.
	pub const fn in_back() -> Self {
		Self::InBack { overshoot: 1.70158 }
	}

	/// Returns a value of `Easing::OutBack` with a reasonable default
	/// overshoot amount.
	pub const fn out_back() -> Self {
		Self::OutBack { overshoot: 1.70158 }
	}

	/// Returns a value of `Easing::Mix`, boxing up the child easings
	/// for you.
	pub fn mix(a: Easing, b: Easing, f: Easing) -> Self {
		Self::Mix {
			a: Box::new(a),
			b: Box::new(b),
			f: Box::new(f),
		}
	}
}

fn reverse(easing: Easing, x: f32) -> f32 {
	1.0 - easing.ease(1.0 - x)
}

fn in_out(easing: Easing, x: f32) -> f32 {
	if x < 0.5 {
		easing.ease(x * 2.0) / 2.0
	} else {
		0.5 + reverse(easing, (x - 0.5) * 2.0) / 2.0
	}
}
