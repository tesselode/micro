use std::path::Path;

use micro::graphics::Shader;

use super::AssetLoader;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ShaderLoader;

impl AssetLoader for ShaderLoader {
	type Asset = Shader;

	type Error = std::io::Error;

	type Settings = ();

	type Context = ();

	const SUPPORTED_FILE_EXTENSIONS: &'static [&'static str] = &["glsl"];

	fn load(
		&mut self,
		_ctx: &mut (),
		path: &Path,
		_settings: Option<&Self::Settings>,
	) -> Result<Self::Asset, Self::Error> {
		Shader::from_file(path.file_stem().unwrap().to_string_lossy(), path)
	}

	fn reload(
		&mut self,
		_ctx: &mut (),
		asset: &mut Self::Asset,
		path: &Path,
		_settings: Option<&Self::Settings>,
	) -> Result<(), Self::Error> {
		asset.set_source(std::fs::read_to_string(path)?);
		Ok(())
	}
}
