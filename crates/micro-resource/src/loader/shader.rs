use std::path::Path;

use micro::{
	Context,
	graphics::shader::{LoadShaderError, Shader},
};

use super::ResourceLoader;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ShaderLoader;

impl ResourceLoader for ShaderLoader {
	type Resource = Shader;

	type Error = LoadShaderError;

	type Settings = ();

	type Context = Context;

	const SUPPORTED_FILE_EXTENSIONS: &'static [&'static str] = &["glsl"];

	fn load(
		&mut self,
		ctx: &mut Context,
		path: &Path,
		_settings: Option<&Self::Settings>,
	) -> Result<Self::Resource, Self::Error> {
		Shader::from_combined_file(ctx, path)
	}

	fn reload(
		&mut self,
		ctx: &mut Context,
		resource: &mut Self::Resource,
		path: &Path,
		_settings: Option<&Self::Settings>,
	) -> Result<(), Self::Error> {
		let new_shader = Shader::from_combined_file(ctx, path)?;
		new_shader.import_uniforms(ctx, resource);
		*resource = new_shader;
		Ok(())
	}
}
