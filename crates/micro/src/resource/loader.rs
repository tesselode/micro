mod texture;

pub use texture::*;

use std::path::Path;

use crate::Context;

pub trait ResourceLoader {
	type Resource;

	type Error: From<std::io::Error>;

	const SUPPORTED_FILE_EXTENSIONS: &'static [&'static str];

	fn load(&mut self, ctx: &mut Context, path: &Path) -> Result<Self::Resource, Self::Error>;
}
