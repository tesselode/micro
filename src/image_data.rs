pub mod error;

use std::path::Path;

use image::RgbaImage;

use self::error::LoadImageDataError;

pub struct ImageData(pub(crate) RgbaImage);

impl ImageData {
	pub fn load(path: impl AsRef<Path>) -> Result<Self, LoadImageDataError> {
		Ok(Self(image::io::Reader::open(path)?.decode()?.to_rgba8()))
	}
}
