use std::path::Path;

use micro::graphics::{LoadShaderError, Shader};

use super::AssetLoader;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ShaderLoader;

impl AssetLoader for ShaderLoader {
	type Asset = Shader;

	type Error = LoadShaderError;

	type Settings = ();

	const SUPPORTED_FILE_EXTENSIONS: &'static [&'static str] = &["glsl"];

	fn load(
		&mut self,
		path: &Path,
		_settings: Option<&Self::Settings>,
	) -> Result<Self::Asset, Self::Error> {
		Shader::from_file(path.file_stem().unwrap().to_string_lossy(), path)
	}

	fn reload(
		&mut self,
		asset: &mut Self::Asset,
		path: &Path,
		_settings: Option<&Self::Settings>,
	) -> Result<(), Self::Error> {
		let source = std::fs::read_to_string(path)?;
		*asset = asset.with_source(source)?;
		Ok(())
	}
}
