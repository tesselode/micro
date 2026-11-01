mod asset_with_metadata;
mod assets;
mod loader;
mod single_asset;

pub use assets::*;
pub use loader::*;
pub use single_asset::*;

use std::{path::PathBuf, time::Duration};

const HOT_RELOAD_INTERVAL: Duration = Duration::from_secs(1);

pub fn base_assets_path() -> PathBuf {
	#[cfg(debug_assertions)]
	{
		"assets".into()
	}
	#[cfg(not(debug_assertions))]
	{
		std::env::current_exe()
			.expect("could not get current executable path")
			.parent()
			.expect("could not get current executable directory")
			.join("assets")
	}
}
