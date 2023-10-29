mod frame_with_metadata;

use std::path::{Path, PathBuf};

use asefile::AsepriteParseError;
use crunch::{Item, PackedItem, Rotation};
use image::{ImageError, RgbaImage};
use indexmap::IndexMap;
use thiserror::Error;

use crate::frame_with_metadata::get_frames_with_metadata_from_file;

pub fn export_texture_atlas(
	aseprite_file_paths: impl IntoIterator<Item = impl AsRef<Path>>,
	output_path: impl AsRef<Path>,
	max_size: usize,
) -> Result<()> {
	let mut frames = IndexMap::new();
	for aseprite_file_path in aseprite_file_paths {
		for (metadata, image) in get_frames_with_metadata_from_file(aseprite_file_path.as_ref())? {
			frames.insert(metadata, image);
		}
	}
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
	for PackedItem { data, rect } in packed_items.items {
		copy_image(&frames[&data], &mut atlas, rect.x as u32, rect.y as u32);
	}
	atlas.save(output_path)?;
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
}

pub type Result<T> = std::result::Result<T, Error>;

fn copy_image(src: &RgbaImage, dest: &mut RgbaImage, x: u32, y: u32) {
	for i in 0..src.width() {
		for j in 0..src.height() {
			dest.put_pixel(x + i, y + j, *src.get_pixel(i, j));
		}
	}
}
