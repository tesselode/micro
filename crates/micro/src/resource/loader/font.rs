use crate::{
	graphics::text::{Font, FontSettings, LoadFontError},
	Context,
};

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
		ctx: &mut Context,
		path: &std::path::Path,
		settings: Option<&Self::Settings>,
	) -> Result<Self::Resource, Self::Error> {
		Font::from_file(
			ctx,
			path,
			settings.cloned().unwrap_or(self.default_settings.clone()),
		)
	}
}
