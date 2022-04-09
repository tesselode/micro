use std::collections::HashMap;

use vek::Vec2;

use super::{RealControl, VirtualControls};

#[derive(Debug, Clone, Default)]
pub struct VirtualControllerConfig<C: VirtualControls> {
	pub control_mapping: HashMap<C, Vec<RealControl>>,
	pub deadzone: f32,
	pub deadzone_shape: DeadzoneShape,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DeadzoneShape {
	Circle,
	Square,
}

impl DeadzoneShape {
	pub(super) fn apply_deadzone(&self, input: Vec2<f32>, deadzone: f32) -> Vec2<f32> {
		match self {
			DeadzoneShape::Circle => {
				if input.magnitude() >= deadzone {
					input
				} else {
					Vec2::zero()
				}
			}
			DeadzoneShape::Square => Vec2 {
				x: if input.x.abs() >= deadzone {
					input.x
				} else {
					0.0
				},
				y: if input.y.abs() >= deadzone {
					input.y
				} else {
					0.0
				},
			},
		}
	}
}

impl Default for DeadzoneShape {
	fn default() -> Self {
		Self::Circle
	}
}
