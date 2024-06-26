use std::path::Path;

use crate::{
	ldtk::{Error, Level},
	resource::loader::ResourceLoader,
	Context,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LdtkLevelLoader;

impl ResourceLoader for LdtkLevelLoader {
	type Resource = Level;

	type Error = Error;

	type Settings = ();

	const SUPPORTED_FILE_EXTENSIONS: &'static [&'static str] = &["ldtkl"];

	fn load(
		&mut self,
		_ctx: &mut Context,
		path: &Path,
		_settings: Option<&Self::Settings>,
	) -> Result<Self::Resource, Self::Error> {
		Level::from_file(path)
	}
}
