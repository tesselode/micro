pub mod loader;

use std::{
	fmt::Debug,
	ops::Index,
	path::{Path, PathBuf},
	time::{Duration, SystemTime},
};

use indexmap::{IndexMap, IndexSet};
use thiserror::Error;

use crate::Context;

use self::loader::ResourceLoader;

const HOT_RELOAD_INTERVAL: Duration = Duration::from_secs(1);

pub struct Resources<L: ResourceLoader> {
	base_dir: PathBuf,
	loader: L,
	resources: IndexMap<PathBuf, Resource<L>>,
	placeholder: Option<L::Resource>,
	hot_reload_timer: Duration,
}

impl<L: ResourceLoader> Resources<L> {
	pub fn new(ctx: &mut Context, base_dir: impl AsRef<Path>, mut loader: L) -> Self {
		let placeholder = loader.placeholder(ctx);
		Self {
			base_dir: Self::base_resources_path().join(base_dir.as_ref()),
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
			let Some(file_path) = L::SUPPORTED_FILE_EXTENSIONS
				.iter()
				.map(|extension| full_resource_path.with_extension(extension))
				.find(|path| path.exists())
			else {
				return;
			};
			let settings = match Self::load_settings(&full_resource_path) {
				Ok(settings) => settings,
				Err(err) => {
					tracing::error!(
						"Error loading settings at path {}: {}",
						full_resource_path.with_extension("settings").display(),
						err
					);
					None
				}
			};
			let resource = match self.loader.load(ctx, &file_path, settings.as_ref()) {
				Ok(resource) => resource,
				Err(err) => {
					tracing::error!(
						"Error loading resource at path {}: {}",
						file_path.display(),
						err
					);
					return;
				}
			};
			let modified = match Self::modified_time(&file_path) {
				Ok(modified) => Some(modified),
				Err(err) => {
					tracing::error!(
						"Error getting modified time of resource at path {}: {}",
						file_path.display(),
						err
					);
					None
				}
			};
			self.resources.insert(
				path.into(),
				Resource {
					file_path,
					modified,
					resource,
					settings,
				},
			);
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

	fn load_settings(resource_path: &Path) -> Result<Option<L::Settings>, LoadSettingsError> {
		let settings_path = resource_path.with_extension("settings");
		if !settings_path.exists() {
			return Ok(None);
		}
		let settings_string = std::fs::read_to_string(&settings_path)?;
		Ok(serde_json::from_str(&settings_string)?)
	}

	fn modified_time(path: &Path) -> std::io::Result<SystemTime> {
		std::fs::metadata(path)?.modified()
	}

	fn hot_reload(&mut self, ctx: &mut Context) {
		for Resource {
			file_path,
			modified,
			resource,
			settings,
		} in self.resources.values_mut()
		{
			let current_modified_time = match Self::modified_time(file_path) {
				Ok(current_modified_time) => current_modified_time,
				Err(err) => {
					tracing::error!(
						"Error getting modified time of resource at path '{}': {}",
						file_path.display(),
						err
					);
					continue;
				}
			};
			if modified.is_some_and(|modified| modified == current_modified_time) {
				continue;
			}
			tracing::info!("hot reloading resource at path '{}'", file_path.display());
			if let Err(err) = self
				.loader
				.reload(ctx, resource, file_path, settings.as_ref())
			{
				tracing::error!(
					"Error loading resource at path {}: {}",
					file_path.display(),
					err
				);
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

#[derive(Debug, Error)]
enum LoadSettingsError {
	#[error("{0}")]
	IoError(#[from] std::io::Error),
	#[error("{0}")]
	LoadSettingsError(#[from] serde_json::Error),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Resource<L: ResourceLoader> {
	file_path: PathBuf,
	modified: Option<SystemTime>,
	resource: L::Resource,
	settings: Option<L::Settings>,
}
