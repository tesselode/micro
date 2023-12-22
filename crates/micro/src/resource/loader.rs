#[cfg(feature = "aseprite")]
mod animation_data;
mod font;
#[cfg(feature = "ldtk")]
mod ldtk;
#[cfg(feature = "kira")]
mod static_sound_data;
#[cfg(feature = "kira")]
mod streaming_sound_data;
mod texture;
#[cfg(feature = "tiled")]
mod tiled;

#[cfg(feature = "aseprite")]
pub use animation_data::*;
pub use font::*;
#[cfg(feature = "ldtk")]
pub use ldtk::*;
#[cfg(feature = "kira")]
pub use static_sound_data::*;
#[cfg(feature = "kira")]
pub use streaming_sound_data::*;
pub use texture::*;
#[cfg(feature = "tiled")]
pub use tiled::*;

use serde::Deserialize;

use std::{fmt::Debug, path::Path};

use crate::Context;

#[allow(unused_variables)]
pub trait ResourceLoader {
	type Resource;

	type Error: Debug;

	type Settings: for<'a> Deserialize<'a>;

	const SUPPORTED_FILE_EXTENSIONS: &'static [&'static str];

	fn load(
		&mut self,
		ctx: &mut Context,
		path: &Path,
		settings: Option<&Self::Settings>,
	) -> Result<Self::Resource, Self::Error>;

	fn reload(
		&mut self,
		ctx: &mut Context,
		resource: &mut Self::Resource,
		path: &Path,
		settings: Option<&Self::Settings>,
	) -> Result<(), Self::Error> {
		*resource = self.load(ctx, path, settings)?;
		Ok(())
	}

	fn placeholder(&mut self, ctx: &mut Context) -> Option<Self::Resource> {
		None
	}
}
