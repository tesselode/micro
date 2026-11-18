mod from_file;
#[cfg(feature = "loader")]
mod loader;
mod multiple_from_file;
mod player;

pub use from_file::LoadAnimationDataError;
#[cfg(feature = "loader")]
pub use loader::*;
pub use player::*;
use serde::{Deserialize, Serialize};

use std::{collections::HashMap, time::Duration};

use micro::math::Rect;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AnimationData {
	pub frames: Vec<Frame>,
	pub animations: HashMap<String, Animation>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Animation {
	pub start_frame: usize,
	pub end_frame: usize,
	pub repeats: Repeats,
	pub next: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Frame {
	pub texture_region: Rect,
	pub duration: Duration,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Repeats {
	Infinite,
	Finite(u32),
}
