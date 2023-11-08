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
			FieldKind::Float { value } => *value,
			_ => None,
		}
	}

	pub fn entity(&self) -> Option<&EntityRef> {
		match &self.kind {
			FieldKind::Entity { value } => value.as_ref(),
			_ => None,
		}
	}

	pub fn entities(&self) -> Option<&[EntityRef]> {
		match &self.kind {
			FieldKind::Entities { value } => value.as_deref(),
			_ => None,
		}
	}
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(tag = "__type")]
pub enum FieldKind {
	Float {
		#[serde(rename = "__value")]
		value: Option<f32>,
	},
	#[serde(rename = "EntityRef")]
	Entity {
		#[serde(rename = "__value")]
		value: Option<EntityRef>,
	},
	#[serde(rename = "Array<EntityRef>")]
	Entities {
		#[serde(rename = "__value")]
		value: Option<Vec<EntityRef>>,
	},
}
