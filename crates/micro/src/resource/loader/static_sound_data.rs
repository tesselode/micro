use std::path::Path;

use kira::sound::static_sound::{StaticSoundData, StaticSoundSettings};

use super::ResourceLoader;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StaticSoundDataLoader {
	pub default_settings: StaticSoundSettings,
}

impl ResourceLoader for StaticSoundDataLoader {
	type Resource = StaticSoundData;

	type Error = kira::sound::FromFileError;

	type Settings = ();

	const SUPPORTED_FILE_EXTENSIONS: &'static [&'static str] = &["mp3", "ogg", "flac", "wav"];

	fn load(
		&mut self,
		path: &Path,
		_settings: Option<&Self::Settings>,
	) -> Result<Self::Resource, Self::Error> {
		StaticSoundData::from_file(path, self.default_settings)
	}
}
