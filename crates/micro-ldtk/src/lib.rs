mod entity;
mod entity_ref;
mod field;
mod layer;
#[cfg(feature = "loader")]
mod loader;
mod tile;

pub use entity::*;
pub use entity_ref::*;
pub use field::*;
pub use layer::*;
pub use tile::*;

use std::path::Path;

use micro::{
	color::{LinSrgba, Srgb, WithAlpha},
	math::{IVec2, UVec2},
};
use serde::Deserialize;
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(try_from = "RawLevel")]
pub struct Level {
	pub name: String,
	pub world_position: IVec2,
	pub pixel_size: UVec2,
	pub background_color: LinSrgba,
	pub fields: Vec<Field>,
	pub layers: Vec<Layer>,
}

impl Level {
	pub fn from_file(path: impl AsRef<Path>) -> Result<Self> {
		let level_string = std::fs::read_to_string(path)?;
		let level = serde_json::from_str::<Level>(&level_string)?;
		Ok(level)
	}

	pub fn field_by_name(&self, name: impl AsRef<str>) -> Option<&Field> {
		self.fields.iter().find(|field| field.name == name.as_ref())
	}

	pub fn field_by_name_mut(&mut self, name: impl AsRef<str>) -> Option<&mut Field> {
		self.fields
			.iter_mut()
			.find(|field| field.name == name.as_ref())
	}

	pub fn layer_by_name(&self, name: impl AsRef<str>) -> Option<&Layer> {
		self.layers.iter().find(|layer| layer.name == name.as_ref())
	}

	pub fn layer_by_name_mut(&mut self, name: impl AsRef<str>) -> Option<&mut Layer> {
		self.layers
			.iter_mut()
			.find(|layer| layer.name == name.as_ref())
	}
}

impl TryFrom<RawLevel> for Level {
	type Error = Error;

	fn try_from(
		RawLevel {
			identifier,
			world_x,
			world_y,
			px_wid,
			px_hei,
			bg_color,
			field_instances,
			layer_instances,
		}: RawLevel,
	) -> Result<Self> {
		Ok(Self {
			name: identifier,
			world_position: IVec2::new(world_x, world_y),
			pixel_size: UVec2::new(px_wid, px_hei),
			background_color: hex_string_to_lin_srgba(&bg_color)?,
			fields: field_instances,
			layers: layer_instances,
		})
	}
}

#[derive(Debug, Error)]
pub enum Error {
	#[error("{0}")]
	IoError(#[from] std::io::Error),
	#[error("Error parsing ldtk file: {0}")]
	ParseError(#[from] serde_json::Error),
	#[error("{0} is not a valid color")]
	InvalidColor(String),
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct RawLevel {
	identifier: String,
	world_x: i32,
	world_y: i32,
	px_wid: u32,
	px_hei: u32,
	#[serde(rename = "__bgColor")]
	bg_color: String,
	field_instances: Vec<Field>,
	layer_instances: Vec<Layer>,
}

fn hex_string_to_lin_srgba(hex: &str) -> Result<LinSrgba> {
	let color_bytes =
		u32::from_str_radix(&hex[1..], 16).map_err(|_| Error::InvalidColor(hex.to_string()))?;
	Ok(Srgb::from(color_bytes).into_linear().with_alpha(1.0))
}
