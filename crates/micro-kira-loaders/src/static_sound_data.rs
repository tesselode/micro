use std::path::Path;

use kira::{
	Frame,
	sound::static_sound::{StaticSoundData, StaticSoundSettings},
};
use micro_asset::AssetLoader;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct StaticSoundDataLoader {
	pub default_settings: StaticSoundSettings,
}

impl AssetLoader for StaticSoundDataLoader {
	type Asset = StaticSoundData;

	type Error = kira::sound::FromFileError;

	type Settings = ();

	const SUPPORTED_FILE_EXTENSIONS: &'static [&'static str] = &["mp3", "ogg", "flac", "wav"];

	fn load(
		&mut self,
		path: &Path,
		_settings: Option<&Self::Settings>,
	) -> Result<Self::Asset, Self::Error> {
		Ok(StaticSoundData::from_file(path)?.with_settings(self.default_settings))
	}

	fn placeholder(&mut self) -> Option<Self::Asset> {
		Some(StaticSoundData {
			sample_rate: 0,
			frames: vec![Frame::ZERO].into(),
			settings: StaticSoundSettings::default(),
			slice: None,
		})
	}
}
