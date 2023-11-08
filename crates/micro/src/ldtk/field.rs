use serde::Deserialize;

use super::EntityRef;

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Field {
	#[serde(rename = "__identifier")]
	pub name: String,
	#[serde(flatten)]
	pub kind: FieldKind,
}

impl Field {
	pub fn float(&self) -> Option<f32> {
		match &self.kind {
			FieldKind::Float { value } => Some(*value),
			_ => None,
		}
	}

	pub fn entities(&self) -> Option<&[EntityRef]> {
		match &self.kind {
			FieldKind::Entities { value } => Some(value),
			_ => None,
		}
	}
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(tag = "__type")]
pub enum FieldKind {
	Float {
		#[serde(rename = "__value")]
		value: f32,
	},
	#[serde(rename = "Array<EntityRef>")]
	Entities {
		#[serde(rename = "__value")]
		value: Vec<EntityRef>,
	},
}
