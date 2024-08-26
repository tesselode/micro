use std::{
	any::Any,
	path::{Path, PathBuf},
};

use indexmap::IndexMap;

#[derive(Debug)]
pub struct UiState(IndexMap<PathBuf, StateWrapper>);

impl UiState {
	pub fn new() -> Self {
		Self(IndexMap::new())
	}

	pub fn get_mut<T: Default + 'static>(&mut self, path: impl AsRef<Path>) -> &mut T {
		self.get_mut_or_insert_with(path, T::default)
	}

	pub fn get_mut_or_insert_with<T: 'static>(
		&mut self,
		path: impl AsRef<Path>,
		default: impl Fn() -> T,
	) -> &mut T {
		let state_wrapper = self
			.0
			.entry(path.as_ref().to_path_buf())
			.or_insert_with(|| StateWrapper {
				state: Box::new(default()),
				used: false,
			});
		state_wrapper.used = true;
		state_wrapper.state.downcast_mut().unwrap()
	}

	pub fn reset_used(&mut self) {
		for StateWrapper { used, .. } in self.0.values_mut() {
			*used = false;
		}
	}

	pub fn remove_unused(&mut self) {
		self.0.retain(|_, StateWrapper { used, .. }| *used)
	}
}

impl Default for UiState {
	fn default() -> Self {
		Self::new()
	}
}

#[derive(Debug)]
struct StateWrapper {
	state: Box<dyn Any>,
	used: bool,
}
