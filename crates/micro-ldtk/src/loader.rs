use std::path::Path;

use crate::{
	ldtk::{Error, Level},
	asset::loader::AssetLoader,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LdtkLevelLoader;

impl AssetLoader for LdtkLevelLoader {
	type Asset = Level;

	type Error = Error;

	type Settings = ();

	type Context = ();

	const SUPPORTED_FILE_EXTENSIONS: &'static [&'static str] = &["ldtkl"];

	fn load(
		&mut self,
		_ctx: &mut (),
		path: &Path,
		_settings: Option<&Self::Settings>,
	) -> Result<Self::Asset, Self::Error> {
		Level::from_file(path)
	}
}
