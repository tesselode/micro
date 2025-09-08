mod loader;
mod asset_with_metadata;

pub use loader::*;

use std::{
	fmt::Debug,
	ops::{Index, IndexMut},
	path::{Path, PathBuf},
	sync::Mutex,
	time::Duration,
};

use indexmap::{IndexMap, IndexSet};
use tracing::warn;

use self::asset_with_metadata::AssetWithMetadata;

const HOT_RELOAD_INTERVAL: Duration = Duration::from_secs(1);

pub struct Assets<L: AssetLoader> {
	base_dir: PathBuf,
	loader: L,
	assets: IndexMap<PathBuf, AssetWithMetadata<L>>,
	placeholder: Option<L::Asset>,
	hot_reload_timer: Duration,
	missing_asset_logger: MissingAssetLogger,
}

impl<L: AssetLoader> Assets<L> {
	pub fn new(ctx: &mut L::Context, base_dir: impl AsRef<Path>, mut loader: L) -> Self {
		let placeholder = loader.placeholder(ctx);
		Self {
			base_dir: base_assets_path().join(base_dir.as_ref()),
			loader,
			assets: IndexMap::new(),
			placeholder,
			hot_reload_timer: Duration::ZERO,
			missing_asset_logger: MissingAssetLogger::new(),
		}
	}

	pub fn autoloaded(ctx: &mut L::Context, base_dir: impl AsRef<Path>, loader: L) -> Self {
		let mut assets = Self::new(ctx, base_dir, loader);
		assets.load_all(ctx);
		assets
	}

	pub fn load(&mut self, ctx: &mut L::Context, path: impl AsRef<Path>) {
		self.load_inner(ctx, path.as_ref())
	}

	pub fn load_all(&mut self, ctx: &mut L::Context) {
		self.load(ctx, "")
	}

	pub fn unload(&mut self, dir: impl AsRef<Path>) {
		let dir = dir.as_ref();
		self.assets.retain(|path, _| path.starts_with(dir));
	}

	pub fn append(&mut self, mut other: Self) {
		self.assets.extend(other.assets.drain(..));
	}

	pub fn get(&self, path: impl AsRef<Path>) -> Option<&L::Asset> {
		let path = path.as_ref();
		if let Some(asset) = self.assets.get(path) {
			Some(&asset.asset)
		} else {
			if self.loader.warn_on_missing() {
				self.missing_asset_logger.log(path);
			}
			if let Some(placeholder) = &self.placeholder {
				Some(placeholder)
			} else {
				None
			}
		}
	}

	pub fn get_mut(&mut self, path: impl AsRef<Path>) -> Option<&mut L::Asset> {
		let path = path.as_ref();
		if let Some(asset) = self.assets.get_mut(path) {
			Some(&mut asset.asset)
		} else {
			if self.loader.warn_on_missing() {
				self.missing_asset_logger.log(path);
			}
			if let Some(placeholder) = &mut self.placeholder {
				Some(placeholder)
			} else {
				None
			}
		}
	}

	pub fn iter(&self) -> impl Iterator<Item = (&Path, &L::Asset)> {
		self.assets
			.iter()
			.map(|(path, asset)| (path.as_ref(), &asset.asset))
	}

	#[cfg(debug_assertions)]
	pub fn update_hot_reload(&mut self, ctx: &mut L::Context, delta_time: Duration) {
		self.hot_reload_timer += delta_time;
		if self.hot_reload_timer >= HOT_RELOAD_INTERVAL {
			self.hot_reload(ctx);
			self.hot_reload_timer = Duration::ZERO;
		}
	}

	#[cfg(not(debug_assertions))]
	pub fn update_hot_reload(&mut self, _ctx: &mut L::Context, _delta_time: Duration) {}

	fn load_inner(&mut self, ctx: &mut L::Context, path: &Path) {
		let full_asset_path = self.base_dir.join(path);
		if full_asset_path.is_dir() {
			let asset_paths = match self.assets_in_dir(&full_asset_path) {
				Ok(paths) => paths,
				Err(err) => {
					tracing::error!(
						"Error getting assets in {}: {}",
						full_asset_path.display(),
						err
					);
					return;
				}
			};
			for asset_path in asset_paths {
				self.load_inner(ctx, &asset_path);
			}
		} else {
			let asset =
				match AssetWithMetadata::load(ctx, &full_asset_path, &mut self.loader) {
					Ok(Some(asset)) => asset,
					Ok(None) => return,
					Err(err) => {
						tracing::error!(
							"Error loading asset at path {}: {:?}",
							full_asset_path.display(),
							err
						);
						return;
					}
				};
			self.assets.insert(path.into(), asset);
		}
	}

	fn assets_in_dir(
		&mut self,
		full_path: &PathBuf,
	) -> Result<IndexSet<PathBuf>, std::io::Error> {
		let mut asset_paths = IndexSet::new();
		for entry in std::fs::read_dir(full_path)? {
			let entry = entry?;
			asset_paths.insert(
				entry
					.path()
					.strip_prefix(&self.base_dir)
					.unwrap()
					.with_extension(""),
			);
		}
		Ok(asset_paths)
	}

	fn hot_reload(&mut self, ctx: &mut L::Context) {
		for (path, asset) in &mut self.assets {
			let reloaded = asset.reload(ctx, &mut self.loader);
			if reloaded {
				self.missing_asset_logger.on_reloaded(path);
			}
		}
	}
}

impl<L> Debug for Assets<L>
where
	L: AssetLoader + Debug,
	L::Asset: Debug,
	L::Settings: Debug,
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("Assets")
			.field("base_dir", &self.base_dir)
			.field("loader", &self.loader)
			.field("assets", &self.assets)
			.field("placeholder", &self.placeholder)
			.field("hot_reload_timer", &self.hot_reload_timer)
			.finish()
	}
}

impl<L> Clone for Assets<L>
where
	L: AssetLoader + Clone,
	L::Asset: Clone,
	L::Settings: Clone,
{
	fn clone(&self) -> Self {
		Self {
			base_dir: self.base_dir.clone(),
			loader: self.loader.clone(),
			assets: self.assets.clone(),
			placeholder: self.placeholder.clone(),
			hot_reload_timer: self.hot_reload_timer,
			missing_asset_logger: self.missing_asset_logger.clone(),
		}
	}
}

impl<L> PartialEq for Assets<L>
where
	L: AssetLoader + PartialEq,
	L::Asset: PartialEq,
	L::Settings: PartialEq,
{
	fn eq(&self, other: &Self) -> bool {
		self.base_dir == other.base_dir
			&& self.loader == other.loader
			&& self.assets == other.assets
			&& self.placeholder == other.placeholder
			&& self.hot_reload_timer == other.hot_reload_timer
	}
}

impl<L> Eq for Assets<L>
where
	L: AssetLoader + Eq,
	L::Asset: Eq,
	L::Settings: Eq,
{
}

impl<T: AsRef<Path>, L: AssetLoader> Index<T> for Assets<L> {
	type Output = L::Asset;

	fn index(&self, path: T) -> &Self::Output {
		self.get(path).unwrap()
	}
}

impl<T: AsRef<Path>, L: AssetLoader> IndexMut<T> for Assets<L> {
	fn index_mut(&mut self, path: T) -> &mut Self::Output {
		self.get_mut(path).unwrap()
	}
}

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

struct MissingAssetLogger {
	logged_paths: Mutex<IndexSet<PathBuf>>,
}

impl MissingAssetLogger {
	fn new() -> Self {
		Self {
			logged_paths: Mutex::new(IndexSet::new()),
		}
	}

	fn log(&self, path: &Path) {
		let mut logged_paths = self.logged_paths.lock().unwrap();
		if logged_paths.contains(path) {
			return;
		}
		warn!("Missing asset '{}'", path.display());
		logged_paths.insert(path.to_path_buf());
	}

	fn on_reloaded(&self, path: &Path) {
		let mut logged_paths = self.logged_paths.lock().unwrap();
		logged_paths.swap_remove(path);
	}
}

impl Clone for MissingAssetLogger {
	fn clone(&self) -> Self {
		Self {
			logged_paths: Mutex::new(self.logged_paths.lock().unwrap().clone()),
		}
	}
}
