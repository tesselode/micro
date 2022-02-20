use std::path::Path;

use image::{ImageError, RgbaImage};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum LoadImageDataError {
	#[error("{0}")]
	IoError(#[from] std::io::Error),
	#[error("{0}")]
	ImageError(#[from] ImageError),
}

pub struct ImageData(pub(crate) RgbaImage);

impl ImageData {
	pub fn load(path: impl AsRef<Path>) -> Result<Self, LoadImageDataError> {
		Ok(Self(image::io::Reader::open(path)?.decode()?.to_rgba8()))
	}
}
