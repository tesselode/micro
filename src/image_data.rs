use std::{error::Error, fs::File, path::Path};

use png::{BitDepth, ColorType};

pub struct ImageData {
	pub(crate) data: Vec<u8>,
	pub(crate) width: u32,
	pub(crate) height: u32,
}

impl ImageData {
	pub fn from_file(path: impl AsRef<Path>) -> Result<Self, Box<dyn Error>> {
		let decoder = png::Decoder::new(File::open(path)?);
		let mut reader = decoder.read_info()?;
		let mut data = vec![0; reader.output_buffer_size()];
		let info = reader.next_frame(&mut data)?;
		assert!(info.color_type == ColorType::Rgb);
		assert!(info.bit_depth == BitDepth::Eight);
		Ok(Self {
			data,
			width: info.width,
			height: info.height,
		})
	}
}
