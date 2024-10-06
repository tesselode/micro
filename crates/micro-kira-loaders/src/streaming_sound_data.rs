use std::path::Path;

use kira::sound::streaming::{StreamingSoundData, StreamingSoundSettings};
use micro::resource::loader::ResourceLoader;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StreamingSoundDataLoader {
	pub default_settings: StreamingSoundSettings,
}

impl ResourceLoader for StreamingSoundDataLoader {
	type Resource = StreamingSoundData<kira::sound::FromFileError>;

	type Error = kira::sound::FromFileError;

	type Settings = ();

	type Context = ();

	const SUPPORTED_FILE_EXTENSIONS: &'static [&'static str] = &["mp3", "ogg", "flac", "wav"];

	fn load(
		&mut self,
		_ctx: &mut (),
		path: &Path,
		_settings: Option<&Self::Settings>,
	) -> Result<Self::Resource, Self::Error> {
		Ok(StreamingSoundData::from_file(path)?.with_settings(self.default_settings))
	}
}
