use std::{
	collections::HashMap,
	path::{Path, PathBuf},
	time::Duration,
};

use asefile::{AsepriteFile, AsepriteParseError, Tag};
use crunch::{Item, PackedItem, PackedItems, Rotation};
use image::{ImageError, RgbaImage};
use indexmap::IndexMap;
use micro::animation::{Animation, AnimationData, Repeats};
use serde::Deserialize;
use thiserror::Error;

pub fn export_aseprite_files(
	aseprite_file_paths: impl IntoIterator<Item = impl AsRef<Path>>,
	texture_atlas_path: impl AsRef<Path>,
	animations_dir: impl AsRef<Path>,
	max_size: usize,
) -> Result<()> {
	let aseprite_files = aseprite_file_paths
		.into_iter()
		.map(|path| -> Result<(String, AsepriteFile)> {
			let path = path.as_ref();
			let aseprite_file_stem = path
				.file_stem()
				.ok_or_else(|| Error::MissingStem {
					path: path.to_path_buf(),
				})?
				.to_str()
				.ok_or_else(|| Error::InvalidUtf8 {
					path: path.to_path_buf(),
				})?
				.to_string();
			let aseprite_file = AsepriteFile::read_file(path)?;
			Ok((aseprite_file_stem, aseprite_file))
		})
		.collect::<Result<Vec<_>>>()?;
	let mut frames = IndexMap::new();
	load_frames_with_metadata_from_aseprite_files(&aseprite_files, &mut frames);
	let packed_items = crunch::pack_into_po2(
		max_size,
		frames.iter().map(|(metadata, image)| Item {
			data: metadata.clone(),
			w: image.width() as usize,
			h: image.height() as usize,
			rot: Rotation::None,
		}),
	)
	.map_err(|()| Error::FramesCannotFit)?;
	let mut atlas = RgbaImage::new(packed_items.w as u32, packed_items.h as u32);
	for PackedItem { data, rect } in &packed_items.items {
		copy_image(&frames[data], &mut atlas, rect.x as u32, rect.y as u32);
	}
	atlas.save(texture_atlas_path)?;
	export_animations(animations_dir.as_ref(), &aseprite_files, &packed_items)?;
	Ok(())
}

#[derive(Debug, Error)]
pub enum Error {
	#[error("aseprite file path has no stem: {path}")]
	MissingStem { path: PathBuf },
	#[error("aseprite file path is invalid utf8: {path}")]
	InvalidUtf8 { path: PathBuf },
	#[error("the frames cannot fit into a texture atlas with the specified max size")]
	FramesCannotFit,
	#[error("{0}")]
	AsepriteParseError(#[from] AsepriteParseError),
	#[error("{0}")]
	ImageError(#[from] ImageError),
	#[error("error parsing tag userdata: {0}")]
	ParseUserdataError(#[from] serde_json::Error),
	#[error("{0}")]
	IoError(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

fn load_frames_with_metadata_from_aseprite_files(
	aseprite_files: &[(String, AsepriteFile)],
	frames: &mut IndexMap<FrameMetadata, RgbaImage>,
) {
	for (aseprite_file_stem, aseprite_file) in aseprite_files {
		for i in 0..aseprite_file.num_frames() {
			let frame = aseprite_file.frame(i);
			frames.insert(
				FrameMetadata {
					aseprite_file_stem: aseprite_file_stem.to_string(),
					index: i,
					duration: Duration::from_millis(frame.duration() as u64),
				},
				frame.image(),
			);
		}
	}
}

fn copy_image(src: &RgbaImage, dest: &mut RgbaImage, x: u32, y: u32) {
	for i in 0..src.width() {
		for j in 0..src.height() {
			dest.put_pixel(x + i, y + j, *src.get_pixel(i, j));
		}
	}
}

fn export_animations(
	output_dir: &Path,
	aseprite_files: &[(String, AsepriteFile)],
	packed_items: &PackedItems<FrameMetadata>,
) -> Result<()> {
	std::fs::create_dir_all(output_dir)?;
	for (stem, aseprite_file) in aseprite_files {
		if aseprite_file.num_tags() == 0 {
			continue;
		}
		let mut packed_items_for_file = packed_items
			.items
			.iter()
			.filter(|item| item.data.aseprite_file_stem == *stem)
			.collect::<Vec<_>>();
		packed_items_for_file.sort_by(|a, b| a.data.index.cmp(&b.data.index));
		let animation_frames = packed_items_for_file
			.iter()
			.map(|item| micro::animation::Frame {
				texture_region: micro::math::Rect::from_xywh(
					item.rect.x as f32,
					item.rect.y as f32,
					item.rect.w as f32,
					item.rect.h as f32,
				),
				duration: item.data.duration,
			})
			.collect::<Vec<_>>();
		let animations = (0..aseprite_file.num_tags())
			.map(|i| -> Result<(String, Animation)> {
				let tag = aseprite_file.tag(i);
				Ok((tag.name().to_string(), micro_animation_from_tag(tag)?))
			})
			.collect::<Result<HashMap<_, _>>>()?;
		let animation_data = AnimationData {
			frames: animation_frames,
			animations,
		};
		std::fs::write(
			output_dir.join(stem).with_extension("json"),
			serde_json::to_string(&animation_data)?,
		)?;
	}
	Ok(())
}

fn micro_animation_from_tag(tag: &Tag) -> Result<Animation> {
	let userdata = tag
		.user_data()
		.and_then(|userdata| {
			userdata
				.text
				.as_ref()
				.map(|text| serde_json::from_str::<TagUserdata>(text))
		})
		.transpose()?;
	let animation = Animation {
		start_frame: tag.from_frame() as usize,
		end_frame: tag.to_frame() as usize,
		repeats: Repeats::Infinite,
		next: userdata.map(|userdata| userdata.next),
	};
	Ok(animation)
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct FrameMetadata {
	aseprite_file_stem: String,
	index: u32,
	duration: Duration,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
struct TagUserdata {
	next: String,
}
