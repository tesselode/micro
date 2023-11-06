use serde::Deserialize;

use crate::EntityRef;

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Field {
	#[serde(rename = "__identifier")]
	name: String,
	#[serde(flatten)]
	kind: FieldKind,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(tag = "__type")]
pub enum FieldKind {
	Float {
		#[serde(rename = "__value")]
		value: f32,
	},
	#[serde(rename = "Array<EntityRef>")]
	EntityRefs {
		#[serde(rename = "__value")]
		value: Vec<EntityRef>,
	},
}
