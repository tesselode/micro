mod entity;
mod layer;
mod tile;

pub use entity::*;
pub use layer::*;
pub use tile::*;

use glam::{IVec2, UVec2};
use palette::{LinSrgba, Srgb, WithAlpha};
use serde::Deserialize;
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(try_from = "RawLevel")]
pub struct Level {
	pub name: String,
	pub world_position: IVec2,
	pub pixel_size: UVec2,
	pub background_color: LinSrgba,
	pub layers: Vec<Layer>,
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
			layer_instances,
		}: RawLevel,
	) -> Result<Self> {
		Ok(Self {
			name: identifier,
			world_position: IVec2::new(world_x, world_y),
			pixel_size: UVec2::new(px_wid, px_hei),
			background_color: hex_string_to_lin_srgba(&bg_color)?,
			layers: layer_instances,
		})
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Error)]
pub enum Error {
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
	layer_instances: Vec<Layer>,
}

fn hex_string_to_lin_srgba(hex: &str) -> Result<LinSrgba> {
	let color_bytes =
		u32::from_str_radix(&hex[1..], 16).map_err(|_| Error::InvalidColor(hex.to_string()))?;
	Ok(Srgb::from(color_bytes).into_linear().with_alpha(1.0))
}
