use glam::UVec2;
use serde::Deserialize;

#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
#[serde(from = "RawTile")]
pub struct Tile {
	pub opacity: f32,
	pub flip_x: bool,
	pub flip_y: bool,
	pub layer_coords: UVec2,
	pub tileset_coords: UVec2,
}

impl From<RawTile> for Tile {
	fn from(RawTile { a, f, px, src }: RawTile) -> Self {
		Self {
			opacity: a,
			flip_x: (f & 0b1) == 1,
			flip_y: ((f >> 1) & 0b1) == 1,
			layer_coords: px.into(),
			tileset_coords: src.into(),
		}
	}
}

#[derive(Deserialize)]
struct RawTile {
	a: f32,
	f: i8,
	px: [u32; 2],
	src: [u32; 2],
}
