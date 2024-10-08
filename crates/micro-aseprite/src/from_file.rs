use std::{num::ParseIntError, path::Path, time::Duration};

use derive_more::derive::{Display, Error, From};
use serde::Deserialize;

use micro::math::Rect;

use super::{Animation, AnimationData, Frame, Repeats};

#[derive(Debug, Error, Display, From)]
pub enum LoadAnimationDataError {
	#[from]
	IoError(std::io::Error),
	#[from]
	ParseError(serde_json::Error),
	#[display("error parsing the repeat amount for animation {animation_name}: {error}")]
	InvalidRepeatAmount {
		animation_name: String,
		error: ParseIntError,
	},
	#[display("error parsing user data for animation {animation_name}: {error}")]
	ParseUserDataError {
		animation_name: String,
		error: serde_json::Error,
	},
	#[display("Invalid format for frame name {}", frame_name)]
	InvalidFrameName { frame_name: String },
	#[display("Invalid format for tag name {}", tag_name)]
	InvalidTagName { tag_name: String },
	#[display("No frames for animation with name {}", filename)]
	NoFramesForAnimation { filename: String },
}

impl AnimationData {
	pub fn from_file(path: impl AsRef<Path>) -> Result<Self, LoadAnimationDataError> {
		Ok(serde_json::from_str(&std::fs::read_to_string(path)?)?)
	}
}

impl TryFrom<RawAnimationData> for AnimationData {
	type Error = LoadAnimationDataError;

	fn try_from(mut raw: RawAnimationData) -> Result<Self, LoadAnimationDataError> {
		Ok(Self {
			frames: raw
				.frames
				.iter()
				.map(|raw_frame| (*raw_frame).into())
				.collect(),
			animations: raw
				.meta
				.frame_tags
				.drain(..)
				.map(
					|raw_frame_tag| -> Result<(String, Animation), LoadAnimationDataError> {
						let name = raw_frame_tag.name.clone();
						let animation = raw_frame_tag.try_into()?;
						Ok((name, animation))
					},
				)
				.collect::<Result<_, _>>()?,
		})
	}
}

#[derive(Deserialize)]
pub(super) struct RawAnimationData {
	frames: Vec<RawFrame>,
	meta: RawMeta,
}

#[derive(Clone, Copy, Deserialize)]
pub(super) struct RawFrame {
	frame: RawFrameRect,
	duration: u64,
}

impl From<RawFrame> for Frame {
	fn from(raw: RawFrame) -> Self {
		Self {
			texture_region: Rect::new(
				(raw.frame.x as f32, raw.frame.y as f32),
				(raw.frame.w as f32, raw.frame.h as f32),
			),
			duration: Duration::from_millis(raw.duration),
		}
	}
}

#[derive(Clone, Copy, Deserialize)]
struct RawFrameRect {
	x: u32,
	y: u32,
	w: u32,
	h: u32,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct RawMeta {
	pub(super) frame_tags: Vec<RawFrameTag>,
}

#[derive(Deserialize, Clone)]
pub(super) struct RawFrameTag {
	pub(super) name: String,
	pub(super) from: usize,
	pub(super) to: usize,
	pub(super) repeat: Option<String>,
	pub(super) data: Option<String>,
}

impl TryFrom<RawFrameTag> for Animation {
	type Error = LoadAnimationDataError;

	fn try_from(raw: RawFrameTag) -> Result<Self, LoadAnimationDataError> {
		let raw_user_data = raw
			.data
			.map(|data| {
				serde_json::from_str::<RawUserData>(&data).map_err(|error| {
					LoadAnimationDataError::ParseUserDataError {
						animation_name: raw.name.clone(),
						error,
					}
				})
			})
			.transpose()?;
		Ok(Self {
			start_frame: raw.from,
			end_frame: raw.to,
			repeats: match &raw.repeat {
				Some(repeats) => Repeats::Finite(repeats.parse().map_err(|error| {
					LoadAnimationDataError::InvalidRepeatAmount {
						animation_name: raw.name.clone(),
						error,
					}
				})?),
				None => Repeats::Infinite,
			},
			next: raw_user_data.and_then(|raw_user_data| raw_user_data.next),
		})
	}
}

#[derive(Deserialize)]
struct RawUserData {
	next: Option<String>,
}
