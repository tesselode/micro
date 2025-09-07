mod font;
mod texture;

pub use font::*;
pub use texture::*;

use serde::Deserialize;

use std::{fmt::Debug, path::Path};

#[allow(unused_variables)]
pub trait AssetLoader {
	type Asset;

	type Error: Debug;

	type Settings: for<'a> Deserialize<'a>;

	const SUPPORTED_FILE_EXTENSIONS: &'static [&'static str];

	fn load(
		&mut self,
		path: &Path,
		settings: Option<&Self::Settings>,
	) -> Result<Self::Asset, Self::Error>;

	fn reload(
		&mut self,
		asset: &mut Self::Asset,
		path: &Path,
		settings: Option<&Self::Settings>,
	) -> Result<(), Self::Error> {
		*asset = self.load(path, settings)?;
		Ok(())
	}

	fn placeholder(&mut self) -> Option<Self::Asset> {
		None
	}

	fn warn_on_missing(&self) -> bool {
		true
	}
}
