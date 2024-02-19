use std::path::Path;

use crate::{
	graphics::shader::{LoadShaderError, Shader},
	Context,
};

use super::ResourceLoader;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ShaderLoader;

impl ResourceLoader for ShaderLoader {
	type Resource = Shader;

	type Error = LoadShaderError;

	type Settings = ();

	const SUPPORTED_FILE_EXTENSIONS: &'static [&'static str] = &["glsl"];

	fn load(
		&mut self,
		ctx: &mut Context,
		path: &Path,
		_settings: Option<&Self::Settings>,
	) -> Result<Self::Resource, Self::Error> {
		Shader::from_combined_file(ctx, path)
	}
}
