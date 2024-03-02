use std::{collections::HashMap, path::Path};

use crate::animation::{AnimationData, LoadAnimationDataError};

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
		path: &Path,
		_settings: Option<&Self::Settings>,
	) -> Result<Self::Resource, Self::Error> {
		AnimationData::from_file(path)
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct MultipleAnimationDataLoader;

impl ResourceLoader for MultipleAnimationDataLoader {
	type Resource = HashMap<String, AnimationData>;

	type Error = LoadAnimationDataError;

	type Settings = ();

	const SUPPORTED_FILE_EXTENSIONS: &'static [&'static str] = &["json"];

	fn load(
		&mut self,
		path: &Path,
		_settings: Option<&Self::Settings>,
	) -> Result<Self::Resource, Self::Error> {
		AnimationData::multiple_from_file(path)
	}
}
