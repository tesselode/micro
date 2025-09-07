mod font;
mod texture;

pub use font::*;
pub use texture::*;

use serde::Deserialize;

use std::{fmt::Debug, path::Path};

#[allow(unused_variables)]
pub trait ResourceLoader {
	type Resource;

	type Error: Debug;

	type Settings: for<'a> Deserialize<'a>;

	const SUPPORTED_FILE_EXTENSIONS: &'static [&'static str];

	fn load(
		&mut self,
		path: &Path,
		settings: Option<&Self::Settings>,
	) -> Result<Self::Resource, Self::Error>;

	fn reload(
		&mut self,
		resource: &mut Self::Resource,
		path: &Path,
		settings: Option<&Self::Settings>,
	) -> Result<(), Self::Error> {
		*resource = self.load(path, settings)?;
		Ok(())
	}

	fn placeholder(&mut self) -> Option<Self::Resource> {
		None
	}

	fn warn_on_missing(&self) -> bool {
		true
	}
}
