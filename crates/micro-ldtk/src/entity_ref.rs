use serde::Deserialize;

use super::{EntityId, LayerId};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
pub struct EntityRef {
	#[serde(rename = "entityIid")]
	pub entity_id: EntityId,
	#[serde(rename = "layerIid")]
	pub layer_id: LayerId,
}
