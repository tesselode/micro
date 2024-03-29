use crate::graphics::text::{Font, FontSettings, LoadFontError};

use super::ResourceLoader;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct FontLoader {
	pub default_settings: FontSettings,
}

impl ResourceLoader for FontLoader {
	type Resource = Font;

	type Error = LoadFontError;

	type Settings = FontSettings;

	const SUPPORTED_FILE_EXTENSIONS: &'static [&'static str] = &["ttf", "ttc", "otf"];

	fn load(
		&mut self,
		path: &std::path::Path,
		settings: Option<&Self::Settings>,
	) -> Result<Self::Resource, Self::Error> {
		Font::from_file(
			path,
			settings.cloned().unwrap_or(self.default_settings.clone()),
		)
	}
}
