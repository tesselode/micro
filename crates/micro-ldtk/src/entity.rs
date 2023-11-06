use glam::{IVec2, UVec2, Vec2};
use serde::Deserialize;

use crate::Error;

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(try_from = "RawEntity")]
pub struct Entity {
	pub id: String,
	pub name: String,
	pub grid_position: IVec2,
	pub position: IVec2,
	pub world_position: IVec2,
	pub size: UVec2,
	pub pivot: Vec2,
}

impl TryFrom<RawEntity> for Entity {
	type Error = Error;

	fn try_from(
		RawEntity {
			grid,
			identifier,
			pivot,
			world_x,
			world_y,
			height,
			iid,
			px,
			width,
		}: RawEntity,
	) -> Result<Self, Self::Error> {
		Ok(Self {
			id: iid,
			name: identifier,
			grid_position: grid.into(),
			position: px.into(),
			world_position: IVec2::new(world_x, world_y),
			size: UVec2::new(width, height),
			pivot: pivot.into(),
		})
	}
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct RawEntity {
	#[serde(rename = "__grid")]
	grid: [i32; 2],
	#[serde(rename = "__identifier")]
	identifier: String,
	#[serde(rename = "__pivot")]
	pivot: [f32; 2],
	#[serde(rename = "__worldX")]
	world_x: i32,
	#[serde(rename = "__worldY")]
	world_y: i32,
	height: u32,
	iid: String,
	px: [i32; 2],
	width: u32,
}
