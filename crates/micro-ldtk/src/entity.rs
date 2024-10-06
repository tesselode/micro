use micro::math::{IVec2, UVec2, Vec2};
use serde::Deserialize;

use super::{Error, Field};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
#[serde(transparent)]
pub struct EntityId(pub String);

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(try_from = "RawEntity")]
pub struct Entity {
	pub id: EntityId,
	pub name: String,
	pub tags: Vec<String>,
	pub grid_position: IVec2,
	pub position: IVec2,
	pub world_position: IVec2,
	pub size: UVec2,
	pub pivot: Vec2,
	pub fields: Vec<Field>,
}

impl Entity {
	pub fn field_by_name(&self, name: impl AsRef<str>) -> Option<&Field> {
		self.fields.iter().find(|field| field.name == name.as_ref())
	}

	pub fn field_by_name_mut(&mut self, name: impl AsRef<str>) -> Option<&mut Field> {
		self.fields
			.iter_mut()
			.find(|field| field.name == name.as_ref())
	}
}

impl TryFrom<RawEntity> for Entity {
	type Error = Error;

	fn try_from(
		RawEntity {
			grid,
			identifier,
			pivot,
			tags,
			world_x,
			world_y,
			height,
			iid,
			px,
			width,
			field_instances,
		}: RawEntity,
	) -> Result<Self, Self::Error> {
		Ok(Self {
			id: iid,
			name: identifier,
			tags,
			grid_position: grid.into(),
			position: px.into(),
			world_position: IVec2::new(world_x, world_y),
			size: UVec2::new(width, height),
			pivot: pivot.into(),
			fields: field_instances,
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
	#[serde(rename = "__tags")]
	tags: Vec<String>,
	#[serde(rename = "__worldX")]
	world_x: i32,
	#[serde(rename = "__worldY")]
	world_y: i32,
	height: u32,
	iid: EntityId,
	px: [i32; 2],
	width: u32,
	field_instances: Vec<Field>,
}
