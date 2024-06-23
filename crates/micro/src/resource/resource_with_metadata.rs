use std::{
	path::{Path, PathBuf},
	time::SystemTime,
};

use thiserror::Error;

use crate::Context;

use super::loader::ResourceLoader;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(super) struct ResourceWithMetadata<L: ResourceLoader> {
	pub file_path: PathBuf,
	pub modified_time: Option<SystemTime>,
	pub settings_modified_time: Option<SystemTime>,
	pub resource: L::Resource,
	pub settings: Option<L::Settings>,
}

impl<L: ResourceLoader> ResourceWithMetadata<L> {
	pub fn load(
		ctx: &mut Context,
		full_resource_path: &Path,
		loader: &mut L,
	) -> Result<Option<Self>, L::Error> {
		let Some(file_path) = L::SUPPORTED_FILE_EXTENSIONS
			.iter()
			.map(|extension| full_resource_path.with_extension(extension))
			.find(|path| path.exists())
		else {
			return Ok(None);
		};
		let settings_path = full_resource_path.with_extension("settings");
		let settings = match Self::load_settings(&settings_path) {
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
		let resource = loader.load(ctx, &file_path, settings.as_ref())?;
		let modified_time = match file_modified_time(&file_path) {
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
		let settings_modified_time = if settings_path.exists() {
			match file_modified_time(&settings_path) {
				Ok(modified) => Some(modified),
				Err(err) => {
					tracing::error!(
						"Error getting modified time of settings at path {}: {}",
						settings_path.display(),
						err
					);
					None
				}
			}
		} else {
			None
		};
		Ok(Some(Self {
			file_path,
			modified_time,
			settings_modified_time,
			resource,
			settings,
		}))
	}

	pub fn reload(&mut self, ctx: &mut Context, loader: &mut L) -> Reloaded {
		if !self.check_for_updates() {
			return false;
		}
		tracing::info!(
			"hot reloading resource at path '{}'",
			self.file_path.display()
		);
		let settings_path = self.file_path.with_extension("settings");
		match Self::load_settings(&settings_path) {
			Ok(settings) => self.settings = settings,
			Err(err) => {
				tracing::error!(
					"Error loading settings at path {}: {}",
					settings_path.display(),
					err
				)
			}
		}
		if let Err(err) = loader.reload(
			ctx,
			&mut self.resource,
			&self.file_path,
			self.settings.as_ref(),
		) {
			tracing::error!(
				"Error loading resource at path {}: {:?}",
				self.file_path.display(),
				err
			);
			return false;
		}
		true
	}

	pub fn check_for_updates(&mut self) -> bool {
		let current_modified_time = match file_modified_time(&self.file_path) {
			Ok(current_modified_time) => Some(current_modified_time),
			Err(err) => {
				tracing::error!(
					"Error getting modified time of resource at path '{}': {}",
					self.file_path.display(),
					err
				);
				None
			}
		};
		let settings_path = self.file_path.with_extension("settings");
		let current_settings_modified_time = if settings_path.exists() {
			match file_modified_time(&settings_path) {
				Ok(current_settings_modified_time) => Some(current_settings_modified_time),
				Err(err) => {
					tracing::error!(
						"Error getting modified time of settings at path '{}': {}",
						self.file_path.display(),
						err
					);
					None
				}
			}
		} else {
			None
		};
		let changed = current_modified_time != self.modified_time
			|| current_settings_modified_time != self.settings_modified_time;
		if changed {
			self.modified_time = current_modified_time;
			self.settings_modified_time = current_settings_modified_time;
		}
		changed
	}

	fn load_settings(settings_path: &Path) -> Result<Option<L::Settings>, LoadSettingsError> {
		if !settings_path.exists() {
			return Ok(None);
		}
		let settings_string = std::fs::read_to_string(settings_path)?;
		Ok(serde_json::from_str(&settings_string)?)
	}
}

fn file_modified_time(path: &Path) -> std::io::Result<SystemTime> {
	std::fs::metadata(path)?.modified()
}

#[derive(Debug, Error)]
enum LoadSettingsError {
	#[error("{0}")]
	IoError(#[from] std::io::Error),
	#[error("{0}")]
	LoadSettingsError(#[from] serde_json::Error),
}

type Reloaded = bool;
