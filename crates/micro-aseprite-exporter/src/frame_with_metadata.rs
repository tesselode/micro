use std::path::Path;

use asefile::AsepriteFile;
use image::RgbaImage;

use super::{Error, Result};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct FrameMetadata {
	pub aseprite_file_stem: String,
	pub index: u32,
}

pub(crate) fn get_frames_with_metadata_from_file(
	path: &Path,
) -> Result<Vec<(FrameMetadata, RgbaImage)>> {
	let aseprite_file_stem = path
		.file_stem()
		.ok_or_else(|| Error::MissingStem {
			path: path.to_path_buf(),
		})?
		.to_str()
		.ok_or_else(|| Error::InvalidUtf8 {
			path: path.to_path_buf(),
		})?;
	let aseprite_file = AsepriteFile::read_file(path)?;
	let frames_with_metadata = (0..aseprite_file.num_frames())
		.map(|i| {
			let frame = aseprite_file.frame(i);
			(
				FrameMetadata {
					aseprite_file_stem: aseprite_file_stem.to_string(),
					index: i,
				},
				frame.image(),
			)
		})
		.collect();
	Ok(frames_with_metadata)
}
