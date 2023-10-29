mod player;

pub use player::*;

use std::{collections::HashMap, time::Duration};

use crate::math::Rect;

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serializing", derive(serde::Serialize, serde::Deserialize))]
pub struct AnimationData {
	pub frames: Vec<Frame>,
	pub animations: HashMap<String, Animation>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serializing", derive(serde::Serialize, serde::Deserialize))]
pub struct Animation {
	pub start_frame: usize,
	pub end_frame: usize,
	pub repeats: Repeats,
	pub next: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serializing", derive(serde::Serialize, serde::Deserialize))]
pub struct Frame {
	pub texture_region: Rect,
	pub duration: Duration,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serializing", derive(serde::Serialize, serde::Deserialize))]
pub enum Repeats {
	Infinite,
	Finite(u32),
}
