pub mod loader;

use std::path::{Path, PathBuf};

use indexmap::{map::Iter, IndexMap, IndexSet};

use crate::Context;

use self::loader::ResourceLoader;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Resources<L: ResourceLoader> {
	resources: IndexMap<PathBuf, L::Resource>,
	loader: L,
}

impl<L: ResourceLoader> Resources<L> {
	pub fn new(loader: L) -> Self {
		Self {
			resources: IndexMap::new(),
			loader,
		}
	}

	pub fn load(&mut self, ctx: &mut Context, path: impl AsRef<Path>) -> Result<(), L::Error> {
		self.load_inner(ctx, path.as_ref())
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

	fn load_inner(&mut self, ctx: &mut Context, path: &Path) -> Result<(), L::Error> {
		let base_resources_path = Self::base_resources_path();
		let full_path = base_resources_path.join(path);
		if full_path.is_dir() {
			let mut resource_paths = IndexSet::new();
			for entry in std::fs::read_dir(&full_path)? {
				let entry = entry?;
				resource_paths.insert(
					entry
						.path()
						.strip_prefix(&base_resources_path)
						.unwrap()
						.with_extension(""),
				);
			}
			for resource_path in resource_paths {
				self.load_inner(ctx, &resource_path)?;
			}
		} else {
			for extension in L::SUPPORTED_FILE_EXTENSIONS {
				let file_path = full_path.with_extension(extension);
				if file_path.exists() {
					let resource = self.loader.load(ctx, &file_path)?;
					self.resources.insert(path.into(), resource);
					return Ok(());
				}
			}
		}
		Ok(())
	}

	fn base_resources_path() -> PathBuf {
		"resources".into()
	}
}
