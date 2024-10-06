mod font;
mod shader;
mod texture;

pub use font::*;
pub use shader::*;
pub use texture::*;

use serde::Deserialize;

use std::{fmt::Debug, path::Path};

#[allow(unused_variables)]
pub trait ResourceLoader {
	type Resource;

	type Error: Debug;

	type Settings: for<'a> Deserialize<'a>;

	type Context;

	const SUPPORTED_FILE_EXTENSIONS: &'static [&'static str];

	fn load(
		&mut self,
		ctx: &mut Self::Context,
		path: &Path,
		settings: Option<&Self::Settings>,
	) -> Result<Self::Resource, Self::Error>;

	fn reload(
		&mut self,
		ctx: &mut Self::Context,
		resource: &mut Self::Resource,
		path: &Path,
		settings: Option<&Self::Settings>,
	) -> Result<(), Self::Error> {
		*resource = self.load(ctx, path, settings)?;
		Ok(())
	}

	fn placeholder(&mut self, ctx: &mut Self::Context) -> Option<Self::Resource> {
		None
	}

	fn warn_on_missing(&self) -> bool {
		true
	}
}
