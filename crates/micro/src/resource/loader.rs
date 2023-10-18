mod texture;

use serde::Deserialize;
pub use texture::*;

use std::{error::Error, path::Path};

use crate::Context;

pub trait ResourceLoader {
	type Resource;

	type Error: Error;

	type Settings: for<'a> Deserialize<'a>;

	const SUPPORTED_FILE_EXTENSIONS: &'static [&'static str];

	fn load(
		&mut self,
		ctx: &mut Context,
		path: &Path,
		settings: Option<Self::Settings>,
	) -> Result<Self::Resource, Self::Error>;
}
