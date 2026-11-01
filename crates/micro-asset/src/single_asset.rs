use std::{
	fmt::Debug,
	hash::Hash,
	ops::{Deref, DerefMut},
	path::Path,
	time::Duration,
};

use crate::{AssetLoader, asset_with_metadata::AssetWithMetadata, base_assets_path};

pub struct SingleAsset<L: AssetLoader> {
	loader: L,
	asset: AssetWithMetadata<L>,
	hot_reload_timer: Duration,
}

impl<L: AssetLoader> SingleAsset<L> {
	pub fn new(
		ctx: &mut L::Context,
		path: impl AsRef<Path>,
		mut loader: L,
	) -> Result<Self, L::Error> {
		let path = path.as_ref();
		let Some(asset) =
			AssetWithMetadata::load(ctx, &base_assets_path().join(path), &mut loader)?
		else {
			let extension = path.extension().expect("Asset path has no extension");
			panic!(
				"Invalid file extension for asset: got {}, expected {}",
				extension.display(),
				L::SUPPORTED_FILE_EXTENSIONS.join(", ")
			)
		};
		Ok(Self {
			loader,
			asset,
			hot_reload_timer: Duration::ZERO,
		})
	}

	pub fn get(&self) -> &L::Asset {
		&self.asset.asset
	}

	pub fn get_mut(&mut self) -> &mut L::Asset {
		&mut self.asset.asset
	}

	#[cfg(debug_assertions)]
	pub fn update_hot_reload(&mut self, ctx: &mut L::Context, delta_time: Duration) {
		self.hot_reload_timer += delta_time;
		if self.hot_reload_timer >= crate::HOT_RELOAD_INTERVAL {
			self.hot_reload(ctx);
			self.hot_reload_timer = Duration::ZERO;
		}
	}

	#[cfg(not(debug_assertions))]
	pub fn update_hot_reload(&mut self, _ctx: &mut L::Context, _delta_time: Duration) {}

	fn hot_reload(&mut self, ctx: &mut L::Context) {
		self.asset.reload(ctx, &mut self.loader);
	}
}

impl<L> Debug for SingleAsset<L>
where
	L: AssetLoader + Debug,
	L::Asset: Debug,
	L::Settings: Debug,
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("SingleAsset")
			.field("loader", &self.loader)
			.field("asset", &self.asset)
			.field("hot_reload_timer", &self.hot_reload_timer)
			.finish()
	}
}

impl<L> Clone for SingleAsset<L>
where
	L: AssetLoader + Clone,
	L::Asset: Clone,
	L::Settings: Clone,
{
	fn clone(&self) -> Self {
		Self {
			loader: self.loader.clone(),
			asset: self.asset.clone(),
			hot_reload_timer: self.hot_reload_timer,
		}
	}
}

impl<L> PartialEq for SingleAsset<L>
where
	L: AssetLoader + PartialEq,
	L::Asset: PartialEq,
	L::Settings: PartialEq,
{
	fn eq(&self, other: &Self) -> bool {
		self.loader == other.loader
			&& self.asset == other.asset
			&& self.hot_reload_timer == other.hot_reload_timer
	}
}

impl<L> Eq for SingleAsset<L>
where
	L: AssetLoader + Eq,
	L::Asset: Eq,
	L::Settings: Eq,
{
}

impl<L> Hash for SingleAsset<L>
where
	L: AssetLoader + Hash,
	L::Asset: Hash,
	L::Settings: Hash,
{
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		self.loader.hash(state);
		self.asset.hash(state);
		self.hot_reload_timer.hash(state);
	}
}

impl<L: AssetLoader> Deref for SingleAsset<L> {
	type Target = L::Asset;

	fn deref(&self) -> &Self::Target {
		self.get()
	}
}

impl<L: AssetLoader> DerefMut for SingleAsset<L> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.get_mut()
	}
}
