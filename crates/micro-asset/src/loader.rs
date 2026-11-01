mod shader;
mod texture;

pub use shader::*;
pub use texture::*;

use serde::Deserialize;

use std::{fmt::Display, path::Path};

#[allow(unused_variables)]
pub trait AssetLoader {
	type Asset;

	type Error: Display;

	type Settings: for<'a> Deserialize<'a>;

	type Context;

	const SUPPORTED_FILE_EXTENSIONS: &'static [&'static str];

	fn load(
		&mut self,
		ctx: &mut Self::Context,
		path: &Path,
		settings: Option<&Self::Settings>,
	) -> Result<Self::Asset, Self::Error>;

	fn reload(
		&mut self,
		ctx: &mut Self::Context,
		asset: &mut Self::Asset,
		path: &Path,
		settings: Option<&Self::Settings>,
	) -> Result<(), Self::Error> {
		*asset = self.load(ctx, path, settings)?;
		Ok(())
	}

	fn placeholder(&mut self, ctx: &mut Self::Context) -> Option<Self::Asset> {
		None
	}

	fn warn_on_missing(&self) -> bool {
		true
	}
}
