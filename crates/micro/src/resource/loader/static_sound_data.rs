use std::path::Path;

use kira::{
	sound::static_sound::{StaticSoundData, StaticSoundSettings},
	Frame,
};

use crate::Context;

use super::ResourceLoader;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
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
		_ctx: &mut Context,
		path: &Path,
		_settings: Option<&Self::Settings>,
	) -> Result<Self::Resource, Self::Error> {
		Ok(StaticSoundData::from_file(path)?.with_settings(self.default_settings))
	}

	fn placeholder(&mut self, _ctx: &mut Context) -> Option<Self::Resource> {
		Some(StaticSoundData {
			sample_rate: 0,
			frames: vec![Frame::ZERO].into(),
			settings: StaticSoundSettings::default(),
			slice: None,
		})
	}
}
