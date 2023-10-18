use std::path::Path;

use kira::sound::streaming::{StreamingSoundData, StreamingSoundSettings};

use crate::Context;

use super::ResourceLoader;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StreamingSoundDataLoader {
	pub default_settings: StreamingSoundSettings,
}

impl ResourceLoader for StreamingSoundDataLoader {
	type Resource = StreamingSoundData<kira::sound::FromFileError>;

	type Error = kira::sound::FromFileError;

	type Settings = ();

	const SUPPORTED_FILE_EXTENSIONS: &'static [&'static str] = &["mp3", "ogg", "flac", "wav"];

	fn load(
		&mut self,
		_ctx: &mut Context,
		path: &Path,
		_settings: Option<Self::Settings>,
	) -> Result<Self::Resource, Self::Error> {
		StreamingSoundData::from_file(path, self.default_settings)
	}
}
