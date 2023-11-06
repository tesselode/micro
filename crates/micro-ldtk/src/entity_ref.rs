use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
pub struct EntityRef {
	#[serde(rename = "entityIid")]
	pub entity_id: String,
	#[serde(rename = "layerIid")]
	pub layer_id: String,
	#[serde(rename = "levelIid")]
	pub level_id: String,
	#[serde(rename = "worldIid")]
	pub world_id: String,
}
