use std::collections::HashMap;

use glam::Vec2;

use super::{RealControl, VirtualControls};

#[derive(Debug, Clone)]
pub struct VirtualControllerConfig<C: VirtualControls> {
	pub control_mapping: HashMap<C, Vec<RealControl>>,
	pub deadzone: f32,
	pub deadzone_shape: DeadzoneShape,
}

impl<C: VirtualControls + Default> Default for VirtualControllerConfig<C> {
	fn default() -> Self {
		Self {
			control_mapping: Default::default(),
			deadzone: 0.5,
			deadzone_shape: Default::default(),
		}
	}
}

pub enum DeadzoneShape {
	Circle,
	Square,
}

impl DeadzoneShape {
	pub(super) fn apply_deadzone(&self, input: Vec2, deadzone: f32) -> Vec2 {
		match self {
			DeadzoneShape::Circle => {
				if input.length() >= deadzone {
					input
				} else {
					Vec2::ZERO
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
