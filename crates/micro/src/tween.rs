mod tween_sequence;

pub use tween_sequence::*;

use std::f32::consts::PI;

use crate::math::Lerp;

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serializing", derive(serde::Serialize, serde::Deserialize))]
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
	InBack {
		overshoot: f32,
	},
	OutBack {
		overshoot: f32,
	},
	InOutBack {
		overshoot: f32,
	},
	Mix {
		a: Box<Easing>,
		b: Box<Easing>,
		f: Box<Easing>,
	},
}

impl Easing {
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

	pub const fn in_back() -> Self {
		Self::InBack { overshoot: 1.70158 }
	}

	pub const fn out_back() -> Self {
		Self::OutBack { overshoot: 1.70158 }
	}

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
