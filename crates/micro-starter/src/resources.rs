mod fonts;
mod textures;

pub use fonts::*;
pub use textures::*;

use std::path::PathBuf;

use micro::Context;

pub struct Resources {
	pub fonts: Fonts,
	pub textures: Textures,
}

impl Resources {
	pub fn base_dir() -> PathBuf {
		#[cfg(debug_assertions)]
		{
			std::env::current_dir()
				.expect("could not get current working directory")
				.join("resources")
		}
		#[cfg(not(debug_assertions))]
		{
			std::env::current_exe()
				.expect("could not get current executable path")
				.parent()
				.expect("could not get current executable directory")
				.join("resources")
		}
	}

	pub fn fonts_dir() -> PathBuf {
		Self::base_dir().join("fonts")
	}

	pub fn textures_dir() -> PathBuf {
		Self::base_dir().join("textures")
	}

	pub fn new(ctx: &mut Context) -> anyhow::Result<Self> {
		Ok(Self {
			fonts: Fonts::new(ctx)?,
			textures: Textures::new(ctx)?,
		})
	}
}
