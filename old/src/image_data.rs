use std::{error::Error, fmt::Display, fs::File, path::Path};

use png::{BitDepth, ColorType};

#[derive(Debug)]
pub enum FromFileError {
	IoError(std::io::Error),
	DecodeError(png::DecodingError),
}

impl Display for FromFileError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			FromFileError::IoError(error) => error.fmt(f),
			FromFileError::DecodeError(error) => error.fmt(f),
		}
	}
}

impl Error for FromFileError {
	fn source(&self) -> Option<&(dyn Error + 'static)> {
		match self {
			FromFileError::IoError(error) => Some(error),
			FromFileError::DecodeError(error) => Some(error),
		}
	}
}

impl From<std::io::Error> for FromFileError {
	fn from(v: std::io::Error) -> Self {
		Self::IoError(v)
	}
}

impl From<png::DecodingError> for FromFileError {
	fn from(v: png::DecodingError) -> Self {
		Self::DecodeError(v)
	}
}

pub struct ImageData {
	pub(crate) data: Vec<u8>,
	pub(crate) width: u32,
	pub(crate) height: u32,
}

impl ImageData {
	pub fn from_file(path: impl AsRef<Path>) -> Result<Self, FromFileError> {
		let decoder = png::Decoder::new(File::open(path)?);
		let mut reader = decoder.read_info()?;
		let mut data = vec![0; reader.output_buffer_size()];
		let info = reader.next_frame(&mut data)?;
		assert!(info.color_type == ColorType::Rgba);
		assert!(info.bit_depth == BitDepth::Eight);
		Ok(Self {
			data,
			width: info.width,
			height: info.height,
		})
	}
}
