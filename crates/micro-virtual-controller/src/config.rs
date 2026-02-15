use std::{collections::HashMap, hash::Hash};

use exhaust::Exhaust;
use micro::math::Vec2;

use super::RealControl;

#[derive(Debug, Clone, Default)]
pub struct VirtualControllerConfig<C: Sized + Hash + Eq + Copy + Exhaust + 'static> {
	pub control_mapping: HashMap<C, Vec<RealControl>>,
	pub deadzone: f32,
	pub deadzone_shape: DeadzoneShape,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum DeadzoneShape {
	#[default]
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
