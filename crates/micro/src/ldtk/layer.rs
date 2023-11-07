use glam::{IVec2, UVec2};
use serde::Deserialize;

use super::{Entity, Tile};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
#[serde(transparent)]
pub struct LayerId(pub String);

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(from = "RawLayer")]
pub struct Layer {
	pub id: LayerId,
	pub name: String,
	pub grid_size: UVec2,
	pub cell_size: u32,
	pub opacity: f32,
	pub pixel_offset: IVec2,
	pub total_pixel_offset: IVec2,
	pub visible: bool,
	pub kind: LayerKind,
}

impl From<RawLayer> for Layer {
	fn from(
		RawLayer {
			identifier,
			c_wid,
			c_hei,
			grid_size,
			opacity,
			px_total_offset_x,
			px_total_offset_y,
			iid,
			px_offset_x,
			px_offset_y,
			visible,
			kind,
		}: RawLayer,
	) -> Self {
		Self {
			id: iid,
			name: identifier,
			grid_size: UVec2::new(c_wid, c_hei),
			cell_size: grid_size,
			opacity,
			pixel_offset: IVec2::new(px_offset_x, px_offset_y),
			total_pixel_offset: IVec2::new(px_total_offset_x, px_total_offset_y),
			visible,
			kind,
		}
	}
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(tag = "__type")]
pub enum LayerKind {
	IntGrid {
		#[serde(rename = "intGridCsv")]
		values: Vec<u32>,
		#[serde(rename = "autoLayerTiles")]
		auto_tiles: Vec<Tile>,
	},
	Tiles {
		#[serde(rename = "gridTiles")]
		tiles: Vec<Tile>,
	},
	Entities {
		#[serde(rename = "entityInstances")]
		entities: Vec<Entity>,
	},
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct RawLayer {
	#[serde(rename = "__identifier")]
	identifier: String,
	#[serde(rename = "__cWid")]
	c_wid: u32,
	#[serde(rename = "__cHei")]
	c_hei: u32,
	#[serde(rename = "__gridSize")]
	grid_size: u32,
	#[serde(rename = "__opacity")]
	opacity: f32,
	#[serde(rename = "__pxTotalOffsetX")]
	px_total_offset_x: i32,
	#[serde(rename = "__pxTotalOffsetY")]
	px_total_offset_y: i32,
	iid: LayerId,
	px_offset_x: i32,
	px_offset_y: i32,
	visible: bool,
	#[serde(flatten)]
	kind: LayerKind,
}
