use std::{collections::HashMap, path::Path, vec::Vec};

use serde::Deserialize;

use crate::animation::Frame;

use super::{
	from_file::{RawFrame, RawMeta},
	Animation, AnimationData, LoadAnimationDataError,
};

impl AnimationData {
	pub fn multiple_from_file(
		path: impl AsRef<Path>,
	) -> Result<HashMap<String, Self>, LoadAnimationDataError> {
		let raw_animation_data =
			serde_json::from_str::<RawAnimationData>(&std::fs::read_to_string(path)?)?;
		let mut frames = frames_grouped_by_filename(&raw_animation_data)?;
		let mut animations = animations_grouped_by_filename(&raw_animation_data)?;
		animations
			.drain()
			.map(|(filename, animations)| {
				let frames = frames.remove(&filename).ok_or_else(|| {
					LoadAnimationDataError::NoFramesForAnimation {
						filename: filename.clone(),
					}
				})?;
				Ok((filename, AnimationData { frames, animations }))
			})
			.collect::<Result<_, _>>()
	}
}

fn frames_grouped_by_filename(
	raw_animation_data: &RawAnimationData,
) -> Result<HashMap<String, Vec<Frame>>, LoadAnimationDataError> {
	let mut frames: HashMap<String, Vec<(usize, Frame)>> = HashMap::new();
	for (frame_name, frame) in &raw_animation_data.frames {
		let (filename, frame_index) = filename_and_frame_index_from_frame_name(frame_name)?;
		frames
			.entry(filename.to_string())
			.or_insert_with(Vec::new)
			.push((frame_index, (*frame).into()))
	}
	// sort each list of frames by the frame index, then discard the frame index
	// because it's redundant information
	let sorted_frames = frames
		.drain()
		.map(|(name, mut frames)| {
			frames.sort_by(|(index_a, _), (index_b, _)| index_a.cmp(index_b));
			let frames = frames.drain(..).map(|(_, frame)| frame).collect::<Vec<_>>();
			(name, frames)
		})
		.collect();
	Ok(sorted_frames)
}

fn animations_grouped_by_filename(
	raw_animation_data: &RawAnimationData,
) -> Result<HashMap<String, HashMap<String, Animation>>, LoadAnimationDataError> {
	let mut animations: HashMap<String, HashMap<String, Animation>> = HashMap::new();
	for frame_tag in &raw_animation_data.meta.frame_tags {
		let (filename, animation_name) =
			filename_and_animation_name_from_tag_name(&frame_tag.name)?;
		animations
			.entry(filename.to_string())
			.or_insert_with(HashMap::new)
			.insert(animation_name.to_string(), frame_tag.clone().try_into()?);
	}
	Ok(animations)
}

fn filename_and_frame_index_from_frame_name(
	frame_name: &str,
) -> Result<(&str, usize), LoadAnimationDataError> {
	let mut components_iter = frame_name.split('/');
	let filename =
		components_iter
			.next()
			.ok_or_else(|| LoadAnimationDataError::InvalidFrameName {
				frame_name: frame_name.to_string(),
			})?;
	let frame_index = components_iter
		.next()
		.ok_or_else(|| LoadAnimationDataError::InvalidFrameName {
			frame_name: frame_name.to_string(),
		})?
		.parse::<usize>()
		.map_err(|_| LoadAnimationDataError::InvalidFrameName {
			frame_name: frame_name.to_string(),
		})?;
	Ok((filename, frame_index))
}

fn filename_and_animation_name_from_tag_name(
	tag_name: &str,
) -> Result<(&str, &str), LoadAnimationDataError> {
	let mut components_iter = tag_name.split('/');
	let filename =
		components_iter
			.next()
			.ok_or_else(|| LoadAnimationDataError::InvalidTagName {
				tag_name: tag_name.to_string(),
			})?;
	let animation_name =
		components_iter
			.next()
			.ok_or_else(|| LoadAnimationDataError::InvalidTagName {
				tag_name: tag_name.to_string(),
			})?;
	Ok((filename, animation_name))
}

#[derive(Deserialize)]
struct RawAnimationData {
	frames: HashMap<String, RawFrame>,
	meta: RawMeta,
}
