mod loader;
mod resource_with_metadata;

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

use self::resource_with_metadata::ResourceWithMetadata;

const HOT_RELOAD_INTERVAL: Duration = Duration::from_secs(1);

pub struct Resources<L: ResourceLoader> {
	base_dir: PathBuf,
	loader: L,
	resources: IndexMap<PathBuf, ResourceWithMetadata<L>>,
	placeholder: Option<L::Resource>,
	hot_reload_timer: Duration,
	missing_resource_logger: MissingResourceLogger,
}

impl<L: ResourceLoader> Resources<L> {
	pub fn new(ctx: &mut L::Context, base_dir: impl AsRef<Path>, mut loader: L) -> Self {
		let placeholder = loader.placeholder(ctx);
		Self {
			base_dir: base_resources_path().join(base_dir.as_ref()),
			loader,
			resources: IndexMap::new(),
			placeholder,
			hot_reload_timer: Duration::ZERO,
			missing_resource_logger: MissingResourceLogger::new(),
		}
	}

	pub fn autoloaded(ctx: &mut L::Context, base_dir: impl AsRef<Path>, loader: L) -> Self {
		let mut resources = Self::new(ctx, base_dir, loader);
		resources.load_all(ctx);
		resources
	}

	pub fn load(&mut self, ctx: &mut L::Context, path: impl AsRef<Path>) {
		self.load_inner(ctx, path.as_ref())
	}

	pub fn load_all(&mut self, ctx: &mut L::Context) {
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
		let path = path.as_ref();
		if let Some(resource) = self.resources.get(path) {
			Some(&resource.resource)
		} else {
			if self.loader.warn_on_missing() {
				self.missing_resource_logger.log(path);
			}
			if let Some(placeholder) = &self.placeholder {
				Some(placeholder)
			} else {
				None
			}
		}
	}

	pub fn get_mut(&mut self, path: impl AsRef<Path>) -> Option<&mut L::Resource> {
		let path = path.as_ref();
		if let Some(resource) = self.resources.get_mut(path) {
			Some(&mut resource.resource)
		} else {
			if self.loader.warn_on_missing() {
				self.missing_resource_logger.log(path);
			}
			if let Some(placeholder) = &mut self.placeholder {
				Some(placeholder)
			} else {
				None
			}
		}
	}

	pub fn iter(&self) -> impl Iterator<Item = (&Path, &L::Resource)> {
		self.resources
			.iter()
			.map(|(path, resource)| (path.as_ref(), &resource.resource))
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
							"Error loading resource at path {}: {:?}",
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

	fn hot_reload(&mut self, ctx: &mut L::Context) {
		for (path, resource) in &mut self.resources {
			let reloaded = resource.reload(ctx, &mut self.loader);
			if reloaded {
				self.missing_resource_logger.on_reloaded(path);
			}
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
			missing_resource_logger: self.missing_resource_logger.clone(),
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

impl<T: AsRef<Path>, L: ResourceLoader> IndexMut<T> for Resources<L> {
	fn index_mut(&mut self, path: T) -> &mut Self::Output {
		self.get_mut(path).unwrap()
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

struct MissingResourceLogger {
	logged_paths: Mutex<IndexSet<PathBuf>>,
}

impl MissingResourceLogger {
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
		warn!("Missing resource '{}'", path.display());
		logged_paths.insert(path.to_path_buf());
	}

	fn on_reloaded(&self, path: &Path) {
		let mut logged_paths = self.logged_paths.lock().unwrap();
		logged_paths.swap_remove(path);
	}
}

impl Clone for MissingResourceLogger {
	fn clone(&self) -> Self {
		Self {
			logged_paths: Mutex::new(self.logged_paths.lock().unwrap().clone()),
		}
	}
}
