mod player;

pub use player::*;

#[cfg(feature = "aseprite")]
mod from_file;

use std::{collections::HashMap, time::Duration};

use crate::math::Rect;

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "aseprite", derive(serde::Deserialize))]
#[cfg_attr(feature = "aseprite", serde(try_from = "from_file::RawAnimationData"))]
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
	pub texture_rect: Rect,
	pub duration: Duration,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Repeats {
	Infinite,
	Finite(u32),
}
