pub mod loader;
mod resource_with_metadata;

use std::{
	fmt::Debug,
	ops::Index,
	path::{Path, PathBuf},
	time::Duration,
};

use indexmap::{IndexMap, IndexSet};

use crate::Context;

use self::{loader::ResourceLoader, resource_with_metadata::ResourceWithMetadata};

const HOT_RELOAD_INTERVAL: Duration = Duration::from_secs(1);

pub struct Resources<L: ResourceLoader> {
	base_dir: PathBuf,
	loader: L,
	resources: IndexMap<PathBuf, ResourceWithMetadata<L>>,
	placeholder: Option<L::Resource>,
	hot_reload_timer: Duration,
}

impl<L: ResourceLoader> Resources<L> {
	pub fn new(ctx: &mut Context, base_dir: impl AsRef<Path>, mut loader: L) -> Self {
		let placeholder = loader.placeholder(ctx);
		Self {
			base_dir: base_resources_path().join(base_dir.as_ref()),
			loader,
			resources: IndexMap::new(),
			placeholder,
			hot_reload_timer: Duration::ZERO,
		}
	}

	pub fn autoloaded(ctx: &mut Context, base_dir: impl AsRef<Path>, loader: L) -> Self {
		let mut resources = Self::new(ctx, base_dir, loader);
		resources.load_all(ctx);
		resources
	}

	pub fn load(&mut self, ctx: &mut Context, path: impl AsRef<Path>) {
		self.load_inner(ctx, path.as_ref())
	}

	pub fn load_all(&mut self, ctx: &mut Context) {
		self.load(ctx, "")
	}

	pub fn unload(&mut self, dir: impl AsRef<Path>) {
		let dir = dir.as_ref();
		self.resources.retain(|path, _| path.starts_with(dir));
	}

	pub fn append(&mut self, mut other: Self) {
		self.resources.extend(other.resources.drain(..));
	}

	pub fn get(&self, path: impl AsRef<Path>) -> Option<&L::Resource> {
		self.resources
			.get(path.as_ref())
			.map(|resource| &resource.resource)
			.or(self.placeholder.as_ref())
	}

	pub fn iter(&self) -> impl Iterator<Item = (&Path, &L::Resource)> {
		self.resources
			.iter()
			.map(|(path, resource)| (path.as_ref(), &resource.resource))
	}

	#[cfg(debug_assertions)]
	pub fn update_hot_reload(&mut self, ctx: &mut Context, delta_time: Duration) {
		self.hot_reload_timer += delta_time;
		if self.hot_reload_timer >= HOT_RELOAD_INTERVAL {
			self.hot_reload(ctx);
			self.hot_reload_timer = Duration::ZERO;
		}
	}

	#[cfg(not(debug_assertions))]
	pub fn update_hot_reload(&mut self, _ctx: &mut Context, _delta_time: Duration) {}

	fn load_inner(&mut self, ctx: &mut Context, path: &Path) {
		let full_resource_path = self.base_dir.join(path);
		if full_resource_path.is_dir() {
			let resource_paths = match self.resources_in_dir(&full_resource_path) {
				Ok(paths) => paths,
				Err(err) => {
					tracing::error!(
						"Error getting resources in {}: {}",
						full_resource_path.display(),
						err
					);
					return;
				}
			};
			for resource_path in resource_paths {
				self.load_inner(ctx, &resource_path);
			}
		} else {
			let resource =
				match ResourceWithMetadata::load(ctx, &full_resource_path, &mut self.loader) {
					Ok(Some(resource)) => resource,
					Ok(None) => return,
					Err(err) => {
						tracing::error!(
							"Error loading resource at path {}: {}",
							full_resource_path.display(),
							err
						);
						return;
					}
				};
			self.resources.insert(path.into(), resource);
		}
	}

	fn resources_in_dir(
		&mut self,
		full_path: &PathBuf,
	) -> Result<IndexSet<PathBuf>, std::io::Error> {
		let mut resource_paths = IndexSet::new();
		for entry in std::fs::read_dir(full_path)? {
			let entry = entry?;
			resource_paths.insert(
				entry
					.path()
					.strip_prefix(&self.base_dir)
					.unwrap()
					.with_extension(""),
			);
		}
		Ok(resource_paths)
	}

	fn hot_reload(&mut self, ctx: &mut Context) {
		for resource in self.resources.values_mut() {
			resource.reload(ctx, &mut self.loader);
		}
	}
}

impl<L> Debug for Resources<L>
where
	L: ResourceLoader + Debug,
	L::Resource: Debug,
	L::Settings: Debug,
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("Resources")
			.field("base_dir", &self.base_dir)
			.field("loader", &self.loader)
			.field("resources", &self.resources)
			.field("placeholder", &self.placeholder)
			.field("hot_reload_timer", &self.hot_reload_timer)
			.finish()
	}
}

impl<L> Clone for Resources<L>
where
	L: ResourceLoader + Clone,
	L::Resource: Clone,
	L::Settings: Clone,
{
	fn clone(&self) -> Self {
		Self {
			base_dir: self.base_dir.clone(),
			loader: self.loader.clone(),
			resources: self.resources.clone(),
			placeholder: self.placeholder.clone(),
			hot_reload_timer: self.hot_reload_timer,
		}
	}
}

impl<L> PartialEq for Resources<L>
where
	L: ResourceLoader + PartialEq,
	L::Resource: PartialEq,
	L::Settings: PartialEq,
{
	fn eq(&self, other: &Self) -> bool {
		self.base_dir == other.base_dir
			&& self.loader == other.loader
			&& self.resources == other.resources
			&& self.placeholder == other.placeholder
			&& self.hot_reload_timer == other.hot_reload_timer
	}
}

impl<L> Eq for Resources<L>
where
	L: ResourceLoader + Eq,
	L::Resource: Eq,
	L::Settings: Eq,
{
}

impl<T: AsRef<Path>, L: ResourceLoader> Index<T> for Resources<L> {
	type Output = L::Resource;

	fn index(&self, path: T) -> &Self::Output {
		self.get(path).unwrap()
	}
}

pub fn base_resources_path() -> PathBuf {
	#[cfg(debug_assertions)]
	{
		"resources".into()
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
