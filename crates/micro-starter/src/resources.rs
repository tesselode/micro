use std::path::PathBuf;

use micro::Context;

pub struct Resources {}

impl Resources {
	pub fn base_dir() -> PathBuf {
		"resources".into()
	}

	pub fn new(ctx: &mut Context) -> anyhow::Result<Self> {
		Ok(Self {})
	}
}
