use std::path::Path;

use micro::{
	Context,
	graphics::{LoadShaderError, Shader},
};

use super::AssetLoader;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ShaderLoader;

impl AssetLoader for ShaderLoader {
	type Asset = Shader;

	type Error = LoadShaderError;

	type Settings = ();

	type Context = Context;

	const SUPPORTED_FILE_EXTENSIONS: &'static [&'static str] = &["glsl"];

	fn load(
		&mut self,
		ctx: &mut Context,
		path: &Path,
		_settings: Option<&Self::Settings>,
	) -> Result<Self::Asset, Self::Error> {
		Shader::from_file(ctx, path.file_stem().unwrap().to_string_lossy(), path)
	}

	fn reload(
		&mut self,
		ctx: &mut Context,
		asset: &mut Self::Asset,
		path: &Path,
		_settings: Option<&Self::Settings>,
	) -> Result<(), Self::Error> {
		let source = std::fs::read_to_string(path)?;
		*asset = asset.with_source(ctx, source)?;
		Ok(())
	}
}
