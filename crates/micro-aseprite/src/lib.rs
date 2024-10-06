mod from_file;
mod multiple_from_file;
mod player;

pub use from_file::LoadAnimationDataError;
pub use player::*;

use std::{collections::HashMap, time::Duration};

use micro::math::Rect;

#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
#[serde(try_from = "from_file::RawAnimationData")]
pub struct AnimationData {
	pub frames: Vec<Frame>,
	pub animations: HashMap<String, Animation>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Animation {
	pub start_frame: usize,
	pub end_frame: usize,
	pub repeats: Repeats,
	pub next: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Frame {
	pub texture_region: Rect,
	pub duration: Duration,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Repeats {
	Infinite,
	Finite(u32),
}
