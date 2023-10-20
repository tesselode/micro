pub mod loader;

use std::{
	ops::Index,
	path::{Path, PathBuf},
};

use indexmap::{map::Iter, IndexMap, IndexSet};
use thiserror::Error;

use crate::Context;

use self::loader::ResourceLoader;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Resources<L: ResourceLoader> {
	base_dir: PathBuf,
	resources: IndexMap<PathBuf, L::Resource>,
	loader: L,
}

impl<L: ResourceLoader> Resources<L> {
	pub fn new(base_dir: impl AsRef<Path>, loader: L) -> Self {
		Self {
			base_dir: Self::base_resources_path().join(base_dir.as_ref()),
			resources: IndexMap::new(),
			loader,
		}
	}

	pub fn autoloaded(ctx: &mut Context, base_dir: impl AsRef<Path>, loader: L) -> Self {
		let mut resources = Self::new(base_dir, loader);
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
		self.resources.get(path.as_ref())
	}

	pub fn iter(&self) -> Iter<'_, PathBuf, L::Resource> {
		self.resources.iter()
	}

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
			let resource = match self.loader.load(ctx, &file_path, settings) {
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
