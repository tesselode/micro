use std::path::Path;

use crate::{
	animation::{AnimationData, LoadAnimationDataError},
	Context,
};

use super::ResourceLoader;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct AnimationDataLoader;

impl ResourceLoader for AnimationDataLoader {
	type Resource = AnimationData;

	type Error = LoadAnimationDataError;

	type Settings = ();

	const SUPPORTED_FILE_EXTENSIONS: &'static [&'static str] = &["json"];

	fn load(
		&mut self,
		_ctx: &mut Context,
		path: &Path,
		_settings: Option<Self::Settings>,
	) -> Result<Self::Resource, Self::Error> {
		AnimationData::from_file(path)
	}
}
